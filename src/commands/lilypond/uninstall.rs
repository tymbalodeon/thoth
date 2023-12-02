use std::fs::{read_dir, remove_dir_all};

use shellexpand::tilde;

use crate::commands::lilypond::INSTALL_PATH;

pub fn uninstall(version: &String) {
    let install_path = tilde(INSTALL_PATH).to_string();
    let err = "Failed to get lilypond installation path.";

    if let Some(Ok(path)) = read_dir(&install_path).expect(err).find(|path| {
        path.as_ref()
            .expect(err)
            .path()
            .display()
            .to_string()
            .replace(&format!("{install_path}/lilypond-"), "")
            == *version
    }) {
        remove_dir_all(path.path())
            .expect("Failed to remove lilypond installation folder.");
    } else {
        println!("Version \"{version}\" not installed.");
    }
}
