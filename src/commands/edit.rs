use super::patterns::get_score_file;
use crate::{commands::compile::compile_main, config::Config};
use miette::{IntoDiagnostic, Result};
use std::convert::Infallible;
use std::path::PathBuf;
use std::process::Command;
use watchexec::{
    action::{Action, Outcome},
    command::Command as WatchexecCommand,
    config::{InitConfig, RuntimeConfig},
    error::ReconfigError,
    event::Event,
    ErrorHook, Watchexec,
};
use watchexec_signals::Signal;

#[tokio::main]
async fn watch(file: PathBuf) -> Result<()> {
    let mut init_config = InitConfig::default();

    init_config.on_error(|error: ErrorHook| async move {
        eprintln!("Watchexec Runtime Error: {}", error.error);
        Ok::<(), Infallible>(())
    });

    let mut runtime_config = RuntimeConfig::default();
    let file = file.to_str().unwrap().to_string();
    runtime_config.pathset([&file]);

    let config = Config::from_config_file();

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

pub fn edit_main(score: &String) {
    compile_main(&vec![score.to_string()]);
    let lilypond_file = get_score_file(score, ".ly");
    let pdf_file = get_score_file(score, ".pdf");

    for file in [&lilypond_file, &pdf_file].into_iter().flatten() {
        open_file(file.to_path_buf());
    }

    if let Some(path) = lilypond_file {
        watch(path).unwrap();
    }
}
