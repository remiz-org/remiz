use std::{
    fs::File,
    io::{self, Cursor, Read},
    path::PathBuf,
};

use mla::{config::ArchiveReaderConfig, ArchiveReader};

use crate::errors::RemizError;

/// Returns the list of filenames given a path to a MLA archive
pub fn get_filenames(path: &PathBuf) -> Vec<String> {
    // Open the file
    let mut f = File::open(path).unwrap();
    let mut data = Vec::new();
    f.read_to_end(&mut data).unwrap();

    // Read archive
    let config = ArchiveReaderConfig::new();
    let buffer = io::Cursor::new(data);

    let mla_read = ArchiveReader::from_config(buffer, config).unwrap();

    let tmp = mla_read.list_files().unwrap().collect::<Vec<&String>>();
    let mut res: Vec<String> = Vec::new();

    for r in tmp {
        res.push(r.to_string());
    }
    res.sort();

    res
}

pub fn get_archive_reader(path: &PathBuf) -> Result<ArchiveReader<Cursor<Vec<u8>>>, RemizError> {
    // Open the file
    debug!("Opening file '{}'...", path.display());
    let mut f = File::open(&path)?;
    let mut data = Vec::new();
    f.read_to_end(&mut data).unwrap();

    // Read archive
    let config = ArchiveReaderConfig::new();
    let buffer = io::Cursor::new(data);

    return match ArchiveReader::from_config(buffer, config) {
        Ok(reader) => Ok(reader),
        Err(_) => {
            error!("Error reading archive. Please check file type.");
            return Err(RemizError::BadRemizFormat);
        }
    };
}
