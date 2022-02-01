use std::path::PathBuf;

use convert_case::{Case, Casing};

use crate::{errors::RemizError, package::Package};

pub fn unpack(path_to_package: PathBuf, destination: Option<PathBuf>) -> Result<(), RemizError> {
    let package = Package::from_file(&path_to_package).unwrap();

    let destination_path = match destination {
        Some(destination) => destination.join(package.metadata.name.to_case(Case::Snake)),
        None => {
            let mut path = path_to_package.clone();
            path.set_extension("");
            path
        }
    };

    debug!(
        "Unpacking package '{}'... to '{}'",
        path_to_package.display(),
        destination_path.display()
    );

    for subpackage in package.subpackages {
        subpackage.decompress(&destination_path)?;
    }

    Ok(())
}
