pub mod global;
pub mod install;
pub mod list;
pub mod list_remote;

use self::global::global;
use self::install::install;
use self::list::list;
use self::list_remote::{list_remote, LilypondReleases};

use super::{LilypondCommand, VersionStability};

use std::fmt;
use std::fmt::{Display, Formatter};

static GLOBAL_PATH: &str = "~/.thoth-versions";
static INSTALL_PATH: &str = "~/.local/share/thoth";

impl Display for VersionStability {
    fn fmt(&self, formatter: &mut Formatter) -> fmt::Result {
        let display = format!("{self:?}").to_lowercase();
        write!(formatter, "{display}")
    }
}

pub fn is_valid_version(version: &String) -> bool {
    let mut versions =
        vec!["latest-stable".to_string(), "latest-unstable".to_string()];
    versions.append(&mut get_versions());

    versions.contains(version)
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

pub fn get_tag_names() -> Vec<String> {
    let mut releases = vec![];

    for release in LilypondReleases::get().unwrap() {
        releases.push(release.unwrap().tag_name.to_string());
    }

    releases
}

pub fn get_versions() -> Vec<String> {
    get_tag_names()
        .iter()
        .map(|release| release.replace(['v', '"'], "").replace("release/", ""))
        .collect()
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
        global(&None).unwrap();
    }
}
