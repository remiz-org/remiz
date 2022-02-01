use core::fmt;
use std::{io, path::PathBuf};

#[derive(Debug)]
pub enum RemizError {
    BadRemizFormat,
    BadTOMLFormat,
    NoGlobalConfig,
    PermissionDenied,
    SubpackageNotCreated,
    SubpackagerFailed,
    SubpackagerNotFound,
    FileNotFound(String),
    IOError(io::Error),
    PackageAlreadyExists(PathBuf),
}

impl fmt::Display for RemizError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // For now, use the debug derived version
        write!(f, "{:?}", self)
    }
}

impl From<io::Error> for RemizError {
    fn from(error: io::Error) -> Self {
        RemizError::IOError(error)
    }
}
