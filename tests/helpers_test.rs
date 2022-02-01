use std::{
    env,
    fs::File,
    io::{self, Read},
    path::PathBuf,
    process::Command,
};

use mla::{config::ArchiveReaderConfig, ArchiveReader};

/// This function is used to test the CLI and returns the path to Remiz executable
pub fn get_remiz_command() -> Command {
    let target_dir = env::var("CARGO_TARGET_DIR").unwrap_or_else(|_| String::from("target"));
    Command::new(format!("{}/debug/remiz", target_dir))
}

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

/// Unoptimized function to compute stable hash of a package
pub fn get_hash(path: &PathBuf) -> String {
    // To get the hash, we first need to decode the MLA archive
    // and then compute the hash on the decompressed data.

    // Open the file
    let mut f = File::open(path).unwrap();
    let mut data = Vec::new();
    f.read_to_end(&mut data).unwrap();

    // Read archive    use crate::helpers;
    let config = ArchiveReaderConfig::new();
    let buffer = io::Cursor::new(data);

    let mut mla_read = ArchiveReader::from_config(buffer, config).unwrap();
    let mut archive_data: Vec<u8> = Vec::new();

    let list = get_filenames(&path);

    for filename in list {
        let file = mla_read.get_file(filename.to_string());
        let mut output = Vec::new();
        std::io::copy(&mut file.unwrap().unwrap().data, &mut output).unwrap();
        archive_data.extend(output);
    }

    blake3::hash(&archive_data).to_hex().to_string()
}
