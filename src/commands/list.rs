use crate::config::Config;
use glob::glob;
use std::{
    cmp::Reverse,
    fs::{read_dir, DirEntry},
};
use titlecase::titlecase;

fn get_display(path: &DirEntry) -> String {
    let artist = String::from(path.file_name().to_str().unwrap());
    artist.replace('-', " ")
}

struct Composition {
    artist: String,
    composition: String,
    pdf: bool,
}

pub fn list_scores(search_terms: &Vec<String>) {
    let config = Config::new();
    let scores_directory = config.scores_directory();
    let score_files = format!("{scores_directory}/scores");
    let mut compositions: Vec<Composition> = vec![];

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
                        let pdfs_directory = config.pdfs_directory();
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

                        compositions.push(Composition {
                            artist: artist.clone(),
                            composition,
                            pdf,
                        });
                    }
                }
            }
            Err(message) => println!("{message}"),
        }
    }

    let mut longest_artist_length = compositions
        .iter()
        .map(|composition| &composition.artist)
        .collect::<Vec<_>>();

    longest_artist_length.sort_by_key(|b| Reverse(b.len()));

    let mut longest_composition_length = compositions
        .iter()
        .map(|composition| &composition.composition)
        .collect::<Vec<_>>();

    longest_composition_length.sort_by_key(|b| Reverse(b.len()));

    let artist_width = longest_artist_length[0].len();
    let composition_width = longest_composition_length[0].len();

    if !compositions.is_empty() {
        println!(
            "    {: <artist_width$}    {: <composition_width$}    Compiled",
            "Artist", "Composition"
        );
        println!(
            "    {: <artist_width$}    {: <composition_width$}    ----",
            "----", "----"
        );
    }

    for composition in compositions {
        let artist = titlecase(&composition.artist);
        let title = titlecase(&composition.composition);
        let pdf = composition.pdf;
        println!("    {artist: <artist_width$}    {title: <composition_width$}    {pdf}");
    }
}
