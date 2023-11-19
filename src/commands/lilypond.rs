pub mod list_remote;
use self::list_remote::list_remote;

use super::{LilypondCommand, VersionStability};

use std::fmt::{Display, Formatter, Result};
use std::fs::{read_to_string, write};

use shellexpand::tilde;

static GLOBAL_PATH: &str = "~/.thoth-versions";

impl Display for VersionStability {
    fn fmt(&self, formatter: &mut Formatter) -> Result {
        let display = format!("{self:?}").to_lowercase();
        write!(formatter, "{display}")
    }
}

fn get_version_stability(version: &str) -> VersionStability {
    let minor_version = version
        .split('.')
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
    let global_path = tilde(GLOBAL_PATH).to_string();

    if let Some(value) = version {
        let stability = get_version_stability(value);

        println!("{stability} {value}");

        let _ = write(global_path, value);
    } else if let Ok(version) = read_to_string(&global_path) {
        println!("lilypond {version}");
    } else {
        println!("No global lilypond version set.");
    };
}

fn install(version: &Option<String>) {
    println!("{version:?}");
}

fn get_versions(
    versions: &[String],
    stability: VersionStability,
) -> Vec<&String> {
    versions
        .iter()
        .filter(|version| get_version_stability(version) == stability)
        .collect()
}

fn list(_version_regex: &Option<String>) {
    let contents = read_to_string(tilde(GLOBAL_PATH).as_ref())
        .expect("Should have been able to read the file");
    println!("{contents}")
}

pub fn lilypond_main(command: &Option<LilypondCommand>) {
    if let Some(command) = command {
        match command {
            LilypondCommand::Global { version } => global(version),
            LilypondCommand::Install { version } => install(version),
            LilypondCommand::List { version_regex } => list(version_regex),
            LilypondCommand::ListRemote {
                version_regex,
                stability,
            } => list_remote(version_regex, stability),
        }
    } else {
        println!("{command:?}")
    }
}
