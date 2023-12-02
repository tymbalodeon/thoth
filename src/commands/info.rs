use std::{
    fs::File,
    io::{BufRead, BufReader},
};

use bat::{PagingMode, PrettyPrinter};
use convert_case::{Case::Title, Casing};

use super::helpers::pushln;
use super::scores::{get_score_ly_file, get_selected_items, search};

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
    let mut lines = String::new();

    if let Some(lilypond_version) = composition_metadata.lilypond_version {
        pushln(
            &mut lines,
            format!("LilyPond version = \"{lilypond_version}\""),
        );
    }

    if let Some(title) = composition_metadata.title {
        pushln(&mut lines, format!("Title = \"{title}\""));
    }

    if let Some(subtitle) = composition_metadata.subtitle {
        pushln(&mut lines, format!("Subtitle = \"{subtitle}\""));
    }

    if let Some(composer) = composition_metadata.composer {
        pushln(&mut lines, format!("Composer = \"{composer}\""));
    }

    if let Some(arranger) = composition_metadata.arranger {
        pushln(&mut lines, format!("Arranger = \"{arranger}\""));
    }

    if let Some(key) = composition_metadata.key {
        pushln(&mut lines, format!("Key = \"{key}\""));
    } else {
        pushln(&mut lines, "Key = C Major".to_string());
    }

    if let Some(time) = composition_metadata.time {
        pushln(&mut lines, format!("Time Signature = \"{time}\""));
    } else {
        pushln(&mut lines, "Time = 4/4".to_string());
    }

    let mut instruments = composition_metadata.instruments;

    if !instruments.is_empty() {
        instruments.sort();
        pushln(&mut lines, "Instrumentation = [ ".to_string());

        for instrument in instruments {
            pushln(&mut lines, format!("    \"{instrument}\""));
        }

        pushln(&mut lines, "]".to_string());
    }

    PrettyPrinter::new()
        .input_from_bytes(lines.as_bytes())
        .language("toml")
        .theme("gruvbox-dark")
        .paging_mode(PagingMode::QuitIfOneScreen)
        .print()
        .unwrap();
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

pub fn main(
    search_term: &String,
    search_artist: bool,
    search_title: bool,
    use_all_matches: bool,
    scores_directory: &Option<String>,
) {
    let matching_scores = search(
        &vec![search_term.to_string()],
        search_artist,
        search_title,
        scores_directory,
    );

    if !use_all_matches && matching_scores.len() > 1 {
        if let Ok(selected_scores) =
            get_selected_items(&matching_scores, false)
        {
            for score in &selected_scores {
                let score = score.output().to_string();

                if let Some(ly_file) = get_score_ly_file(&score) {
                    display_score_info(&ly_file);
                }
            }
        }
    } else {
        for score in matching_scores {
            let score = score.to_str().unwrap().to_string();

            if let Some(ly_file) = get_score_ly_file(&score) {
                display_score_info(&ly_file);
            }
        }
    }
}
