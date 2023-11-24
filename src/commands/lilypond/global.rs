use std::{
    fs::{read_to_string, write, File},
    io::{self, prelude::*},
    path::Path,
};

use shellexpand::tilde;

use super::{
    get_version_stability, install, is_latest_version, is_valid_version,
    GLOBAL_PATH,
};

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

fn get_global_path() -> io::Result<String> {
    let global_path = tilde(GLOBAL_PATH).to_string();

    if !Path::new(&global_path).exists() {
        let mut file = File::create(&global_path)?;
        file.write_all(b"latest-stable")?;
    }

    Ok(global_path)
}

pub fn read_global_version() -> io::Result<String> {
    read_to_string(get_global_path()?)
}

pub fn global(version: &Option<String>) -> io::Result<()> {
    let global_path = get_global_path()?;

    if let Some(value) = version {
        if is_valid_version(value) {
            write(global_path, value)?;
            install(version)?;
        }

        print_version(value);
    } else {
        let version = read_global_version()?;
        if is_valid_version(&version) {
            print_version(&version);
        } else {
            println!("No global lilypond version set.");
        }
    }

    Ok(())
}
