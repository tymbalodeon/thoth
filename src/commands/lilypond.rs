use shellexpand::tilde;
use std::fmt::{Display, Formatter, Result};
use std::fs::{read_to_string, write};

use super::LilypondCommand;

#[derive(Debug)]
enum VersionStability {
    Stable,
    Unstable,
}

impl Display for VersionStability {
    fn fmt(&self, formatter: &mut Formatter) -> Result {
        let display = format!("{self:?}").to_lowercase();
        write!(formatter, "{display}")
    }
}

fn get_version_stability(version: &String) -> VersionStability {
    let minor_version = version
        .split(".")
        .enumerate()
        .filter(|(index, _)| index == &1usize)
        .map(|(_, value)| value)
        .next()
        .unwrap()
        .parse::<i32>()
        .unwrap();

    if minor_version % 2 == 0 {
        VersionStability::Stable
    } else {
        VersionStability::Unstable
    }
}

fn global(version: &Option<String>) {
    let global_path = tilde("~/.thoth-versions").to_string();

    if let Some(value) = version {
        let stability = get_version_stability(value);

        println!("{stability} {value}");

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
