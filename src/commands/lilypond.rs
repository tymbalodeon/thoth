pub mod global;
pub mod install;
pub mod list;
pub mod list_remote;
pub mod uninstall;

use std::fmt;
use std::fmt::{Display, Formatter};

use itertools::{EitherOrBoth::*, Itertools};
use owo_colors::OwoColorize;
use regex::Regex;

use self::global::global;
use self::install::install;
use self::list::list;
use self::list_remote::{list_remote, LilypondReleases};
use self::uninstall::uninstall;
use super::table;
use super::{LilypondCommand, VersionStability};

static GLOBAL_PATH: &str = "~/.thoth-versions";
static INSTALL_PATH: &str = "~/.local/share/thoth";
static GITLAB_URL: &str =
    "https://gitlab.com/api/v4/projects/18695663/releases";

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
        let err = "Failed to parse lilypond minor version.";
        let minor_version = version
            .split('.')
            .enumerate()
            .filter(|(index, _)| index == &1usize)
            .map(|(_, value)| value)
            .next()
            .expect(err)
            .parse::<i32>()
            .expect(err);
        if minor_version % 2 == 0 {
            Ok(VersionStability::Stable)
        } else {
            Ok(VersionStability::Unstable)
        }
    }
}

pub fn get_tag_names() -> Vec<String> {
    let mut releases = vec![];
    let err = "Failed to parse lilypond release.";

    for release in LilypondReleases::get().expect(err) {
        let release = release.expect(err);
        if !release.assets.links.is_empty() {
            releases.push(release.tag_name.to_string());
        }
    }

    releases
}

pub fn get_versions() -> Vec<String> {
    get_tag_names()
        .iter()
        .map(|release| release.replace(['v', '"'], "").replace("release/", ""))
        .collect()
}

pub fn filter_versions<'a>(
    versions: &'a [String],
    stability: &'a VersionStability,
) -> Vec<&'a String> {
    versions
        .iter()
        .filter(|version| {
            &get_version_stability(version)
                .expect("Failed to get lilypond version stability.")
                == stability
        })
        .collect()
}

pub fn list_versions(
    mut versions: Vec<String>,
    version_regex: &Option<String>,
    stability: &Option<VersionStability>,
) {
    if let Some(stability) = stability {
        versions = versions
            .iter()
            .filter(|version| {
                get_version_stability(version)
                    .expect("Failed to get lilypond version stability.")
                    == *stability
            })
            .map(ToString::to_string)
            .collect();
    }

    if let Some(regex) = version_regex {
        let re = Regex::new(regex)
            .expect("Failed to parse lilypond version regex.");

        versions = versions
            .iter()
            .filter(|version| re.is_match(version))
            .map(ToString::to_string)
            .collect();
    }

    let stable = filter_versions(&versions, &VersionStability::Stable);
    let unstable = filter_versions(&versions, &VersionStability::Unstable);

    let mut titles = vec![];

    if !stable.is_empty() {
        titles.push("Stable".italic().green().to_string());
    }

    if !unstable.is_empty() {
        titles.push("Unstable".italic().yellow().to_string());
    }

    let mut rows: Vec<Vec<String>> = vec![];

    if !stable.is_empty() && !unstable.is_empty() {
        for pair in stable.iter().zip_longest(unstable.iter()) {
            match pair {
                Both(stable, unstable) => rows.push(vec![
                    (*stable).to_string(),
                    (*unstable).to_string(),
                ]),
                Left(stable) => {
                    rows.push(vec![(*stable).to_string(), String::new()]);
                }
                Right(unstable) => {
                    rows.push(vec![String::new(), (*unstable).to_string()]);
                }
            }
        }
    } else if !stable.is_empty() {
        for version in &stable {
            rows.push(vec![(*version).to_string()]);
        }
    } else if !unstable.is_empty() {
        for version in &unstable {
            rows.push(vec![(*version).to_string()]);
        }
    }

    table::print(&titles, rows);
}

pub fn main(version: &Option<String>, command: &Option<LilypondCommand>) {
    command.as_ref().map_or_else(
        || {
            global(version).unwrap_or_else(|err| {
                panic!(
                    "{}",
                    format!("Failed to get global lilypond version ({err})")
                )
            });
        },
        |command| match command {
            LilypondCommand::Install { version } => {
                install(version).expect("Failed to install lilypond.");
            }
            LilypondCommand::Uninstall { version } => uninstall(version),
            LilypondCommand::List {
                version_regex,
                stability,
            } => list(version_regex, stability),
            LilypondCommand::ListRemote {
                version_regex,
                stability,
            } => list_remote(version_regex, stability),
        },
    );
}
