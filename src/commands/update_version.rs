use std::path::PathBuf;
use std::process::Command;

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

fn update_version(file: PathBuf, new_version: &String) {
    match Command::new("convert-ly")
        .args(["--to", new_version])
        .arg("--edit")
        .arg(&file.display().to_string())
        .output()
    {
        Ok(result) => {
            println!("{}", String::from_utf8(result.stderr).unwrap(),)
        }
        Err(err) => println!("{err}"),
    }
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
                update_version(path, &new_version);
            }
        }
    } else {
        for file in matching_files {
            update_version(file, &new_version);
        }
    }
}
