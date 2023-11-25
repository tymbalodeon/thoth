use std::cmp::Ordering;
use std::fs::{rename, File};
use std::io::{BufRead, BufReader, Write as IoWrite};

use human_sort::compare;

use super::lilypond::global::get_global_version;
use super::scores::get_found_ly_files;

fn get_new_version(version: &Option<String>) -> String {
    if let Some(new_version) = version {
        new_version.to_owned()
    } else {
        get_global_version()
    }
}

fn is_outdated(version: &String, new_version: &String) -> bool {
    compare(version, new_version) == Ordering::Less
}

pub fn bump_version_main(
    search_terms: &Vec<String>,
    version: &Option<String>,
    scores_directory: &Option<String>,
) {
    let new_version = get_new_version(version);
    let ly_files =
        get_found_ly_files(search_terms, &false, &false, scores_directory);

    for file in ly_files {
        let output_file =
            format!("/tmp/{}", file.file_name().unwrap().to_str().unwrap());
        let mut output = File::create(&output_file).unwrap();

        for line in BufReader::new(File::open(&file).unwrap()).lines() {
            let mut line = line.unwrap();

            if line.contains("\\version") {
                let file_version =
                    line.replace("\\version ", "").replace('"', "");

                if version.is_some()
                    || is_outdated(&file_version, &new_version)
                {
                    line = line.replace(&file_version, new_version.as_str());
                    println!(
                        "Updated {} to lilypond {}",
                        &file.display(),
                        &new_version
                    );
                }
            }

            line = format!("{line}\n");
            output.write(line.as_bytes()).ok();
        }

        rename(output_file, file).ok();
    }
}
