use crate::commands::table::print_table;
use crate::config::Config;
use glob::glob;
use std::cmp::Ordering;
use std::fs::{read_dir, DirEntry};
use titlecase::titlecase;

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
) {
    let config = Config::from_config_file();
    let scores_directory = config.scores_directory;
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
                    let entry = entry.unwrap();

                    if entry.file_name() == ".DS_Store" {
                        continue;
                    }

                    let composition = get_display(&entry);
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
                        let path =
                            String::from(entry.file_name().to_str().unwrap());
                        let pdfs_directory = &config.pdfs_directory;
                        let pattern =
                            format!("{pdfs_directory}/{}*.pdf", path);

                        for entry in glob(&pattern)
                            .expect("Failed to read glob pattern")
                        {
                            if entry.is_ok() {
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
