use std::process::Command;

use crate::commands::scores::{get_matching_scores, get_selected_items};

fn open_file(file_path: &String) {
    Command::new("open").arg(file_path).output().unwrap();
    println!("Opened {file_path}");
}

pub fn open_main(scores: &Vec<String>, pdfs_directory: &Option<String>) {
    let matching_scores =
        get_matching_scores(scores, ".pdf", &None, pdfs_directory);

    if matching_scores.len() > 1 {
        let selected_items = get_selected_items(matching_scores, true);

        for item in selected_items.iter() {
            let path = item.output().to_string();
            open_file(&path);
        }
    } else {
        for score in matching_scores {
            let path = score.to_str().unwrap().to_string();
            open_file(&path);
        }
    }
}
