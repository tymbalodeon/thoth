use crate::commands::patterns::get_patterns;
use crate::config::Config;
use glob::glob;
use shellexpand::tilde;
use std::fs::create_dir_all;
use std::fs::metadata;
use std::io::{self, Write};
use std::path::PathBuf;
use std::println;
use std::process::Command;
use std::time::SystemTime;

fn get_modified(file: &PathBuf) -> Option<SystemTime> {
    if let Ok(file_metadata) = metadata(file) {
        Some(file_metadata.modified().unwrap())
    } else {
        None
    }
}

pub fn is_already_compiled(
    input_file: &PathBuf,
    output_file: &PathBuf,
) -> bool {
    let input_modified = get_modified(input_file);
    let output_modified = get_modified(output_file);

    input_modified <= output_modified
}

fn compile_input_file(input_file: &PathBuf, pdfs_directory: &String) {
    create_dir_all(pdfs_directory).unwrap();

    if let Some(file) = input_file.to_str() {
        let output_file_pattern = format!(
            "{pdfs_directory}/*{}*.pdf",
            input_file.file_stem().unwrap().to_str().unwrap()
        );

        for entry in glob(&output_file_pattern)
            .expect("Failed to read glob pattern")
            .flatten()
        {
            if is_already_compiled(input_file, &entry) {
                return;
            }
        }

        match Command::new("lilypond")
            .args(["--include", &Config::get_scores_directory()])
            .args(["--output", pdfs_directory])
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

pub fn compile_main(scores: &Vec<String>, pdfs_directory: &Option<String>) {
    let pdfs_directory = if let Some(path) = pdfs_directory {
        tilde(&path).to_string()
    } else {
        Config::get_pdfs_directory()
    };

    let patterns = get_patterns(scores, ".ly");

    for pattern in patterns {
        for entry in glob(&pattern).expect("Failed to read glob pattern") {
            match entry {
                Ok(path) => {
                    compile_input_file(&path, &pdfs_directory);
                }
                Err(error) => println!("{error}"),
            }
        }
    }
}
