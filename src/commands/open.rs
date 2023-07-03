use std::process::Command;

use crate::commands::scores::get_matching_scores;

use super::{
    scores::{get_selected_lilypond_files, get_selected_pdf_files},
    ScoreFileType,
};

fn open_file(file_path: &String) {
    Command::new("open").arg(file_path).output().unwrap();
    println!("Opened {file_path}");
}

pub fn open_main(
    search_terms: &Vec<String>,
    _artist: &bool,
    _title: &bool,
    file_type: &Option<ScoreFileType>,
    scores_directory: &Option<String>,
    pdfs_directory: &Option<String>,
) {
    let matching_files = match file_type {
        Some(ScoreFileType::Both) => {
            let mut ly_files = get_selected_lilypond_files(
                search_terms,
                scores_directory,
                pdfs_directory,
            );

            let file_stems: Vec<String> = ly_files
                .iter()
                .map(|file| {
                    file.file_stem().unwrap().to_str().unwrap().to_string()
                })
                .collect();

            let pdfs = get_matching_scores(
                &file_stems,
                ".pdf",
                &None,
                pdfs_directory,
            );

            ly_files.extend(pdfs);

            ly_files
        }
        Some(ScoreFileType::Lilypond) => get_selected_lilypond_files(
            search_terms,
            scores_directory,
            pdfs_directory,
        ),
        Some(ScoreFileType::Pdf) | None => get_selected_pdf_files(
            search_terms,
            scores_directory,
            pdfs_directory,
        ),
    };

    for file in matching_files {
        let path = file.to_str().unwrap().to_string();
        open_file(&path);
    }
}
