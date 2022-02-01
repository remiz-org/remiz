use semver::Version;
use serde::{Deserialize, Serialize};
use toml::Value;

// Represents the 'info' block in package configuration file.

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
