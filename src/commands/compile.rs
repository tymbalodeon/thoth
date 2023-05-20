use glob::glob;
use std::fs::create_dir_all;
use std::io::{self, Write};
use std::path::PathBuf;
use std::process::Command;

use crate::config::{get_pdfs_directory, get_scores_directory};

fn compile(input_file: &PathBuf) {
    let pdfs_directory = get_pdfs_directory();

    match create_dir_all(&pdfs_directory) {
        Err(message) => println!("Error: {message}"),
        Ok(_) => {}
    }

    if let Some(file) = input_file.to_str() {
        match Command::new("lilypond")
            .args(["--include", &get_scores_directory()])
            .args(["--output", &pdfs_directory])
            .arg(file)
            .output()
        {
            Ok(output) => {
                if output.status.success() {
                    let file_name =
                        input_file.file_stem().unwrap().to_str().unwrap();
                    println!("Compiled {pdfs_directory}/{}.pdf", file_name);
                } else {
                    io::stdout().write_all(output.stderr.as_ref()).unwrap();
                }
            }
            Err(error) => {
                println!("Error: {error}");
            }
        }
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
