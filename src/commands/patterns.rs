use std::path::PathBuf;

use glob::glob;

use crate::commands::{
    get_pdfs_directory_from_arg, get_scores_directory_from_arg,
};

fn get_base(
    extension: &str,
    scores_directory: &Option<String>,
    pdfs_directory: &Option<String>,
) -> String {
    if extension == ".pdf" {
        let pdfs_directory = get_pdfs_directory_from_arg(pdfs_directory);
        format!("{pdfs_directory}/**/")
    } else {
        let scores_directory = get_scores_directory_from_arg(scores_directory);
        format!("{scores_directory}/**/")
    }
}

pub fn get_score_file(
    score: &String,
    extension: &str,
    scores_directory: &Option<String>,
    pdfs_directory: &Option<String>,
) -> Option<PathBuf> {
    let base = get_base(extension, scores_directory, pdfs_directory);
    let pattern = format!("{base}*{score}*{extension}");

    for entry in glob(&pattern).expect("Failed to read glob pattern") {
        match entry {
            Ok(path) => return Some(path),
            Err(message) => {
                println!("{:?}", message);
            }
        }
    }

    None
}

pub fn get_patterns(
    scores: &Vec<String>,
    extension: &str,
    scores_directory: &Option<String>,
    pdfs_directory: &Option<String>,
) -> Vec<String> {
    let base = get_base(extension, scores_directory, pdfs_directory);

    if !scores.is_empty() {
        scores
            .iter()
            .map(|score| format!("{base}*{score}*{extension}"))
            .collect()
    } else {
        vec![format!("{base}*{extension}")]
    }
}
