use std::convert::Infallible;
use std::fs::remove_dir_all;
use std::path::{Path, PathBuf};
use std::process::Command;

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

use crate::commands::patterns::get_score_file;
use crate::commands::scores::TEMPORARY_DIRECTORY;
use crate::commands::scores::{get_matching_scores, get_score_ly_file};
use crate::config::Config;

use super::compile::compile_input_file;
use super::scores::get_selected_items;

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

#[tokio::main]
pub async fn watch(file: PathBuf, is_sketch: &bool) -> Result<()> {
    let mut init_config = InitConfig::default();

    init_config.on_error(|error: ErrorHook| async move {
        eprintln!("Watchexec Runtime Error: {}", error.error);
        Ok::<(), Infallible>(())
    });

    let mut runtime_config = RuntimeConfig::default();
    let file = file.to_str().unwrap().to_string();
    let config = Config::from_config_file();

    let pdfs_directory = if *is_sketch {
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

    let interrupt_action = if *is_sketch {
        |action: Action| {
            print!("{esc}[2J{esc}[1;1H", esc = 27 as char);

            let ans = Confirm::new("Do you want to save sketch?")
                .with_default(false)
                .prompt();

            match ans {
                Ok(true) => println!("Saving!"),
                Ok(false) => {
                    let _ = remove_dir_all(TEMPORARY_DIRECTORY);
                }
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
    is_sketch: &bool,
    scores_directory: &Option<String>,
    pdfs_directory: &Option<String>,
) {
    let score_path = PathBuf::from(&lilypond_file);

    let scores_directory = if *is_sketch {
        Some(TEMPORARY_DIRECTORY.to_string())
    } else {
        scores_directory.to_owned()
    };

    let pdfs_directory = if *is_sketch {
        Some(TEMPORARY_DIRECTORY.to_string())
    } else {
        pdfs_directory.to_owned()
    };

    compile_input_file(&score_path, &scores_directory, &pdfs_directory);

    let pdf_file = get_score_file(
        &score_path
            .file_stem()
            .unwrap()
            .to_str()
            .unwrap()
            .to_string(),
        ".pdf",
        &scores_directory,
        &pdfs_directory,
    )
    .unwrap();

    for file in vec![&score_path, &pdf_file].iter() {
        open_file(file.to_path_buf());
    }

    watch(score_path, is_sketch).unwrap();
}

pub fn edit_main(
    search_term: &String,
    search_artist: &bool,
    search_title: &bool,
    use_all_matches: &bool,
    is_sketch: &bool,
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
