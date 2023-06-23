use std::cmp::Ordering;
use std::fs::{read_dir, DirEntry};

use glob::glob;
use titlecase::titlecase;

use super::compile::is_already_compiled;
use super::{get_pdfs_directory_from_arg, get_scores_directory_from_arg};
use crate::commands::table::print_table;

fn get_display(path: &DirEntry) -> String {
    let artist = String::from(path.file_name().to_str().unwrap());
    artist.replace('-', " ")
}

#[derive(Eq)]
struct Composition {
    artist: String,
    composition: String,
    pdf: bool,
}

impl Composition {
    fn remove_leading_the(&self) -> String {
        self.artist.replace("the ", "")
    }

    fn get_row_values(&self) -> Vec<String> {
        let artist = titlecase(&self.artist);
        let title = titlecase(&self.composition);
        let pdf = self.pdf.to_string();

        vec![artist, title, pdf]
    }
}

impl Ord for Composition {
    fn cmp(&self, other: &Self) -> Ordering {
        let self_artist = self.remove_leading_the();
        let other_artist = other.remove_leading_the();
        self_artist.cmp(&other_artist)
    }
}

impl PartialOrd for Composition {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl PartialEq for Composition {
    fn eq(&self, other: &Self) -> bool {
        self.artist == other.artist
    }
}

pub fn list_main(
    search_terms: &Vec<String>,
    outdated: &bool,
    compiled: &bool,
    scores_directory: &Option<String>,
    pdfs_directory: &Option<String>,
) {
    let scores_directory = get_scores_directory_from_arg(scores_directory);
    let score_files = format!("{scores_directory}/scores");
    let mut compositions: Vec<Composition> = vec![];
    let scores = read_dir(&score_files);

    if scores.is_err() {
        return;
    }

    for entry in read_dir(score_files).unwrap() {
        match entry {
            Ok(path) => {
                if !path.path().is_dir() {
                    continue;
                }

                let artist = get_display(&path);

                for entry in read_dir(path.path()).unwrap() {
                    let score_file = entry.unwrap();

                    if score_file.file_name() == ".DS_Store" {
                        continue;
                    }

                    let composition = get_display(&score_file);
                    let mut is_match = true;

                    if !search_terms.is_empty() {
                        is_match = false;

                        for term in search_terms {
                            if artist.contains(term)
                                || composition.contains(term)
                            {
                                is_match = true;
                                break;
                            }
                        }
                    }

                    if is_match {
                        let mut pdf = false;
                        let path = String::from(
                            score_file.file_name().to_str().unwrap(),
                        );
                        let pdfs_directory =
                            get_pdfs_directory_from_arg(pdfs_directory);
                        let pattern =
                            format!("{pdfs_directory}/{}*.pdf", path);

                        for pdf_file in glob(&pattern)
                            .expect("Failed to read glob pattern")
                            .flatten()
                        {
                            if is_already_compiled(
                                &score_file.path(),
                                &pdf_file,
                            ) {
                                pdf = true;
                                break;
                            }
                        }

                        let should_display = *outdated && !pdf
                            || *compiled && pdf
                            || !*outdated && !*compiled;

                        if should_display {
                            compositions.push(Composition {
                                artist: artist.clone(),
                                composition,
                                pdf,
                            });
                        }
                    }
                }
            }
            Err(message) => println!("{message}"),
        }
    }

    if !compositions.is_empty() {
        let titles = vec![
            "ARTIST".to_string(),
            "COMPOSITION".to_string(),
            "STATUS".to_string(),
        ];

        let rows = compositions
            .iter()
            .map(|composition| composition.get_row_values())
            .collect();

        print_table(titles, rows);
    }
}
