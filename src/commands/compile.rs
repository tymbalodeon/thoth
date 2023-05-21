use crate::config::Config;
use glob::glob;
use std::fs::create_dir_all;
use std::io::{self, Write};
use std::path::PathBuf;
use std::process::Command;

fn compile_input_file(input_file: &PathBuf, config: &Config) {
    let pdfs_directory = config.pdfs_directory();
    create_dir_all(&pdfs_directory).unwrap();

    if let Some(file) = input_file.to_str() {
        match Command::new("lilypond")
            .args(["--include", &config.scores_directory()])
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
    let config: Config = Config::new();
    let scores_directory = config.scores_directory();
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
                    compile_input_file(&path, &config);
                }
                Err(error) => println!("{error}"),
            }
        }
    }
}
