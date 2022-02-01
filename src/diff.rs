use std::{collections::HashSet, path::PathBuf};

use bat::PrettyPrinter;
use similar::{ChangeTag, TextDiff};

use crate::helpers;

pub fn diff(path_to_package_1: PathBuf, path_to_package_2: PathBuf) {
    debug!(
        "Diffing package {} against {}...",
        path_to_package_1.display(),
        path_to_package_2.display()
    );

    let package_1_filenames: HashSet<String> = helpers::get_filenames(&path_to_package_1)
        .into_iter()
        .filter(|filename| filename.contains("/"))
        .collect();
    let package_2_filenames: HashSet<String> = helpers::get_filenames(&path_to_package_2)
        .into_iter()
        .filter(|filename| filename.contains("/"))
        .collect();

    // Difference between file names
    let diff_1 = package_1_filenames.difference(&package_2_filenames);
    for x in diff_1 {
        info!("Only in {} : {}", &path_to_package_1.display(), x);
    }

    let diff_2 = package_2_filenames.difference(&package_1_filenames);
    for x in diff_2 {
        info!("Only in {} : {}", &path_to_package_2.display(), x);
    }

    let mut package_1_reader = helpers::get_archive_reader(&path_to_package_1).unwrap();
    let mut package_2_reader = helpers::get_archive_reader(&path_to_package_2).unwrap();

    // Print diff between common files
    for x in package_1_filenames.intersection(&package_2_filenames) {
        let archive_file = package_1_reader.get_file(x.to_string());
        let mut data_1 = Vec::new();
        std::io::copy(&mut archive_file.unwrap().unwrap().data, &mut data_1).unwrap();

        let archive_file = package_2_reader.get_file(x.to_string());
        let mut data_2 = Vec::new();
        std::io::copy(&mut archive_file.unwrap().unwrap().data, &mut data_2).unwrap();

        let text_1 = std::str::from_utf8(&data_1).unwrap();
        let text_2 = std::str::from_utf8(&data_2).unwrap();

        // Max 5 seconds for computing a diff
        let deadline = std::time::Instant::now() + std::time::Duration::from_secs(5);
        trace!("Computing diff for {}...", &x);
        let diff = TextDiff::configure()
            .deadline(deadline)
            .diff_lines(text_1, text_2);

        let mut res = String::new();

        for change in diff.iter_all_changes() {
            let sign = match change.tag() {
                ChangeTag::Delete => "-",
                ChangeTag::Insert => "+",
                ChangeTag::Equal => " ",
            };
            if change.tag() != ChangeTag::Equal {
                res += &format!("{}{}", sign, change);
            }
        }

        if !res.is_empty() {
            info!("Difference for '{}':", x);
            PrettyPrinter::new()
                .input_from_bytes(res.as_bytes())
                .language("diff")
                .print()
                .unwrap();
        }
    }
}
