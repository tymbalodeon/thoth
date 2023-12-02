use std::fs::read_dir;

use human_sort::sort;
use shellexpand::tilde;

use super::list_versions;
use crate::commands::{lilypond::INSTALL_PATH, VersionStability};

pub fn list(
    version_regex: &Option<String>,
    stability: &Option<VersionStability>,
) {
    let install_path = tilde(INSTALL_PATH).to_string();
    let err = "Failed to parse installed lilypond version.";

    let versions: Vec<String> = read_dir(&install_path)
        .expect(err)
        .map(|path| {
            path.expect(err)
                .path()
                .display()
                .to_string()
                .replace(&format!("{}/lilypond-", &install_path), "")
        })
        .collect();

    let mut versions: Vec<&str> =
        versions.iter().map(String::as_str).collect();

    sort(&mut versions);

    let versions: Vec<String> =
        versions.iter().rev().map(ToString::to_string).collect();

    list_versions(versions, version_regex, stability);
}
