use semver::Version;
use serde::{Deserialize, Serialize};
use toml::Value;

/// Represents the 'info' block in package configuration file.
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Metadata {
    /// Package name
    pub name: String,
    /// Package version (must use semver specification)
    pub version: Version,
    /// TOML raw representation of the package configuration file
    pub toml: String,
    pub other: Vec<(String, Value)>,
    /// store userid who created the package
    pub created_by: String,
    /// Remiz version used to create the package
    pub remiz_version: Version,
}

impl Metadata {
    // function to get info section from metadata as a list of key/value vec
    pub fn info_to_vec(&self) -> Vec<(String, Value)> {
        let mut list = Vec::new();
        let parsed_toml = &self.toml.parse::<Value>().unwrap();
        let table = parsed_toml["info"].as_table().unwrap();
        for el in table.iter() {
            list.push((el.0.to_string(), el.1.clone()));
        }
        return list;
    }
}