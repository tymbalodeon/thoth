pub mod list_remote;
use self::list_remote::list_remote;

use super::{LilypondCommand, VersionStability};

use std::fmt;
use std::fmt::{Display, Formatter};
use std::fs::{read_to_string, write};

use shellexpand::tilde;

static GLOBAL_PATH: &str = "~/.thoth-versions";

impl Display for VersionStability {
    fn fmt(&self, formatter: &mut Formatter) -> fmt::Result {
        let display = format!("{self:?}").to_lowercase();
        write!(formatter, "{display}")
    }
}

fn is_latest_version(version: &str) -> bool {
    version.chars().all(|char| !char.is_numeric())
}

fn get_version_stability(
    version: &str,
) -> Result<VersionStability, &'static str> {
    if is_latest_version(version) {
        match version {
            "latest-stable" => Ok(VersionStability::Stable),
            "latest-unstable" => Ok(VersionStability::Unstable),
            _ => Err("invalid version specifier"),
        }
    } else {
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
            Ok(VersionStability::Stable)
        } else {
            Ok(VersionStability::Unstable)
        }
    }
}

fn print_version(version: &String) {
    match get_version_stability(version) {
        Ok(stability) => {
            let value = if is_latest_version(version) {
                "latest"
            } else {
                version
            };
            let formatted_version = format!("{value} ({stability})");
            println!("{formatted_version}");
        }
        Err(err) => println!("{err}"),
    }
}

fn global(version: &Option<String>) -> Result<(), &'static str> {
    let global_path = tilde(GLOBAL_PATH).to_string();

    if let Some(value) = version {
        let _ = write(global_path, value);
        print_version(value);
    } else if let Ok(version) = read_to_string(&global_path) {
        print_version(&version);
    } else {
        println!("No global lilypond version set.");
    };

    Ok(())
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
        .filter(|version| get_version_stability(version).unwrap() == stability)
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
            LilypondCommand::Global { version } => global(version).unwrap(),
            LilypondCommand::Install { version } => install(version),
            LilypondCommand::List { version_regex } => list(version_regex),
            LilypondCommand::ListRemote {
                version_regex,
                stability,
            } => list_remote(version_regex, stability),
        }
    } else {
        list(&None);
    }
}
