use std::{fs::File, io::Read, path::PathBuf};

use semver::Version;
use toml::Value;

use crate::{errors::RemizError, metadata::Metadata};

// Represents a package configuration file (TOML)
#[derive(Debug, Clone)]
pub struct PackageConfig {
    pub path: PathBuf,
    pub metadata: Metadata,
    pub subpackages: Vec<SubPackageConfig>,
}
#[derive(Debug, Clone)]
pub struct SubPackageConfig {
    pub name: String,
    pub value: Value,
}

impl PackageConfig {
    pub fn from_file(path: PathBuf) -> Result<PackageConfig, RemizError> {
        let mut file = match File::open(&path) {
            Ok(file) => file,
            Err(err) => {
                error!("Could not open configuration file: {:?}", path);
                return Err(RemizError::FileNotFound(err.to_string()));
            }
        };

        let mut toml = String::new();
        file.read_to_string(&mut toml).unwrap();

        let parsed_toml = match toml.parse::<Value>() {
            Ok(value) => value,
            Err(err) => {
                error!("Badly formatted TOML: {:?} ({})", path, err);
                return Err(RemizError::BadTOMLFormat);
            }
        };
        let name = parsed_toml["info"]["name"].as_str().unwrap().to_string();
        let version = Version::parse(parsed_toml["info"]["version"].as_str().unwrap()).expect(
            "Could not parse package version. Please use semver as defined by https://semver.org/.",
        );
        // TODO: parse other fields and add them
        let mut other = Vec::new();
        for (other_metadata_name, other_metadata_value) in parsed_toml["info"].as_table().unwrap() {
            if other_metadata_name != "name" && other_metadata_name != "version" {
                other.push((
                    other_metadata_name.to_string(),
                    other_metadata_value.to_owned(),
                ));
            }
        }

        let created_by = whoami::username();

        const REMIZ_VERSION: &str = env!("CARGO_PKG_VERSION");

        let metadata = Metadata {
            name,
            version,
            toml,
            other,
            created_by,
            remiz_version: Version::parse(REMIZ_VERSION).expect("Could not parse REMIZ_VERSION"),
        };

        let mut subpackages = Vec::new();
        for (name, value) in parsed_toml.as_table().unwrap() {
            if name != "info" {
                subpackages.push(SubPackageConfig {
                    name: name.to_string(),
                    value: value.to_owned(),
                })
            }
        }

        Ok(PackageConfig {
            path,
            metadata,
            subpackages,
        })
    }
}
