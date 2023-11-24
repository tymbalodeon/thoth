use human_sort::sort;
use shellexpand::tilde;
use std::fs::read_dir;

use crate::commands::{lilypond::INSTALL_PATH, VersionStability};

use super::list_versions;

pub fn list(
    version_regex: &Option<String>,
    stability: &Option<VersionStability>,
) {
    let install_path = tilde(INSTALL_PATH).to_string();
    let versions: Vec<String> = read_dir(&install_path)
        .unwrap()
        .map(|path| {
            path.unwrap()
                .path()
                .display()
                .to_string()
                .replace(&format!("{}/lilypond-", &install_path), "")
        })
        .collect();

    let mut versions: Vec<&str> =
        versions.iter().map(|version| version.as_str()).collect();

    sort(&mut versions);

    let versions: Vec<String> = versions
        .iter()
        .rev()
        .map(|version| version.to_string())
        .collect();

    list_versions(versions, version_regex, stability)
}
