use std::fs::remove_file;
use std::io::Write;
use std::io::{stdin, stdout};
use std::path::Path;

use crate::commands::scores::{get_matching_scores, get_selected_items};

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

fn remove_score(path: &Path) {
    if let Err(message) = remove_file(path) {
        let display = path.display();
        println!("Removed {display}");
        println!("Failed to remove {display} ({message})");
    } else {
        println!("Removed {}", path.display());
    };
}

pub fn clean_main(scores: &Vec<String>) {
    if scores.is_empty() && !received_confirmation() {
        return;
    };

    let matching_scores = get_matching_scores(scores, ".pdf");

    if matching_scores.len() > 1 {
        let selected_items = get_selected_items(matching_scores);

        for item in selected_items.iter() {
            let path = item.output().to_string();
            let path = Path::new(&path);
            remove_score(path);
        }
    } else {
        for score in matching_scores {
            remove_score(score.as_path());
        }
    }
}
