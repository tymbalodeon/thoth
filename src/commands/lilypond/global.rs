use std::fs::{read_to_string, write};

use shellexpand::tilde;

use super::{get_version_stability, is_latest_version, is_valid_version};

static GLOBAL_PATH: &str = "~/.thoth-versions";

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

pub fn read_global_version() -> String {
    read_to_string(tilde(GLOBAL_PATH).to_string()).unwrap()
}

pub fn global(version: &Option<String>) -> Result<(), &'static str> {
    let global_path = tilde(GLOBAL_PATH).to_string();

    if let Some(value) = version {
        if is_valid_version(value) {
            let _ = write(global_path, value);
        }

        print_version(value);
    } else {
        let version = read_global_version();
        if is_valid_version(&version) {
            print_version(&version);
        } else {
            println!("No global lilypond version set.");
        }
    }

    Ok(())
}
