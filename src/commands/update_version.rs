use std::cmp::Ordering;
use std::fs::{rename, File};
use std::io::{BufRead, BufReader, Write as IoWrite};
use std::path::PathBuf;

use human_sort::compare;

use super::lilypond::global::get_global_version;
use super::scores::{get_found_ly_files, get_selected_items};
use crate::commands::received_confirmation;

fn get_new_version(version: &Option<String>) -> String {
    if let Some(new_version) = version {
        new_version.to_owned()
    } else {
        get_global_version()
    }
}

fn is_outdated(version: &str, new_version: &str) -> bool {
    compare(version, new_version) == Ordering::Less
}

fn update_version(
    file: PathBuf,
    version: &Option<String>,
    new_version: &String,
) {
    let output_file =
        format!("/tmp/{}", file.file_name().unwrap().to_str().unwrap());
    let mut output = File::create(&output_file).unwrap();

    for line in BufReader::new(File::open(&file).unwrap()).lines() {
        let mut line = line.unwrap();

        if line.contains("\\version") {
            let file_version = line.replace("\\version ", "").replace('"', "");

            if version.is_some() || is_outdated(&file_version, new_version) {
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

pub fn update_version_main(
    search_terms: &Vec<String>,
    version: &Option<String>,
    search_artist: &bool,
    search_title: &bool,
    use_all_matches: &bool,
    scores_directory: &Option<String>,
) {
    if search_terms.is_empty()
        && !received_confirmation(
            "Are you sure you want to uppdate all scores? [y/n]",
        )
    {
        return;
    };

    let new_version = get_new_version(version);
    let matching_files = get_found_ly_files(
        search_terms,
        search_artist,
        search_title,
        scores_directory,
    );

    if !use_all_matches && matching_files.len() > 1 {
        if let Ok(selected_items) = get_selected_items(matching_files, true) {
            for item in selected_items.iter() {
                let path = item.output().to_string();
                let path = PathBuf::from(path);
                update_version(path, version, &new_version);
            }
        }
    } else {
        for file in matching_files {
            update_version(file, version, &new_version);
        }
    }
}
