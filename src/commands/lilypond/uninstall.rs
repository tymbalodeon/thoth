use std::fs::{read_dir, remove_dir_all};

use shellexpand::tilde;

use crate::commands::lilypond::INSTALL_PATH;

pub fn uninstall(version: &String) {
    let install_path = tilde(INSTALL_PATH).to_string();
    if let Some(Ok(path)) = read_dir(&install_path).unwrap().find(|path| {
        path.as_ref()
            .unwrap()
            .path()
            .display()
            .to_string()
            .replace(&format!("{}/lilypond-", install_path), "")
            == *version
    }) {
        remove_dir_all(path.path()).unwrap();
    } else {
        println!("Version \"{version}\" not installed.");
    }
}
