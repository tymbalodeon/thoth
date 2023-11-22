use std::{
    env,
    fs::{read_to_string, write},
    path::Path,
};

use shellexpand::tilde;

use crate::commands::lilypond::install::parse_version;

use super::{
    get_version_stability, install::get_install_path, is_latest_version,
    is_valid_version, GLOBAL_PATH,
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

pub fn read_global_version() -> String {
    read_to_string(tilde(GLOBAL_PATH).to_string()).unwrap()
}

fn update_path() {
    let install_path = get_install_path();
    let global_version = parse_version(&read_global_version());
    let global_version_path =
        format!("{}/lilypond-{}/bin", &install_path, &global_version);

    if !Path::new(&global_version_path).exists() {
        return;
    }

    let mut path = env::var("PATH").unwrap();
    let values: &Vec<String> = &path
        .split(':')
        .filter(|value| value.contains("lilypond"))
        .map(|value| value.to_string())
        .collect();

    for value in values {
        path = path.replace(&format!("{value}:"), "");
    }

    let mut new_path = "".to_string();

    new_path.push_str(&format!("{}:", global_version_path));
    new_path.push_str(&path);

    env::set_var("PATH", new_path);
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

    update_path();

    Ok(())
}
