use std::fs::read_dir;
use std::fs::DirEntry;
use std::io::ErrorKind;
use std::io::{Cursor, Error};
use std::path::PathBuf;

use glob::glob;
use skim::prelude::*;

use crate::commands::compile::is_compiled;
use crate::commands::get_pdfs_directory_from_arg;
use crate::commands::get_scores_directory_from_arg;

pub const TEMPORARY_DIRECTORY: &str = "/tmp/thoth";

pub fn get_temporary_ly_file() -> String {
    format!("{TEMPORARY_DIRECTORY}/sketch.ly")
}

fn get_items(paths: &[PathBuf]) -> Receiver<Arc<dyn SkimItem>> {
    let paths: Vec<&str> = paths
        .iter()
        .map(|path| path.to_str().expect("Failed to parse path."))
        .collect();
    let input = paths.join("\n");
    let item_reader = SkimItemReader::default();

    item_reader.of_bufread(Cursor::new(input))
}

pub fn get_selected_items(
    matching_scores: &[PathBuf],
    multi: bool,
) -> Result<Vec<Arc<dyn SkimItem>>, Error> {
    let options = SkimOptionsBuilder::default()
        .multi(multi)
        .build()
        .expect("Failed to create Skim builder.");
    let selections = get_items(matching_scores);

    let selected_items = Skim::run_with(&options, Some(selections));

    for item in &selected_items {
        if item.is_abort {
            return Err(Error::new(ErrorKind::Other, "User aborted"));
        }
    }

    Ok(selected_items
        .map(|output| output.selected_items)
        .unwrap_or_default())
}

fn convert_path_to_string(path: &DirEntry) -> String {
    let artist = String::from(
        path.file_name().to_str().expect("Failed to parse artist."),
    );
    artist.replace('-', " ")
}

pub fn search(
    search_terms: &Vec<String>,
    search_artist: bool,
    search_title: bool,
    scores_directory: &Option<String>,
) -> Vec<PathBuf> {
    let scores_directory = get_scores_directory_from_arg(scores_directory);
    let score_files = format!("{scores_directory}/scores");
    let scores = read_dir(&score_files);

    let mut matching_scores = vec![];

    if scores.is_err() {
        return matching_scores;
    }

    for entry in read_dir(score_files).unwrap_or_else(|err| panic!("{err}")) {
        match entry {
            Ok(path) => {
                if !path.path().is_dir() {
                    continue;
                }

                for entry in
                    read_dir(path.path()).unwrap_or_else(|err| panic!("{err}"))
                {
                    let score_file =
                        entry.unwrap_or_else(|err| panic!("{err}"));

                    if score_file.file_name() == ".DS_Store" {
                        continue;
                    }

                    let artist = convert_path_to_string(&path);
                    let title = convert_path_to_string(&score_file);
                    let mut is_match = true;

                    if !search_terms.is_empty() {
                        is_match = false;

                        for term in search_terms {
                            if !search_artist
                                && !search_title
                                && (artist.contains(term)
                                    || title.contains(term))
                                || (search_artist && artist.contains(term))
                                || (search_title && title.contains(term))
                            {
                                is_match = true;
                                break;
                            }
                        }
                    }

                    if is_match {
                        matching_scores.push(score_file.path());
                    }
                }
            }
            Err(message) => println!("{message}"),
        }
    }

    matching_scores
}

pub fn get_score_ly_file(score: &String) -> Option<String> {
    glob(&format!("{score}/*.ly"))
        .expect("Failed to read glob pattern")
        .flatten()
        .next()
        .map(|ly_file| {
            ly_file
                .to_str()
                .expect("Failed to get score file path.")
                .to_string()
        })
}

pub fn get_found_ly_files(
    search_terms: &Vec<String>,
    search_artist: bool,
    search_title: bool,
    scores_directory: &Option<String>,
) -> Vec<PathBuf> {
    let found_scores =
        search(search_terms, search_artist, search_title, scores_directory);

    found_scores
        .iter()
        .filter_map(|score| {
            get_score_ly_file(
                &score
                    .to_str()
                    .expect("Failed to get score file path.")
                    .to_string(),
            )
        })
        .map(PathBuf::from)
        .collect()
}

pub fn get_found_pdfs(
    search_terms: &Vec<String>,
    search_artist: bool,
    search_title: bool,
    scores_directory: &Option<String>,
    pdfs_directory: &Option<String>,
) -> Vec<PathBuf> {
    let pdfs_directory = &get_pdfs_directory_from_arg(pdfs_directory);

    let matching_scores =
        search(search_terms, search_artist, search_title, scores_directory);

    let mut matching_pdfs = vec![];

    for score in matching_scores {
        let pattern = format!("{}/*.ly", score.display());

        for input_file in glob(&pattern)
            .expect("Failed to read glob pattern")
            .flatten()
        {
            let output_file_pattern = format!(
                "{pdfs_directory}/*{}*.pdf",
                input_file
                    .file_stem()
                    .unwrap_or_else(|| panic!(
                        "{}",
                        format!(
                            "Failed to get file stem for file: {}",
                            &input_file.display()
                        )
                    ))
                    .to_str()
                    .unwrap_or_else(|| panic!(
                        "{}",
                        format!(
                            "Failed to get file stem for file: {}",
                            &input_file.display()
                        )
                    ))
            );

            for entry in glob(&output_file_pattern)
                .expect("Failed to read glob pattern")
                .flatten()
            {
                if is_compiled(&input_file, &entry) {
                    matching_pdfs.push(entry);
                }
            }
        }
    }

    matching_pdfs
}
