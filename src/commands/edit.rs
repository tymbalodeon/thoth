use std::fs::create_dir_all;
use std::fs::remove_dir_all;
use std::fs::rename;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::{Path, PathBuf};
use std::process::Command;

use chrono::offset::Local;
use color_eyre::eyre;
use glob::glob;
use watchexec::Watchexec;
use watchexec_signals::Signal;

use super::compile::compile_input_file;
use super::scores::get_selected_items;
use crate::commands::create::get_file_system_name;
use crate::commands::patterns::get_score_file;
use crate::commands::scores::get_temporary_ly_file;
use crate::commands::scores::TEMPORARY_DIRECTORY;
use crate::commands::scores::{get_score_ly_file, search};
use crate::config::Config;

fn get_ily_files(pattern: &str) -> Vec<String> {
    glob(pattern)
        .expect("")
        .flatten()
        .map(|path| {
            path.to_str()
                .expect("Failed to parse .ily file path.")
                .to_string()
        })
        .collect()
}

fn get_watched_files(file: &String) -> Vec<String> {
    let parent = Path::new(file)
        .parent()
        .expect("Failed to get watched file's parent path.");
    let score_ily_files_pattern = format!(
        "{}/**/*.ily",
        parent.to_str().expect("Failed to get included .ily files.")
    );
    let mut watched_files = get_ily_files(&score_ily_files_pattern);
    let scores_directory = Config::get_scores_directory();
    let helper_ily_files_pattern = format!("{scores_directory}/helpers/*.ily");
    let mut helper_ily_files = get_ily_files(&helper_ily_files_pattern);

    watched_files.append(&mut helper_ily_files);
    watched_files.push(file.to_string());

    watched_files
}

fn exit_sketch(save: bool) {
    if save {
        let file =
            File::open(get_temporary_ly_file()).expect("file not found");
        let buf_reader = BufReader::new(file);
        let lines: Vec<String> = buf_reader
            .lines()
            .collect::<eyre::Result<_, _>>()
            .expect("Failed to read file.");
        let config = Config::from_config_file();
        let mut composer = config.composer;
        let mut title = "Sketch".to_string();

        for line in lines {
            let composer_line = "  composer = ";
            let title_line = "  title = ";

            if line.starts_with(composer_line) {
                composer = get_file_system_name(
                    &line.replace(composer_line, "").replace('"', ""),
                );
            }

            if line.starts_with(title_line) {
                title = get_file_system_name(
                    &line.replace(title_line, "").replace('"', ""),
                );
            }
        }

        let sketches_directory =
            format!("{}/scores/{composer}/sketches", config.scores_directory);

        if create_dir_all(&sketches_directory).is_ok() {
            let saved_file_path = format!(
                "{sketches_directory}/{}-{title}.ly",
                Local::now().format("%Y-%m-%d_%H:%M:%S")
            );

            if let Err(message) =
                rename(get_temporary_ly_file(), saved_file_path)
            {
                println!("{message}");
            }
        }
    }

    let _ = remove_dir_all(TEMPORARY_DIRECTORY);
}

#[tokio::main]
pub async fn watch(file: &Path, _is_sketch: bool) -> eyre::Result<()> {
    color_eyre::install()?;

    let file = file
        .to_str()
        .expect("Faild to parse file name.")
        .to_string();

    let watched_files = get_watched_files(&file);

    let watchexec = Watchexec::new(|mut action| {
        let config = Config::from_config_file();

        for event in action.events.iter() {
            for path in event.paths() {
                match path.0.parent() {
                    Some(directory) => {
                        let stuff = glob(&format!(
                            "{}/*.ly",
                            directory.to_str().unwrap()
                        ));

                        for entry in stuff.expect("").flatten() {
                            let lilypond_file =
                                entry.to_str().unwrap().to_string();
                            let scores_directory =
                                config.scores_directory.clone();
                            let pdfs_directory = config.pdfs_directory.clone();

                            let output = Command::new("lilypond")
                                .args([
                                    "--include".to_string(),
                                    scores_directory,
                                    "--output".to_string(),
                                    pdfs_directory,
                                    lilypond_file,
                                ])
                                .output()
                                .unwrap();

                            let out =
                                String::from_utf8(output.stdout).unwrap();
                            let err =
                                String::from_utf8(output.stderr).unwrap();

                            println!("{out}");
                            eprintln!("{err}");
                        }
                    }

                    None => {
                        eprintln!("Error: Failed to locate parent folder.");
                    }
                }
            }
        }

        if action.signals().any(|signal| signal == Signal::Interrupt) {
            action.quit();
        }

        action
    })?;

    watchexec.config.pathset(watched_files);
    let _ = watchexec.main().await?;

    Ok(())
}

pub fn open_file(file: &PathBuf) {
    Command::new("open")
        .arg(file)
        .output()
        .unwrap_or_else(|err| {
            panic!("{}", format!("Failed to open file: {err}"))
        });
}

pub fn edit_file(
    lilypond_file: &str,
    is_sketch: bool,
    lilypond_version: &Option<String>,
    scores_directory: &Option<String>,
    pdfs_directory: &Option<String>,
) {
    let score_path = PathBuf::from(&lilypond_file);

    let pdfs_directory = if is_sketch {
        Some(TEMPORARY_DIRECTORY.to_string())
    } else {
        pdfs_directory.to_owned()
    };

    compile_input_file(
        &score_path,
        lilypond_version,
        scores_directory,
        &pdfs_directory,
        false,
    );

    let err = "Failed to get score pdf file.";
    let pdf_file = get_score_file(
        &score_path
            .file_stem()
            .expect(err)
            .to_str()
            .expect(err)
            .to_string(),
        ".pdf",
        scores_directory,
        &pdfs_directory,
    )
    .expect(err);

    for file in [&score_path, &pdf_file] {
        open_file(file);
    }

    watch(&score_path, is_sketch).expect("Failed to open score for editing.");
}

pub fn main(
    search_term: &String,
    search_artist: bool,
    search_title: bool,
    use_all_matches: bool,
    is_sketch: bool,
    lilypond_version: &Option<String>,
    scores_directory: &Option<String>,
    pdfs_directory: &Option<String>,
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
                    edit_file(
                        &ly_file,
                        is_sketch,
                        lilypond_version,
                        scores_directory,
                        pdfs_directory,
                    );
                }
            }
        }
    } else {
        for score in matching_scores {
            let score =
                score.to_str().expect("Failed to parse score.").to_string();

            if let Some(ly_file) = get_score_ly_file(&score) {
                edit_file(
                    &ly_file,
                    is_sketch,
                    lilypond_version,
                    scores_directory,
                    pdfs_directory,
                );
            }
        }
    }
}
