use std::path::PathBuf;

use super::LilypondCommand;
use shellexpand::tilde;

static GLOBAL_PATH: &str = "~/.thoth-versions";

fn global(version: &Option<String>) {
    if let Some(value) = version {
        println!("{value:?}");
    } else {
        let global_path = tilde(GLOBAL_PATH);
        if PathBuf::from(global_path.to_string()).exists() {
            println!("{global_path}");
        } else {
            println!("No global lilypond version set.");
        }
    }
}

pub fn lilypond_main(command: &Option<LilypondCommand>) {
    if let Some(command) = command {
        match command {
            LilypondCommand::Global { version } => global(&version),
            LilypondCommand::Install { version } => println!("{version:?}"),
            LilypondCommand::List { version_regex } => {
                println!("{version_regex:?}")
            }
        }
    } else {
        println!("{command:?}")
    }
}
