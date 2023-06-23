use std::fs::remove_file;
use std::io::Cursor;
use std::io::Write;
use std::io::{stdin, stdout};
use std::path::Path;
use std::path::PathBuf;

use glob::glob;
use skim::prelude::*;

use crate::commands::patterns::get_patterns;

fn received_confirmation() -> bool {
    print!("Are you sure you want to remove all pdfs? [y/n] ");
    stdout().flush().unwrap();
    let mut response = String::new();
    let stdin = stdin();
    stdin
        .read_line(&mut response)
        .expect("Failed to read input.");

    response.replace('\n', "").to_lowercase().eq("y")
}

fn get_matching_scores(scores: &Vec<String>) -> Vec<PathBuf> {
    let patterns = get_patterns(scores, ".pdf");
    let mut paths = vec![];

    for pattern in patterns {
        for entry in glob(&pattern).expect("Failed to read glob pattern") {
            match entry {
                Ok(path) => {
                    paths.push(path);
                }
                Err(message) => println!("{:?}", message),
            }
        }
    }

    paths
}

fn get_items(paths: Vec<PathBuf>) -> Option<Receiver<Arc<dyn SkimItem>>> {
    let paths: Vec<&str> =
        paths.iter().map(|path| path.to_str().unwrap()).collect();
    let input = paths.join("\n");
    let item_reader = SkimItemReader::default();

    Some(item_reader.of_bufread(Cursor::new(input)))
}

fn get_selected_items(scores: &Vec<String>) -> Vec<Arc<dyn SkimItem>> {
    let matching_scores = get_matching_scores(scores);
    let options = SkimOptionsBuilder::default().build().unwrap();
    let selections = get_items(matching_scores);

    Skim::run_with(&options, selections)
        .map(|out| out.selected_items)
        .unwrap_or_else(Vec::new)
}

pub fn clean_main(scores: &Vec<String>) {
    if scores.is_empty() && !received_confirmation() {
        return;
    };

    let selected_items = get_selected_items(scores);

    for item in selected_items.iter() {
        let path = item.output().to_string();
        let path = Path::new(&path);

        if let Err(error) = remove_file(path) {
            println!("Removed {}", path.display());
            println!("Failed to remove {} ({error})", path.display());
        } else {
            println!("Removed {}", path.display());
        };
    }
}
