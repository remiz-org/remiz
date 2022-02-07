use std::collections::{HashMap, HashSet};
use std::fs::{self, File};
use std::io::Write;
use std::path::PathBuf;
use std::process::Command;

use uuid::Uuid;

use walkdir::WalkDir;

use mla::config::ArchiveWriterConfig;
use mla::{ArchiveWriter, Layers};

use crate::errors::RemizError;
use crate::helpers;
use crate::subpackage::Subpackage;
use crate::{
    global_configuration::GlobalConfig, metadata::Metadata, package_configuration::PackageConfig,
};

// File describing the .pack file format.

#[derive(Debug)]
pub struct Package {
    pub metadata: Metadata,
    pub subpackages: Vec<Subpackage>,
}

impl Package {
    pub fn from_config(
        global_conf: &GlobalConfig,
        package_config: &PackageConfig,
    ) -> Result<Package, RemizError> {
        trace!("Building package...");
        let mut subpackages = Vec::new();
        // For each subpackage in package config, call the packager
        // defined in the global configuration file.
        for sub in package_config.subpackages.to_owned() {
            let mut files = HashMap::new();
            let path_to_subpackager = match global_conf.packagers.get(&sub.name) {
                Some(path) => {
                    // If subpackager path is relative, make it relative to the
                    // global config file.
                    if path.is_relative() {
                        global_conf.path.parent().unwrap().join(path)
                    }
                    // Otherwise, use the path as-is.
                    else {
                        path.to_owned()
                    }
                }
                None => {
                    let subpackagers = global_conf
                        .packagers
                        .keys()
                        .map(|k| k.to_string())
                        .collect::<Vec<String>>();
                    error!(
                        "No subpackager found for {}. Available subpackagers: {}",
                        sub.name,
                        subpackagers.join(", ")
                    );
                    return Err(RemizError::SubpackagerNotFound);
                }
            };

            // Make sure the path is absolute and check if it exists.
            let path_to_subpackager = match path_to_subpackager.canonicalize() {
                Ok(path) => path,
                Err(e) => {
                    error!("{}", e);
                    return Err(RemizError::SubpackagerNotFound);
                }
            };

            let uuid = Uuid::new_v4();

            let folder = format!("{}_{}", &sub.name, uuid.to_simple());
            let subpackage_path = path_to_subpackager.parent().unwrap().join(folder);

            // Create the folder
            debug!("Creating {} folder...", &subpackage_path.display());
            fs::create_dir(&subpackage_path)?;

            let mut args = Vec::new();
            args.push("build".to_string());
            args.push(format!("--path"));
            args.push(subpackage_path.display().to_string());

            // Add subpackager defined arguments
            let params = &sub.value.as_table();
            for opt in params {
                for (param_name, param_value) in opt.into_iter() {
                    let arg_name = format!("--{}", param_name);

                    if param_value.is_array() {
                        for val in param_value.as_array().unwrap() {
                            let arg = val.as_str().unwrap().to_owned();
                            args.push(arg_name.to_string());
                            args.push(arg);
                        }
                    } else if param_value.is_str() {
                        args.push(arg_name);
                        args.push(param_value.as_str().unwrap().to_owned());
                    }
                }
            }

            let mut current_dir = package_config.path.clone().canonicalize().unwrap();
            current_dir.pop(); // remove filename
            trace!("Calling packager {}...", &path_to_subpackager.display());

            // Call packager
            let output = Command::new(&path_to_subpackager)
                .current_dir(&current_dir)
                .env("REMIZ", "1")
                .args(args)
                .output();

            match output {
                Ok(output) => {
                    let status_code = output.status.code().unwrap();
                    trace!("Packager {} exits with code {}", &sub.name, status_code);

                    // Print the stdout of the subpackager
                    let stdout = String::from_utf8_lossy(&output.stdout);
                    let stderr = String::from_utf8_lossy(&output.stderr);

                    if !stdout.is_empty() {
                        info!("   --- {} (stdout) ---\n{}", &sub.name, stdout);
                    }
                    if !stderr.is_empty() {
                        error!("   --- {} (stderr) ---\n{}", &sub.name, stderr);
                    }

                    if status_code != 0 {
                        error!("Subpackager {} failed.", &sub.name);
                        fs::remove_dir_all(subpackage_path)?;
                        return Err(RemizError::SubpackagerFailed);
                    }

                    let mut number_files_in_subpackage = 0;
                    for e in WalkDir::new(&subpackage_path)
                        .into_iter()
                        .filter_map(|e| e.ok())
                    {
                        number_files_in_subpackage += 1;
                        if e.metadata().unwrap().is_file() {
                            let data = match fs::read(&e.path()) {
                                Ok(data) => data,
                                Err(err) => {
                                    error!(
                                        "Error reading subpackage file: {} ({})",
                                        &e.path().display().to_string(),
                                        err
                                    );
                                    fs::remove_dir_all(subpackage_path)?;
                                    return Err(RemizError::SubpackageNotCreated);
                                }
                            };
                            let relative_file_path =
                                e.path().strip_prefix(&subpackage_path).unwrap();
                            files.insert(
                                format!(
                                    "{}/{}",
                                    &sub.name,
                                    relative_file_path.display().to_string()
                                ),
                                data,
                            );
                        }
                    }

                    if number_files_in_subpackage == 0 {
                        warn!("Subpackager {} did not create any files.", &sub.name);
                    } else {
                        info!(
                            "Subpackager {} created {} files.",
                            &sub.name,
                            number_files_in_subpackage
                        );
                    }

                    fs::remove_dir_all(subpackage_path)?;

                    subpackages.push(Subpackage {
                        name: sub.name.to_owned(),
                        files: files.to_owned(),
                    });
                }
                Err(e) => {
                    fs::remove_dir_all(subpackage_path)?;
                    // if error code is 13 (permission denied)
                    if e.raw_os_error() == Some(13) {
                        error!(
                            "Permission denied while executing {} ({}). chmod +x ?",
                            sub.name,
                            path_to_subpackager.display()
                        );
                        return Err(RemizError::PermissionDenied);
                    } else {
                        error!("{}", e);
                        return Err(RemizError::IOError(e));
                    }
                }
            }
        }
        Ok(Package {
            metadata: package_config.metadata.to_owned(),
            subpackages,
        })
    }

