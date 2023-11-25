use std::fs::{create_dir_all, metadata, File};
use std::io::{self, BufRead, BufReader, Write};
use std::path::{Path, PathBuf};
use std::println;
use std::process::Command;
use std::time::SystemTime;

use glob::glob;

use super::get_pdfs_directory_from_arg;
use super::get_scores_directory_from_arg;
use super::lilypond::global::get_global_version;
use super::lilypond::install::{get_install_path, install};
use super::lilypond::is_valid_version;
use super::scores::get_matching_scores;
use super::scores::get_score_ly_file;
use super::scores::get_selected_items;

fn get_modified(file: &PathBuf) -> Option<SystemTime> {
    if let Ok(file_metadata) = metadata(file) {
        Some(file_metadata.modified().unwrap())
    } else {
        None
    }
}

pub fn is_compiled(input_file: &PathBuf, output_file: &PathBuf) -> bool {
    let input_modified = get_modified(input_file);
    let output_modified = get_modified(output_file);

    input_modified <= output_modified
}

fn get_binary(version: String) -> Option<String> {
    let global_version = get_global_version();
    let install_path = get_install_path();
    let version_path = format!("{}/lilypond-{}/bin", &install_path, &version);

    if global_version != version && is_valid_version(&version) {
        if !Path::new(&version_path).exists() {
            install(&Some(version)).unwrap();
        }

        Some(version_path)
    } else {
        None
    }
}

pub fn compile_input_file(
    input_file: &PathBuf,
    scores_directory: &Option<String>,
    pdfs_directory: &Option<String>,
) {
    let pdfs_directory = &get_pdfs_directory_from_arg(pdfs_directory);
    let scores_directory = &get_scores_directory_from_arg(scores_directory);

    if let Some(file) = input_file.to_str() {
        let output_file_pattern = format!(
            "{pdfs_directory}/{}*.pdf",
            input_file.file_stem().unwrap().to_str().unwrap()
        );

        for entry in glob(&output_file_pattern)
            .expect("Failed to read glob pattern")
            .flatten()
        {
            if is_compiled(input_file, &entry) {
                return;
            }
        }

        let mut version = String::new();

        for line in BufReader::new(File::open(file).unwrap()).lines() {
            let line = line.unwrap();

            if line.contains("\\version") {
                version = line.replace("\\version ", "").replace('"', "");
                break;
            }
        }

        let command = if let Some(command) = get_binary(version) {
            format!("{command}/lilypond")
        } else {
            "lilypond".to_string()
        };

        match Command::new(command)
            .args(["--include", scores_directory])
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

pub fn compile_main(
    search_terms: &Vec<String>,
    search_artist: &bool,
    search_title: &bool,
    use_all_matches: &bool,
    scores_directory: &Option<String>,
    pdfs_directory: &Option<String>,
) {
    {
        let pdfs_directory = get_pdfs_directory_from_arg(pdfs_directory);
        create_dir_all(pdfs_directory).unwrap();
    }

    let matching_scores = get_matching_scores(
        search_terms,
        search_artist,
        search_title,
        scores_directory,
    );

    if !search_terms.is_empty()
        && !use_all_matches
        && matching_scores.len() > 1
    {
        if let Ok(selected_items) = get_selected_items(matching_scores, true) {
            for item in selected_items.iter() {
                let score = item.output().to_string();

                if let Some(input_file) = get_score_ly_file(&score) {
                    compile_input_file(
                        &PathBuf::from(input_file),
                        scores_directory,
                        pdfs_directory,
                    );
                }
            }
        }
    } else {
        for score in matching_scores {
            let score = score.to_str().unwrap().to_string();

            if let Some(input_file) = get_score_ly_file(&score) {
                compile_input_file(
                    &PathBuf::from(input_file),
                    scores_directory,
                    pdfs_directory,
                );
            }
        }
    }
}
