use std::{
    collections::HashMap,
    fs::File,
    io::Write,
    path::{Path, PathBuf},
};

use uuid::Uuid;

use crate::errors::RemizError;

#[derive(Debug)]
pub struct Subpackage {
    pub name: String,
    pub files: HashMap<String, Vec<u8>>,
}

impl Subpackage {
    /// Decompress files inside the subpackage to the given path.
    /// Return the magic path created.
    pub fn uncompress(&self, base_path: &PathBuf) -> Result<PathBuf, RemizError> {
        let uuid = Uuid::new_v4();
        let folder_name = format!("{}_{}", &self.name, uuid.to_simple());

        let folder_path = base_path.join(&folder_name);

        for (archive_file_path, data) in &self.files {
            let destination_file_path = folder_path.join(
                (&archive_file_path)
                    .split("/")
                    .collect::<Vec<&str>>()
                    .split_first()
                    .unwrap()
                    .1
                    .join("/")
                    .to_owned(),
            );

            debug!(
                "Copying file '{}' from subpackage '{}' to '{}'...",
                &archive_file_path,
                &self.name,
                destination_file_path.display()
            );
            let prefix = Path::new(&destination_file_path).parent().unwrap();
            // creates directory if it does not exist
            std::fs::create_dir_all(prefix).unwrap();
            let mut file = File::create(&destination_file_path)?;
            file.write_all(data)?;
        }

        Ok(folder_path)
    }
}
