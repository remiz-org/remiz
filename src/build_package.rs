use std::path::PathBuf;

use crate::errors::RemizError;
use crate::global_configuration::GlobalConfig;
use crate::package::Package;
use crate::package_configuration::PackageConfig;


/// Try to build a package given a configuration file
pub fn build(
    path_to_config_file: PathBuf,
    global_config_file: Option<PathBuf>,
    extra_args: Vec<String>,
) -> Result<(), RemizError> {
    let global_conf = GlobalConfig::load(global_config_file)?;

    let package_config = PackageConfig::from_file(path_to_config_file)?;

    // Build .pack package...
    let package = Package::from_config(&global_conf, &package_config, extra_args)?;

    // For each store used, write .pack
    for store in global_conf.stores {
        let destination_path = store.get_package_file_path(&package_config);

        // make path relative to global conf path (is relative)
         let destination_path = match destination_path.is_relative() {
            true => global_conf.path.parent().unwrap().join(destination_path),
            false => destination_path,
        };

        // if package already exists, throw error
        if destination_path.exists() {
            error!(
                "{} already exists. Please bump version or remove file.",
                destination_path.display()
            );
            return Err(RemizError::PackageAlreadyExists(destination_path));
        }

        package.write(&destination_path)?;
        info!(
            "Package created at {:?} (store '{}')",
            destination_path, store.name
        );
    }

    Ok(())
}
