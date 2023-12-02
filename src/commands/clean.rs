use std::fs::remove_file;
use std::path::Path;

use super::scores::{get_found_pdfs, get_selected_items};
use crate::commands::received_confirmation;

fn remove_score(path: &Path) {
    if let Err(message) = remove_file(path) {
        let display = path.display();
        println!("Removed {display}");
        println!("Failed to remove {display} ({message})");
    } else {
        println!("Removed {}", path.display());
    };
}

pub fn clean_main(
    search_terms: &Vec<String>,
    search_artist: bool,
    search_title: bool,
    use_all_matches: bool,
    scores_directory: &Option<String>,
    pdfs_directory: &Option<String>,
) {
    if search_terms.is_empty()
        && !received_confirmation(
            "Are you sure you want to remove all pdfs? [y/n]",
        )
    {
        return;
    };

    let matching_pdfs = get_found_pdfs(
        search_terms,
        search_artist,
        search_title,
        scores_directory,
        pdfs_directory,
    );

    if !use_all_matches && matching_pdfs.len() > 1 {
        if let Ok(selected_items) = get_selected_items(matching_pdfs, true) {
            for item in selected_items.iter() {
                let path = item.output().to_string();
                let path = Path::new(&path);
                remove_score(path);
            }
        }
    } else {
        for score in matching_pdfs {
            remove_score(score.as_path());
        }
    }
}
