use std::fs::{rename, File};
use std::io::{BufRead, BufReader, Write as IoWrite};

use super::scores::get_found_ly_files;
use crate::commands::lilypond::install::get_latest_version;

pub fn bump_version_main(
    search_terms: &Vec<String>,
    version: &Option<String>,
    scores_directory: &Option<String>,
) {
    let new_version = if let Some(new_version) = version {
        new_version.to_owned()
    } else {
        get_latest_version("latest-unstable").unwrap()
    };

    let ly_files =
        get_found_ly_files(search_terms, &false, &false, scores_directory);

    for file in ly_files {
        let output_file =
            format!("/tmp/{}", file.file_name().unwrap().to_str().unwrap());
        let mut output = File::create(&output_file).unwrap();

        for line in BufReader::new(File::open(&file).unwrap()).lines() {
            let mut line = line.unwrap();

            if line.contains("\\version") {
                let version = line.replace("\\version ", "").replace('"', "");
                line = line.replace(&version, new_version.as_str());
            }

            line = format!("{line}\n");
            output.write(line.as_bytes()).ok();
        }

        rename(output_file, file).ok();
    }
}
