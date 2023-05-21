use glob::glob;
use std::fs::remove_file;
use std::io::Write;
use std::io::{stdin, stdout};

use crate::config::Config;

fn confirmation_received() -> bool {
    print!("Are you sure you want to remove all pdfs? [y/n] ");
    stdout().flush().unwrap();
    let mut response = String::new();
    let stdin = stdin();
    stdin
        .read_line(&mut response)
        .expect("Failed to read input.");
    response = response.replace("\n", "");

    if response.eq("y") || response.eq("Y") {
        true
    } else {
        false
    }
}

pub fn clean_pdfs(scores: &Vec<String>) {
    if scores.len() == 0 && !confirmation_received() {
        return;
    };

    let scores_directory = Config::new().scores_directory();
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
