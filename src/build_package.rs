use std::path::PathBuf;

use convert_case::{Case, Casing};

use crate::errors::RemizError;
use crate::global_configuration::GlobalConfig;
use crate::package::Package;
use crate::package_configuration::PackageConfig;

/// Try to build a package given a configuration file
pub fn build(
    path_to_config_file: PathBuf,
    global_config_file: Option<PathBuf>,
) -> Result<(), RemizError> {
    let global_conf = GlobalConfig::load(global_config_file)?;

    let package_config = PackageConfig::from_file(path_to_config_file)?;

    // Build .pack package...
    let package = Package::from_config(&global_conf, &package_config)?;

    // For each store used, write .pack
    for store in global_conf.stores {
        let template_filename = store.get_template_filename();
        let filename = template_filename.replace("{name}", &package_config.metadata.name);
        let filename = filename.to_case(Case::Snake);
        let filename = filename.replace("{version}", &package_config.metadata.version.to_string());
        trace!("Getting parent store uri ('{}')...", store.name);
        let package_folder = PathBuf::from(store.uri.parent().unwrap());
        let package_path = package_folder.join(filename);

        // if package already exists, throw error
        if package_path.exists() {
            error!(
                "{} already exists. Please bump version or remove file.",
                package_path.display()
            );
            return Err(RemizError::PackageAlreadyExists(package_path));
        }

        package.write(&package_path)?;
        info!(
            "Package created at {:?} (store '{}')",
            package_path, store.name
        );
    }

    Ok(())
}
