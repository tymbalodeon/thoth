use crate::config::Config;
use glob::glob;
use std::path::PathBuf;

fn get_base(extension: &str) -> String {
    if extension == ".pdf" {
        let pdfs_directory = Config::get_pdfs_directory();
        format!("{pdfs_directory}/**/")
    } else {
        let scores_directory = Config::get_scores_directory();
        format!("{scores_directory}/**/")
    }
}

pub fn get_score_file(score: &String, extension: &str) -> Option<PathBuf> {
    let base = get_base(extension);
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

pub fn get_patterns(scores: &Vec<String>, extension: &str) -> Vec<String> {
    let base = get_base(extension);

    if !scores.is_empty() {
        scores
            .iter()
            .map(|score| format!("{base}*{score}*{extension}"))
            .collect()
    } else {
        vec![format!("{base}*{extension}")]
    }
}
