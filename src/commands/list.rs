use crate::{commands::patterns::get_patterns, config::Config};
use clap::builder::OsStringValueParser;
use glob::glob;
use std::{
    ffi::OsString,
    fs::{read_dir, DirEntry},
};
use titlecase::titlecase;

pub fn list_pdfs(scores: &Vec<String>) {
    let patterns = get_patterns(scores, ".pdf");

    for pattern in patterns {
        for entry in glob(&pattern).expect("Failed to read glob pattern") {
            match entry {
                Ok(path) => println!("{}", path.display()),
                Err(message) => println!("{:?}", message),
            }
        }
    }
}

fn get_display(path: &DirEntry) -> String {
    let artist = String::from(path.file_name().to_str().unwrap());
    artist.replace("-", " ")
}

struct Composition {
    artist: String,
    composition: String,
}

pub fn list_scores(search_terms: &Vec<String>) {
    let config = Config::new();
    let scores_directory = config.scores_directory();
    let score_files = format!("{scores_directory}/scores");
    let mut compositions: Vec<Composition> = vec![];

    for entry in read_dir(score_files).unwrap() {
        match entry {
            Ok(path) => {
                let artist = get_display(&path);

                for entry in read_dir(path.path()).unwrap() {
                    let entry = entry.unwrap();
                    let composition = get_display(&entry);

                    let mut is_match = true;

                    if search_terms.len() > 0 {
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
                        let path =
                            String::from(entry.path().to_str().unwrap());

                        // for file in glob(&format!("{path}/*.ly"))
                        //     .expect("Failed to read glob pattern")
                        // {
                        //     match file {
                        //         Ok(path) => println!("{}", path.display()),
                        //         Err(message) => println!("{:?}", message),
                        //     }
                        // }

                        compositions.push(Composition {
                            artist: artist.clone(),
                            composition,
                        });
                    }
                }
            }
            Err(message) => println!("{message}"),
        }
    }

    if compositions.len() > 0 {
        println!("    {: <21}    {}", "Artist", "Composition");
        println!("    {: <21}    {}", "----", "----");
    }

    for composition in compositions {
        let artist = titlecase(&composition.artist);
        let composition = titlecase(&composition.composition);
        println!("    {artist: <21}    {composition}");
    }
}
