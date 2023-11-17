use shellexpand::tilde;
use std::fs::{read_to_string, write};

use super::LilypondCommand;

fn global(version: &Option<String>) {
    let global_path = tilde("~/.thoth-versions").to_string();

    if let Some(value) = version {
        let _ = write(global_path, value);
    } else {
        if let Ok(version) = read_to_string(&global_path) {
            println!("lilypond {version}");
        } else {
            println!("No global lilypond version set.");
        };
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
