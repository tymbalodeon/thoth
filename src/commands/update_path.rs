use std::{env, path::Path};

use crate::commands::lilypond::global::read_global_version;

use super::lilypond::install::{get_install_path, parse_version};

pub fn update_path_main() {
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

    let mut new_path = String::new();

    new_path.push_str(&format!("{}:", global_version_path));
    new_path.push_str(&path);

    env::set_var("PATH", new_path);
}
