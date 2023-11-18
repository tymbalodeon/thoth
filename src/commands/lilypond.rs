use std::fmt::{Display, Formatter, Result};
use std::fs::{read_to_string, write};

use shellexpand::tilde;

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

fn install(version: &Option<String>) {
    println!("{version:?}");
}

fn list(_version_regex: &Option<String>) {
    let response = reqwest::blocking::get(
        "https://gitlab.com/api/v4/projects/18695663/releases",
    )
    .unwrap()
    .json::<serde_json::Value>()
    .unwrap();

    println!("{response}");
}

pub fn lilypond_main(command: &Option<LilypondCommand>) {
    if let Some(command) = command {
        match command {
            LilypondCommand::Global { version } => global(&version),
            LilypondCommand::Install { version } => install(&version),
            LilypondCommand::List { version_regex } => list(version_regex),
        }
    } else {
        println!("{command:?}")
    }
}
