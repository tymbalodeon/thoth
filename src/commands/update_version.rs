use std::cmp::Ordering;
use std::fs::{remove_file, rename, File};
use std::io::{BufRead, BufReader, Write as IoWrite};
use std::path::PathBuf;

use human_sort::compare;

use super::lilypond::global::get_global_version;
use super::scores::{get_found_ly_files, get_selected_items};
use crate::commands::received_confirmation;

fn get_new_version(version: &Option<String>) -> String {
    version
        .as_ref()
        .map_or_else(get_global_version, ToOwned::to_owned)
}

fn is_outdated(version: &str, new_version: &str) -> bool {
    compare(version, new_version) == Ordering::Less
}

fn update_version(
    file: PathBuf,
    version: &Option<String>,
    new_version: &String,
) {
    let output_file = format!(
        "/tmp/{}",
        file.file_name()
            .unwrap_or_else(|| panic!(
                "{}",
                format!("Failed to get file name for {}", &file.display())
            ))
            .to_str()
            .expect("Failed to parse file name as &str.")
    );
    let mut output = File::create(&output_file).unwrap_or_else(|err| {
        panic!("{}", format!("Failed to create file {output_file} ({err})"))
    });
    let mut changed = false;

    for line in BufReader::new(File::open(&file).unwrap_or_else(|err| {
        panic!(
            "{}",
            format!("Failed to open file {} ({})", &file.display(), err)
        )
    }))
    .lines()
    {
        let mut line = line.unwrap_or_else(|err| {
            panic!(
                "{}",
                format!("Failed to read {} ({})", &file.display(), err)
            )
        });
        let version_regex = "\\version ";

        if line.contains(version_regex) {
            let file_version =
                line.replace(version_regex, "").replace('"', "");

            if version.is_some() || is_outdated(&file_version, new_version) {
                line = line.replace(&file_version, new_version);
                changed = true;
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

    if changed {
        rename(output_file, file).ok();
    } else {
        remove_file(output_file).ok();
    }
}

pub fn main(
    search_terms: &Vec<String>,
    version: &Option<String>,
    search_artist: bool,
    search_title: bool,
    use_all_matches: bool,
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
        if let Ok(selected_items) = get_selected_items(&matching_files, true) {
            for item in &selected_items {
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
