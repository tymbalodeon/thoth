use crate::config::Config;
use glob::glob;
use std::path::PathBuf;

fn get_scores_directory_pattern() -> String {
    let config: Config = Config::from_config_file();
    let scores_directory = config.scores_directory;
    format!("{scores_directory}/**/")
}

pub fn get_score_file(score: &String, extension: &str) -> Option<PathBuf> {
    let base = get_scores_directory_pattern();
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
    let base = get_scores_directory_pattern();

    if !scores.is_empty() {
        scores
            .iter()
            .map(|score| format!("{base}*{score}*{extension}"))
            .collect()
    } else {
        vec![format!("{base}*{extension}")]
    }
}
