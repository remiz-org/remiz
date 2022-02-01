use std::{collections::HashMap, fs, path::PathBuf};

use toml::Value;

use crate::errors::RemizError;

#[derive(Debug, Clone)]
pub struct Store {
    pub name: String,
    pub uri: PathBuf,
}

impl Store {
    pub fn get_template_filename(&self) -> String {
        match self.uri.file_name() {
            Some(path) => path.to_str().unwrap().to_string(),
            None => "{name}_{version}.pack".to_string(),
        }
    }
}

#[derive(Debug, Clone)]
pub struct GlobalConfig {
    pub path: PathBuf,
    pub packagers: HashMap<String, PathBuf>,
    pub stores: Vec<Store>,
}

impl GlobalConfig {
    /// Load main remiz configuration file
    /// This TOML configuration file mus be placed in the folder as the remiz binary.
    pub fn load(path: Option<PathBuf>) -> Result<GlobalConfig, RemizError> {
        debug!("Searching Remiz configuration...");
        let global_configuration_path = match path {
            Some(path) => path,
            None => std::env::current_exe()
                .unwrap()
                .parent()
                .unwrap()
                .join("configuration.toml"),
        };
        debug!(
            "Using global configuration file path : {}",
            global_configuration_path.display()
        );

        let content = match fs::read_to_string(&global_configuration_path) {
            Ok(content) => content,
            Err(_) => {
                error!(
                    "No configuration file found at {:?}.
                    You must create this file to fit your needs.",
                    &global_configuration_path
                );
                return Err(RemizError::NoGlobalConfig);
            }
        };

        let toml = match content.parse::<Value>() {
            Ok(value) => value,
            Err(_) => return Err(RemizError::BadTOMLFormat),
        };

        let toml_packagers = toml.get("packagers").unwrap().as_table().unwrap();

        let mut packagers = HashMap::new();
        for (name, path) in toml_packagers {
            packagers.insert(name.to_string(), PathBuf::from(path.as_str().unwrap()));
        }

        let toml_store = toml.get("store").unwrap().as_table().unwrap();

        let mut stores: Vec<Store> = Vec::new();
        for (name, uri) in toml_store {
            stores.push(Store {
                name: name.to_string(),
                uri: PathBuf::from(uri.as_str().unwrap()),
            });
        }

        Ok(GlobalConfig {
            path: global_configuration_path,
            packagers,
            stores,
        })
    }
}
