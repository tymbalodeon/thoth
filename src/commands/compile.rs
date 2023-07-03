use std::fs::create_dir_all;
use std::fs::metadata;
use std::io::{self, Write};
use std::path::Path;
use std::path::PathBuf;
use std::println;
use std::process::Command;
use std::time::SystemTime;

use glob::glob;

use super::get_pdfs_directory_from_arg;
use super::get_scores_directory_from_arg;
use super::scores::get_found_scores;
use super::scores::get_matching_scores;
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

pub fn compile_input_file(
    input_file: &PathBuf,
    scores_directory: &Option<String>,
    pdfs_directory: &Option<String>,
) {
    let pdfs_directory = &get_pdfs_directory_from_arg(pdfs_directory);
    let scores_directory = &get_scores_directory_from_arg(scores_directory);

    if let Some(file) = input_file.to_str() {
        let output_file_pattern = format!(
            "{pdfs_directory}/*{}*.pdf",
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

        match Command::new("lilypond")
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

fn compile_selected_scores(
    scores: &Vec<String>,
    use_all_matches: &bool,
    scores_directory: &Option<String>,
    pdfs_directory: &Option<String>,
) {
    let matching_scores =
        get_matching_scores(scores, ".ly", scores_directory, pdfs_directory);

    if !use_all_matches && matching_scores.len() > 1 {
        let selected_items = get_selected_items(matching_scores, true);

        for item in selected_items.iter() {
            let path = item.output().to_string();
            let path = Path::new(&path);
            compile_input_file(
                &path.to_path_buf(),
                scores_directory,
                pdfs_directory,
            );
        }
    } else {
        for score in matching_scores {
            compile_input_file(&score, scores_directory, pdfs_directory);
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
        create_dir_all(&pdfs_directory).unwrap();
    }

    if search_terms.is_empty() {
        let found_scores = get_found_scores(
            search_terms,
            search_artist,
            search_title,
            scores_directory,
        );

        for score in found_scores {
            compile_input_file(&score, scores_directory, pdfs_directory);
        }
    } else {
        compile_selected_scores(
            search_terms,
            use_all_matches,
            scores_directory,
            pdfs_directory,
        );
    }
}
