use std::fs::{read_to_string, write};

use shellexpand::tilde;

use super::{get_releases, get_version_stability, is_latest_version};

static GLOBAL_PATH: &str = "~/.thoth-versions";

fn is_valid_version(version: &String) -> bool {
    let mut versions =
        vec!["latest-stable".to_string(), "latest-unstable".to_string()];
    versions.append(&mut get_releases());

    versions.contains(version)
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

pub fn global(version: &Option<String>) -> Result<(), &'static str> {
    let global_path = tilde(GLOBAL_PATH).to_string();

    if let Some(value) = version {
        if is_valid_version(value) {
            let _ = write(global_path, value);
        }

        print_version(value);
    } else if let Ok(version) = read_to_string(&global_path) {
        print_version(&version);
    } else {
        println!("No global lilypond version set.");
    };

    Ok(())
}
