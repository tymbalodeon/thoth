use glob::glob;
use std::fs::remove_file;

use crate::config::get_scores_directory;

pub fn clean_pdfs(scores: &Vec<String>) {
    let scores_directory = get_scores_directory();
    let base = format!("{scores_directory}/**/");
    let extension = ".pdf";

    let patterns: Vec<String> = if scores.len() > 0 {
        scores
            .iter()
            .map(|score| format!("{base}*{score}*{extension}"))
            .collect()
    } else {
        vec![format!("{base}*{extension}")]
    };

    for pattern in patterns {
        for entry in glob(&pattern).expect("Failed to read glob pattern") {
            match entry {
                Ok(path) => {
                    if let Err(error) = remove_file(&path) {
                        println!("Removed {}", path.display());
                        println!(
                            "Failed to remove {} ({error})",
                            path.display()
                        );
                    } else {
                        println!("Removed {}", path.display());
                    };
                }
                Err(message) => println!("{:?}", message),
            }
        }
    }
}
