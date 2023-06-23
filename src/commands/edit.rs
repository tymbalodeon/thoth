use std::convert::Infallible;
use std::path::{Path, PathBuf};
use std::process::Command;

use glob::glob;
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
use crate::{commands::compile::compile_main, config::Config};

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
async fn watch(file: PathBuf) -> Result<()> {
    let mut init_config = InitConfig::default();

    init_config.on_error(|error: ErrorHook| async move {
        eprintln!("Watchexec Runtime Error: {}", error.error);
        Ok::<(), Infallible>(())
    });

    let mut runtime_config = RuntimeConfig::default();
    let file = file.to_str().unwrap().to_string();
    let config = Config::from_config_file();
    let watched_files = get_watched_files(&file);
    runtime_config.pathset(watched_files);

    runtime_config.command(WatchexecCommand::Exec {
        prog: "lilypond".to_string(),
        args: vec![
            "--include".to_string(),
            config.scores_directory,
            "--output".to_string(),
            config.pdfs_directory,
            file,
        ],
    });

    let watchexec = Watchexec::new(init_config, runtime_config.clone())?;

    runtime_config.on_action(move |action: Action| async move {
        let signals = action
            .events
            .iter()
            .flat_map(Event::signals)
            .collect::<Vec<_>>();

        if signals.iter().any(|signal| signal == &Signal::Interrupt) {
            action.outcome(Outcome::Exit);
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

pub fn edit_main(score: &String, pdfs_directory: &Option<String>) {
    compile_main(&vec![score.to_string()], pdfs_directory);
    let lilypond_file = get_score_file(score, ".ly");
    let pdf_file = get_score_file(score, ".pdf");

    for file in [&lilypond_file, &pdf_file].into_iter().flatten() {
        open_file(file.to_path_buf());
    }

    if let Some(path) = lilypond_file {
        watch(path).unwrap();
    }
}
