pub mod global;
pub mod list_remote;

use self::global::global;
use self::list_remote::{list_remote, LilypondReleases};

use super::{LilypondCommand, VersionStability};

use std::fmt;
use std::fmt::{Display, Formatter};
use std::fs::read_to_string;

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

pub fn get_releases() -> Vec<String> {
    let mut releases = vec![];

    for release in LilypondReleases::get().unwrap() {
        releases.push(release.unwrap().tag_name.to_string());
    }

    releases
        .iter()
        .map(|release| release.replace(['v', '"'], "").replace("release/", ""))
        .collect()
}

fn install(version: &Option<String>) {
    println!("{version:?}");
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
