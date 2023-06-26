use super::scores::{get_matching_scores, get_selected_items};
use std::{
    fs::File,
    io::{BufRead, BufReader},
};

fn display_score_info(score: &String) {
    let file = File::open(score).expect("file not found");
    let buf_reader = BufReader::new(file);
    let lines: Vec<String> =
        buf_reader.lines().collect::<Result<_, _>>().unwrap();

    for line in lines {
        let mut title: Option<String> = None;
        let mut composer: Option<String> = None;
        let mut arranger: Option<String> = None;

        let title_line = "  title = ";
        let composer_line = "  composer = ";
        let arranger_line = "  arranger = ";

        if line.starts_with(title_line) {
            let line = line.replace(title_line, "").replace('"', "");
            title = Some(line);
        }

        if line.starts_with(composer_line) {
            let line = line.replace(composer_line, "").replace('"', "");
            composer = Some(line);
        }

        if line.starts_with(arranger_line) {
            let line = line.replace(arranger_line, "").replace('"', "");
            arranger = Some(line);
        }

        if let Some(title) = title {
            println!("Title = {title}");
        }

        if let Some(composer) = composer {
            println!("Composer = {composer}");
        }

        if let Some(arranger) = arranger {
            println!("Arranger = {arranger}");
        }
    }
}

pub fn info_main(
    score: &String,
    scores_directory: &Option<String>,
    pdfs_directory: &Option<String>,
) {
    let matching_scores = get_matching_scores(
        &vec![score.to_string()],
        ".ly",
        scores_directory,
        pdfs_directory,
    );

    if matching_scores.len() > 1 {
        let selected_scores = get_selected_items(matching_scores, false);

        for score in selected_scores.iter() {
            let score = score.output().to_string();
            display_score_info(&score);
        }
    } else {
        for score in matching_scores {
            let score = score.to_str().unwrap().to_string();
            display_score_info(&score);
        }
    }
}
