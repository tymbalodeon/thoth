use std::convert::Infallible;
use std::fs::create_dir_all;
use std::fs::remove_dir_all;
use std::fs::rename;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::{Path, PathBuf};
use std::process::Command;

use chrono::offset::Local;
use glob::glob;
use inquire::Confirm;
use miette::{IntoDiagnostic, Result};
use watchexec::{
    action::{Action, Outcome},
    command::Command as WatchexecCommand,
    config::{InitConfig, RuntimeConfig},
    error::ReconfigError,
    event::Event,
    ErrorHook, Watchexec,
};
use watchexec_signals::Signal;

use super::compile::compile_input_file;
use super::scores::get_selected_items;
use crate::commands::create::get_file_system_name;
use crate::commands::patterns::get_score_file;
use crate::commands::scores::get_temporary_ly_file;
use crate::commands::scores::TEMPORARY_DIRECTORY;
use crate::commands::scores::{get_matching_scores, get_score_ly_file};
use crate::config::Config;

fn get_ily_files(pattern: String) -> Vec<String> {
    glob(&pattern)
        .expect("")
        .flatten()
        .map(|path| path.to_str().unwrap().to_string())
        .collect()
}

fn get_watched_files(file: &String) -> Vec<String> {
    let parent = Path::new(file).parent().unwrap();
    let score_ily_files_pattern =
        format!("{}/**/*.ily", parent.to_str().unwrap());
    let mut watched_files = get_ily_files(score_ily_files_pattern);
    let scores_directory = Config::get_scores_directory();
    let helper_ily_files_pattern =
        format!("{}/helpers/*.ily", scores_directory);
    let mut helper_ily_files = get_ily_files(helper_ily_files_pattern);

    watched_files.append(&mut helper_ily_files);
    watched_files.push(file.to_string());

    watched_files
}

fn exit_sketch(save: bool) {
    if save {
        let file =
            File::open(get_temporary_ly_file()).expect("file not found");
        let buf_reader = BufReader::new(file);
        let lines: Vec<String> =
            buf_reader.lines().collect::<Result<_, _>>().unwrap();
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
pub async fn watch(file: PathBuf, is_sketch: bool) -> Result<()> {
    let mut init_config = InitConfig::default();

    init_config.on_error(|error: ErrorHook| async move {
        eprintln!("Watchexec Runtime Error: {}", error.error);
        Ok::<(), Infallible>(())
    });

    let mut runtime_config = RuntimeConfig::default();
    let file = file.to_str().unwrap().to_string();
    let config = Config::from_config_file();

    let pdfs_directory = if is_sketch {
        TEMPORARY_DIRECTORY.to_string()
    } else {
        config.pdfs_directory
    };

    let watched_files = get_watched_files(&file);
    runtime_config.pathset(watched_files);

    runtime_config.command(WatchexecCommand::Exec {
        prog: "lilypond".to_string(),
        args: vec![
            "--include".to_string(),
            config.scores_directory,
            "--output".to_string(),
            pdfs_directory,
            file,
        ],
    });

    let watchexec = Watchexec::new(init_config, runtime_config.clone())?;

    let interrupt_action = if is_sketch {
        |action: Action| {
            print!("{esc}[2J{esc}[1;1H", esc = 27 as char);

            let response = Confirm::new("Do you want to save the sketch?")
                .with_default(false)
                .prompt();

            match response {
                Ok(true) => exit_sketch(true),
                Ok(false) => exit_sketch(false),
                Err(message) => println!("{message}"),
            }
            action.outcome(Outcome::Exit);
        }
    } else {
        |action: Action| {
            action.outcome(Outcome::Exit);
        }
    };

    runtime_config.on_action(move |action: Action| async move {
        let signals = action
            .events
            .iter()
            .flat_map(Event::signals)
            .collect::<Vec<_>>();

        if signals.iter().any(|signal| signal == &Signal::Interrupt) {
            interrupt_action(action)
        } else if action.events.iter().flat_map(Event::paths).next().is_some()
        {
            action.outcome(Outcome::if_running(
                Outcome::both(Outcome::Stop, Outcome::Start),
                Outcome::Start,
            ));
        }

        Ok::<(), ReconfigError>(())
    });

    watchexec.reconfigure(runtime_config)?;
    watchexec.main().await.into_diagnostic()??;

    Ok(())
}

pub fn open_file(file: PathBuf) {
    Command::new("open").arg(&file).output().unwrap();
}

pub fn edit_file(
    lilypond_file: String,
    is_sketch: bool,
    scores_directory: &Option<String>,
    pdfs_directory: &Option<String>,
) {
    let score_path = PathBuf::from(&lilypond_file);

    let pdfs_directory = if is_sketch {
        Some(TEMPORARY_DIRECTORY.to_string())
    } else {
        pdfs_directory.to_owned()
    };

    compile_input_file(&score_path, scores_directory, &pdfs_directory);

    let pdf_file = get_score_file(
        &score_path
            .file_stem()
            .unwrap()
            .to_str()
            .unwrap()
            .to_string(),
        ".pdf",
        scores_directory,
        &pdfs_directory,
    )
    .unwrap();

    for file in [&score_path, &pdf_file].iter() {
        open_file(file.to_path_buf());
    }

    watch(score_path, is_sketch).unwrap();
}

pub fn edit_main(
    search_term: &String,
    search_artist: bool,
    search_title: bool,
    use_all_matches: bool,
    is_sketch: bool,
    scores_directory: &Option<String>,
    pdfs_directory: &Option<String>,
) {
    let matching_scores = get_matching_scores(
        &vec![search_term.to_string()],
        search_artist,
        search_title,
        scores_directory,
    );

    if !use_all_matches && matching_scores.len() > 1 {
        if let Ok(selected_scores) = get_selected_items(matching_scores, false)
        {
            for score in selected_scores.iter() {
                let score = score.output().to_string();

                if let Some(ly_file) = get_score_ly_file(&score) {
                    edit_file(
                        ly_file,
                        is_sketch,
                        scores_directory,
                        pdfs_directory,
                    );
                }
            }
        }
    } else {
        for score in matching_scores {
            let score = score.to_str().unwrap().to_string();

            if let Some(ly_file) = get_score_ly_file(&score) {
                edit_file(
                    ly_file,
                    is_sketch,
                    scores_directory,
                    pdfs_directory,
                );
            }
        }
    }
}
