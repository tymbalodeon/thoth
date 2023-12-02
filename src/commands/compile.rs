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
use super::scores::get_score_ly_file;
use super::scores::get_selected_items;
use super::scores::search;

fn get_modified(file: &PathBuf) -> Option<SystemTime> {
    metadata(file).map_or_else(
        |_| None,
        |file_metadata| {
            Some(
                file_metadata
                    .modified()
                    .expect("Failed to read 'last modified' for file."),
            )
        },
    )
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
            install(&Some(version)).unwrap_or_else(|err| {
                panic!(
                    "{}",
                    format!("Failed to install lilypond version ({err})",)
                )
            });
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
        let err = "Failed to get input file stem.";

        let output_file_pattern = format!(
            "{pdfs_directory}/{}*.pdf",
            input_file.file_stem().expect(err).to_str().expect(err)
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

        for line in
            BufReader::new(File::open(file).expect("Failed to open file."))
                .lines()
        {
            let line = line.expect("Failed to read line in file.");

            if line.contains("\\version") {
                version = line.replace("\\version ", "").replace('"', "");
                break;
            }
        }

        let command = get_binary(version).map_or_else(
            || "lilypond".to_string(),
            |command| format!("{command}/lilypond"),
        );

        match Command::new(command)
            .args(["--include", scores_directory])
            .args(["--output", pdfs_directory])
            .arg(file)
            .output()
        {
            Ok(output) => {
                let err = "Failed to parse input file path.";

                if output.status.success() {
                    let file_name = input_file
                        .file_stem()
                        .expect(err)
                        .to_str()
                        .expect(err);
                    println!("Compiled {pdfs_directory}/{file_name}.pdf");
                } else {
                    io::stdout()
                        .write_all(output.stderr.as_ref())
                        .expect("Failed to print lilypond command output.");
                }
            }
            Err(error) => {
                println!("Error: {error}");
            }
        }
    }
}

pub fn main(
    search_terms: &Vec<String>,
    search_artist: bool,
    search_title: bool,
    use_all_matches: bool,
    scores_directory: &Option<String>,
    pdfs_directory: &Option<String>,
) {
    {
        let pdfs_directory = get_pdfs_directory_from_arg(pdfs_directory);
        create_dir_all(pdfs_directory)
            .expect("Failed to create pdfs directory.");
    }

    let matching_scores =
        search(search_terms, search_artist, search_title, scores_directory);

    if !search_terms.is_empty()
        && !use_all_matches
        && matching_scores.len() > 1
    {
        if let Ok(selected_items) = get_selected_items(&matching_scores, true)
        {
            for item in &selected_items {
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
            let score = score
                .to_str()
                .expect("Failed to parse score file name.")
                .to_string();

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
