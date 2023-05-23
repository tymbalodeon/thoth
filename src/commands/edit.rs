use crate::commands::patterns::get_scores_directory_pattern;
use glob::glob;
use std::process::Command;

pub fn edit_score(score: &String) {
    let base = get_scores_directory_pattern();

    let patterns: Vec<String> = vec![".ly", ".pdf"]
        .iter()
        .map(|extension| format!("{base}*{score}*{extension}"))
        .collect();

    for pattern in patterns {
        for entry in glob(&pattern).expect("Failed to read glob pattern") {
            match entry {
                Ok(path) => {
                    Command::new("open").arg(path).output().unwrap().stdout;
                }
                Err(message) => println!("{:?}", message),
            }
        }
    }
}
