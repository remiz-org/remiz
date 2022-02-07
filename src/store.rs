use std::path::PathBuf;

use convert_case::{Casing, Case};

use crate::package_configuration::PackageConfig;

#[derive(Debug, Clone)]
pub struct Store {
    pub name: String,
    pub uri: PathBuf,
}

impl Store {
    pub fn get_package_file_path(&self, package_config: &PackageConfig) -> PathBuf {
        trace!("Computing package file path for store ('{}')...", &self.name);

        let template_filename = match self.uri.file_name() {
            Some(path) => path.to_str().unwrap().to_string(),
            None => "{name}_{version}.pack".to_string(),
        };

        let package_folder = PathBuf::from(&self.uri.parent().unwrap());
        let raw_path_string = package_folder.join(template_filename).to_string_lossy().to_string();

        let mut result = raw_path_string;

        for (key, value) in &package_config.metadata.info_to_vec() {
            let pattern = format!("{{{}}}", key);
            let value = &value.as_str().unwrap().to_case(Case::Snake);
            result = result.replace(&pattern, value);
        }

        trace!("Computed package file path: {:?}", result);
        return PathBuf::from(result);
    }
}