use bat::PrettyPrinter;
use std::path::PathBuf;

/// Inspect command
pub fn inspect(path: PathBuf) {
    debug!("Inspecting package {}...", path.display());

    let package = crate::package::Package::from_file(&path);

    match package {
        Ok(package) => {
            info!("Package name        : {}", package.metadata.name);
            info!("Package version     : {}", package.metadata.version);
            info!("Package created by  : {}", package.metadata.created_by);
            info!("Builded with remiz : {}", package.metadata.remiz_version);

            info!("TOML configuration file of the package:");

            let toml = package.metadata.toml;

            PrettyPrinter::new()
                .input_from_bytes(toml.as_bytes())
                .language("toml")
                .print()
                .unwrap();
            println!("");

            if package.metadata.other.len() > 0 {
                info!("Additionnal user fields :");
                for (key, value) in package.metadata.other {
                    info!("     - {} : {}", key, value);
                }
            }

            info!("Number of subpackages : {}", package.subpackages.len());
            // List all subpackages
            for sub in package.subpackages {
                info!("Subpackage : {}", sub.name);
                for file in sub.files.keys() {
                    info!("    - {}", file);
                }
            }
        }
        Err(err) => {
            error!(
                "Failed to inspect package ({}). File may be corrupted.",
                err
            );
        }
    }
}