    pub fn from_file(path: &PathBuf) -> Result<Package, RemizError> {
        let mut mla_read = helpers::get_archive_reader(&path)?;

        let mut metadata_file = mla_read
            .get_file("remiz-metadata".to_string())
            .unwrap()
            .unwrap();

        let mut metadata_data = Vec::new();
        {
            std::io::copy(&mut metadata_file.data, &mut metadata_data).unwrap();
        }
        let metadata: Metadata = toml::from_slice(&metadata_data).unwrap();
        let mut subpackages = Vec::new();

        let file_names = helpers::get_filenames(&path);

        let subpackages_in_file: Vec<String> = (&file_names)
            .into_iter()
            .filter(|filename| filename.contains("/"))
            .map(|filename| filename.split("/").next().unwrap().to_string())
            .collect::<HashSet<_>>()
            .into_iter()
            .collect();

        for name in subpackages_in_file {
            let mut files = HashMap::new();

            for filename in &file_names {
                if filename.split("/").next().unwrap().to_string() == name {
                    let archive_file = mla_read.get_file(filename.to_string());
                    let mut data = Vec::new();
                    std::io::copy(&mut archive_file.unwrap().unwrap().data, &mut data).unwrap();
                    files.insert(filename.to_string(), data);
                }
            }

            subpackages.push(Subpackage { name, files })
        }

        Ok(Package {
            metadata,
            subpackages,
        })
    }
    pub fn write(&self, path: &PathBuf) -> Result<(), RemizError> {
        let mut buf = Vec::new();
        let mut config = ArchiveWriterConfig::default();

        // TODO: support encryption
        config.disable_layer(Layers::ENCRYPT);

        let mut archive = ArchiveWriter::from_config(&mut buf, config).unwrap();

        // Add metadata to the archive
        let metadata_buffer = toml::to_string(&self.metadata).unwrap();

        let metadata_buffer = metadata_buffer.as_bytes();
        archive
            .add_file(
                "remiz-metadata", //TODO collision risk with a package named "remiz-metadata"?
                metadata_buffer.len() as u64,
                metadata_buffer,
            )
            .unwrap();

        // Add subpackages to the archive
        for subpackage in &self.subpackages {
            // For each files in the subpackage
            for (filename, data) in &subpackage.files {
                archive
                    .add_file(filename, data.len() as u64, &data[..])
                    .unwrap();
            }
        }

        archive.finalize().unwrap();

        debug!("Package created. Writing content to {:?}...", path);

        // Create all missing directories in path
        fs::create_dir_all(path.parent().unwrap())?;

        // Create the file
        let mut f = File::create(path)?; // TODO improve error message
        f.write_all(archive.into_raw())?;

        Ok(())
    }
}
