use glob::glob;
use std::io::{self, Write};
use std::path::PathBuf;
use std::process::Command;

use crate::config::get_scores_directory;

pub fn compile(input_file: &PathBuf) -> bool {
    if let Some(file) = input_file.to_str() {
        match Command::new("lilypond").arg(file).output() {
            Ok(output) => {
                if output.status.success() {
                    io::stdout().write_all(output.stdout.as_ref()).unwrap();
                    println!("Compiled {file}");
                    true
                } else {
                    io::stdout().write_all(output.stderr.as_ref()).unwrap();
                    false
                }
            }
            Err(error) => {
                println!("Error: {error}");
                false
            }
        }
    } else {
        false
    }
}

pub fn compile_pdfs(scores: &Vec<String>) {
    let scores_directory = get_scores_directory();
    let base = format!("{scores_directory}/**/");
    let extension = ".ly";

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
                    compile(&path);
                }
                Err(error) => println!("{error}"),
            }
        }
    }
}
