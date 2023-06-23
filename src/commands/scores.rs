use std::io::Cursor;
use std::path::PathBuf;

use glob::glob;
use skim::prelude::*;

use super::patterns::get_patterns;

pub fn get_matching_scores(
    scores: &Vec<String>,
    extension: &str,
    pdfs_directory: &Option<String>,
) -> Vec<PathBuf> {
    let patterns = get_patterns(scores, extension, pdfs_directory);
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

pub fn get_selected_items(
    matching_scores: Vec<PathBuf>,
) -> Vec<Arc<dyn SkimItem>> {
    let options = SkimOptionsBuilder::default().multi(true).build().unwrap();
    let selections = get_items(matching_scores);

    Skim::run_with(&options, selections)
        .map(|out| out.selected_items)
        .unwrap_or_else(Vec::new)
}
