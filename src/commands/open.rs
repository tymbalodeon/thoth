#![allow(clippy::module_name_repetitions)]

use std::{
    path::{Path, PathBuf},
    process::Command,
};

use super::{
    scores::{get_found_ly_files, get_found_pdfs, get_selected_items},
    ScoreFileType,
};

pub fn open_file(file_path: &Path) {
    let file_path = file_path
        .to_str()
        .expect("Failed to parse file path.")
        .to_string();
    Command::new("open")
        .arg(&file_path)
        .output()
        .expect("Failed to run `open` command.");
    println!("Opened {file_path}");
}

pub fn main(
    search_terms: &Vec<String>,
    search_artist: bool,
    search_title: bool,
    use_all_matches: bool,
    file_type: &Option<ScoreFileType>,
    scores_directory: &Option<String>,
    pdfs_directory: &Option<String>,
) {
    let matching_files = match file_type {
        Some(ScoreFileType::Both) => {
            let mut ly_files = get_found_ly_files(
                search_terms,
                search_artist,
                search_title,
                scores_directory,
            );

            let pdfs = get_found_pdfs(
                search_terms,
                search_artist,
                search_title,
                scores_directory,
                pdfs_directory,
            );

            ly_files.extend(pdfs);

            ly_files
        }
        Some(ScoreFileType::Lilypond) => get_found_ly_files(
            search_terms,
            search_artist,
            search_title,
            scores_directory,
        ),
        Some(ScoreFileType::Pdf) | None => get_found_pdfs(
            search_terms,
            search_artist,
            search_title,
            scores_directory,
            pdfs_directory,
        ),
    };

    if !use_all_matches && matching_files.len() > 1 {
        if let Ok(selected_items) = get_selected_items(&matching_files, true) {
            for item in &selected_items {
                let path = item.output().to_string();
                let path = PathBuf::from(path);
                open_file(&path);
            }
        }
    } else {
        for score in matching_files {
            open_file(&score);
        }
    }
}
