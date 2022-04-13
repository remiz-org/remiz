use std::{
    fs::{self},
    path::PathBuf,
    process::Command,
};

use convert_case::{Case, Casing};

use crate::{errors::RemizError, global_configuration::GlobalConfig, package::Package};

/// Try to deploy a package given a package file
pub fn deploy(
    path_to_package_file: PathBuf,
    env: String,
    global_config_file: Option<PathBuf>,
    extra_args: Vec<String>,
) -> Result<(), RemizError> {
    let global_conf = GlobalConfig::load(global_config_file)?;

    let package = Package::from_file(&path_to_package_file)?;

    // For each of the subpackages, mount the 'magic' folder and call the deployer
    // with the path of this folder
    for subpackage in &package.subpackages {
        let subpackage_name = &subpackage.name;
        trace!("Getting parent folder of global_conf path...");
        let base_path = global_conf
            .path
            .parent()
            .unwrap()
            .join(&package.metadata.name.to_case(Case::Snake));
        let folder_path = &subpackage.uncompress(&base_path)?;

        // Call the subremiz
        let path_to_subpackager = global_conf.packagers.get(subpackage_name).unwrap();

        let mut args = Vec::new();
        args.push("deploy".to_string());
        args.push(format!("--path"));
        trace!("Compute absolute folder path...");
        let abs_folder_path = folder_path
            .canonicalize()
            .unwrap()
            .display()
            .to_string();

        args.push((&abs_folder_path).to_string());
        args.push(format!("--env"));
        args.push(env.clone());

        // Add extra args (if present)
        args.extend(extra_args.clone());

        trace!("Building command...");
        let output = Command::new(&path_to_subpackager)
            .current_dir(&global_conf.path.parent().unwrap())
            .env("REMIZ", "1")
            .args(args)
            .output();

        // Remove folder
        fs::remove_dir_all(abs_folder_path.to_string())?;

        match output {
            Ok(output) => {
                // Print the stdout of the subpackager
                let stdout = String::from_utf8_lossy(&output.stdout);
                let stderr = String::from_utf8_lossy(&output.stderr);

                if !stdout.is_empty() {
                    info!("   --- {} (stdout) ---\n{}", &subpackage_name, stdout);
                }
                if !stderr.is_empty() {
                    error!("   --- {} (stderr) ---\n{}", &subpackage_name, stderr);
                }

                if output.status.code().unwrap() != 0 {
                    error!("Subpackager {} failed.", &subpackage_name);
                    return Err(RemizError::SubpackagerFailed);
                }
            }
            Err(e) => {
                // if error code is 13 (permission denied)
                if e.raw_os_error() == Some(13) {
                    error!(
                        "Permission denied while executing {} ({}). chmod +x ?",
                        &subpackage_name,
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

    Ok(())
}
