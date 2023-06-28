use convert_case::{Case::Title, Casing};

use super::scores::{get_matching_scores, get_selected_items};
use std::{
    fs::File,
    io::{BufRead, BufReader},
};

struct CompositionMetadata {
    lilypond_version: Option<String>,
    title: Option<String>,
    subtitle: Option<String>,
    composer: Option<String>,
    arranger: Option<String>,
    key: Option<String>,
    time: Option<String>,
    instruments: Vec<String>,
}

fn print_info(composition_metadata: CompositionMetadata) {
    if let Some(lilypond_version) = composition_metadata.lilypond_version {
        println!("LilyPond version = {lilypond_version}");
    }

    if let Some(title) = composition_metadata.title {
        println!("Title = {title}");
    }

    if let Some(subtitle) = composition_metadata.subtitle {
        println!("Subtitle = {subtitle}");
    }

    if let Some(composer) = composition_metadata.composer {
        println!("Composer = {composer}");
    }

    if let Some(arranger) = composition_metadata.arranger {
        println!("Arranger = {arranger}");
    }

    if let Some(key) = composition_metadata.key {
        println!("Key = {key}");
    } else {
        println!("Key = C Major");
    }

    if let Some(time) = composition_metadata.time {
        println!("Time Signature = {time}");
    } else {
        println!("Time = 4/4");
    }

    let mut instruments = composition_metadata.instruments;

    if !instruments.is_empty() {
        instruments.sort();
        println!("Instrumentation:");

        for instrument in instruments {
            println!("    - {instrument}");
        }
    }
}

fn display_score_info(score: &String) {
    let file = File::open(score).expect("file not found");
    let buf_reader = BufReader::new(file);
    let lines: Vec<String> =
        buf_reader.lines().collect::<Result<_, _>>().unwrap();

    let mut lilypond_version: Option<String> = None;
    let mut title: Option<String> = None;
    let mut subtitle: Option<String> = None;
    let mut composer: Option<String> = None;
    let mut arranger: Option<String> = None;
    let mut key: Option<String> = None;
    let mut time: Option<String> = None;
    let mut instruments: Vec<String> = vec![];

    let lilypond_version_line = "\\version ";
    let title_line = "  title = ";
    let subtitle_line = "  subtitle = ";
    let composer_line = "  composer = ";
    let arranger_line = "  arranger = ";
    let instrument_line = "instrumentName = ";
    let key_line = "  \\key ";
    let time_line = "  \\time ";

    for line in lines {
        if line.starts_with(lilypond_version_line) {
            let line =
                line.replace(lilypond_version_line, "").replace('"', "");
            lilypond_version = Some(line);
        }

        if line.starts_with(title_line) {
            let line = line.replace(title_line, "").replace('"', "");
            title = Some(line);
        }

        if line.starts_with(subtitle_line) {
            let line = line.replace(subtitle_line, "").replace('"', "");
            subtitle = Some(line);
        }

        if line.starts_with(composer_line) {
            let line = line.replace(composer_line, "").replace('"', "");
            composer = Some(line);
        }

        if line.starts_with(arranger_line) {
            let line = line.replace(arranger_line, "").replace('"', "");
            arranger = Some(line);
        }

        if line.starts_with(key_line) {
            let line = line
                .replace(key_line, "")
                .replace('\\', "")
                .to_case(Title)
                .replace('s', "♯")
                .replace('f', "♭");
            key = Some(line);
        }

        if line.starts_with(time_line) {
            let line = line.replace(time_line, "").replace('\\', "");
            time = Some(line);
        }

        if line.contains(instrument_line) {
            let line =
                line.replace(instrument_line, "").replace(['"', ' '], "");
            instruments.push(line);
        }
    }

    let metadata = CompositionMetadata {
        lilypond_version,
        title,
        subtitle,
        composer,
        arranger,
        key,
        time,
        instruments,
    };

    print_info(metadata);
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
