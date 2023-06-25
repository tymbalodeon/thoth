use std::fs::{read_dir, DirEntry};

use glob::glob;
use titlecase::titlecase;

use super::compile::is_already_compiled;
use super::{get_pdfs_directory_from_arg, get_scores_directory_from_arg};
use crate::commands::table::print_table;

fn convert_path_to_string(path: &DirEntry) -> String {
    let artist = String::from(path.file_name().to_str().unwrap());
    artist.replace('-', " ")
}

struct Composition {
    artist: String,
    title: String,
    is_compiled: bool,
}

impl Composition {
    fn get_row_values(&self) -> Vec<String> {
        let artist = titlecase(&self.artist);
        let title = titlecase(&self.title);
        let pdf = self.is_compiled.to_string();

        vec![artist, title, pdf]
    }
}

fn remove_leading_article(value: &String, article: &str) -> String {
    let article = format!("{article} ");

    if value.to_lowercase().starts_with(&article) {
        value[article.len()..].to_string()
    } else {
        value.to_string()
    }
}

fn remove_leading_articles(mut value: String) -> String {
    value = remove_leading_article(&value, "the");
    value = remove_leading_article(&value, "a");
    value = remove_leading_article(&value, "an");

    value
}

pub fn list_main(
    search_terms: &Vec<String>,
    outdated: &bool,
    compiled: &bool,
    search_artist: &bool,
    search_title: &bool,
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
        let search_all_fields = !search_artist && !search_title;

        match entry {
            Ok(path) => {
                if !path.path().is_dir() {
                    continue;
                }

                let artist = convert_path_to_string(&path);

                for entry in read_dir(path.path()).unwrap() {
                    let score_file = entry.unwrap();

                    if score_file.file_name() == ".DS_Store" {
                        continue;
                    }

                    let title = convert_path_to_string(&score_file);
                    let mut is_match = true;

                    if !search_terms.is_empty() {
                        is_match = false;

                        for term in search_terms {
                            if search_all_fields
                                && (artist.contains(term)
                                    || title.contains(term))
                                || (*search_artist && artist.contains(term))
                                || (*search_title && title.contains(term))
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
                                title,
                                is_compiled: pdf,
                            });
                        }
                    }
                }
            }
            Err(message) => println!("{message}"),
        }
    }

    if !compositions.is_empty() {
        compositions.sort_by(|a, b| {
            let self_artist = remove_leading_articles(a.artist.clone());
            let other_artist = remove_leading_articles(b.artist.clone());
            let self_title = remove_leading_articles(a.title.clone());
            let other_title = remove_leading_articles(b.title.clone());

            self_artist
                .cmp(&other_artist)
                .then(self_title.cmp(&other_title))
        });

        let header = vec![
            "ARTIST".to_string(),
            "TITLE".to_string(),
            "STATUS".to_string(),
        ];

        let rows = compositions
            .iter()
            .map(|composition| composition.get_row_values())
            .collect();

        print_table(header, rows);
    }
}
