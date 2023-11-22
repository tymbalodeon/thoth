use shellexpand::tilde;
use std::fs::read_dir;

use crate::commands::lilypond::INSTALL_PATH;

pub fn list(_version_regex: &Option<String>) {
    let install_path = tilde(INSTALL_PATH).to_string();
    for path in read_dir(&install_path).unwrap() {
        println!(
            "{}",
            path.unwrap()
                .path()
                .display()
                .to_string()
                .replace(&format!("{}/lilypond-", &install_path), "")
        )
    }
}
