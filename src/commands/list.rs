use crate::{commands::patterns::get_patterns, config::Config};
use clap::builder::OsStringValueParser;
use glob::glob;
use std::{
    ffi::OsString,
    fs::{read_dir, DirEntry},
};

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

pub fn list_scores(scores: &Vec<String>) {
    let config = Config::new();
    let scores_directory = config.scores_directory();
    let score_files = format!("{scores_directory}/scores");

    for entry in read_dir(score_files).unwrap() {
        match entry {
            Ok(path) => {
                let mut lines: Vec<Vec<String>> = vec![];
                let artist = get_display(&path);
                // println!("{artist:?}");

                for entry in read_dir(path.path()).unwrap() {
                    let entry = entry.unwrap();
                    let name = get_display(&entry);
                    lines.push(vec![artist.clone(), name]);
                }

                for line in lines {
                    println!("    {: <21}    {}", line[0], line[1]);
                }

                // match score_name.to_str() {
                //     Some(test) => {
                //         println!("{}", test.replace('-', " "));
                //         list_pdfs(&vec![test.to_string()]);
                //     }
                //     None => println!("NO"),
                // }
            }
            Err(message) => println!("{message}"),
        }
    }
}
