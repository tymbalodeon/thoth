use super::table::print_table;
use super::LilypondCommand;
use super::VersionStability;

use std::fmt::{Display, Formatter, Result};
use std::fs::{read_to_string, write};

use itertools::{EitherOrBoth::*, Itertools};
use owo_colors::OwoColorize;
use regex::Regex;
use shellexpand::tilde;

static GLOBAL_PATH: &str = "~/.thoth-versions";

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
    let global_path = tilde(GLOBAL_PATH).to_string();

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

fn get_versions(
    versions: &Vec<String>,
    stability: VersionStability,
) -> Vec<&String> {
    versions
        .iter()
        .filter(|version| get_version_stability(version) == stability)
        .collect()
}

fn list_remote(
    version_regex: &Option<String>,
    stability: &Option<VersionStability>,
) {
    let mut versions: Vec<String> = reqwest::blocking::get(
        "https://gitlab.com/api/v4/projects/18695663/releases",
    )
    .unwrap()
    .json::<serde_json::Value>()
    .unwrap()
    .as_array()
    .unwrap()
    .iter()
    .map(|object| {
        object
            .as_object()
            .unwrap()
            .get("tag_name")
            .unwrap()
            .to_string()
            .replace('v', "")
            .replace('"', "")
            .bold()
            .to_string()
    })
    .collect();

    if let Some(stability) = stability {
        versions = versions
            .iter()
            .filter(|version| get_version_stability(version) == *stability)
            .map(|version| version.to_string())
            .collect();
    }

    if let Some(regex) = version_regex {
        let re = Regex::new(regex).unwrap();

        versions = versions
            .iter()
            .filter(|version| re.is_match(version))
            .map(|version| version.to_string())
            .collect();
    }

    let stable = get_versions(&versions, VersionStability::Stable);
    let unstable = get_versions(&versions, VersionStability::Unstable);

    let mut titles = vec![];

    if stable.len() > 0 {
        titles.push("Stable".italic().green().to_string())
    }

    if unstable.len() > 0 {
        titles.push("Unstable".italic().yellow().to_string())
    }

    let mut rows: Vec<Vec<String>> = vec![];

    if stable.len() > 0 && unstable.len() > 0 {
        for pair in stable.iter().zip_longest(unstable.iter()) {
            match pair {
                Both(stable, unstable) => {
                    rows.push(vec![stable.to_string(), unstable.to_string()])
                }
                Left(stable) => {
                    rows.push(vec![stable.to_string(), "".to_string()])
                }
                Right(unstable) => {
                    rows.push(vec!["".to_string(), unstable.to_string()])
                }
            }
        }
    } else if stable.len() > 0 {
        for version in stable.iter() {
            rows.push(vec![version.to_string()]);
        }
    } else if unstable.len() > 0 {
        for version in unstable.iter() {
            rows.push(vec![version.to_string()]);
        }
    }

    print_table(titles, rows);
}

fn list(_version_regex: &Option<String>) {
    let contents = read_to_string(&tilde(GLOBAL_PATH).as_ref())
        .expect("Should have been able to read the file");
    println!("{contents}")
}

pub fn lilypond_main(command: &Option<LilypondCommand>) {
    if let Some(command) = command {
        match command {
            LilypondCommand::Global { version } => global(&version),
            LilypondCommand::Install { version } => install(&version),
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
