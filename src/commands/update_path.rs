use std::io::{self, Write};
use std::{env, path::Path};

use super::lilypond::global::get_global_version;
use super::lilypond::install::get_install_path;

fn clear_lilypond(mut path: String) -> String {
    let values: &Vec<String> = &path
        .split(':')
        .filter(|value| value.contains("lilypond"))
        .map(|value| value.to_string())
        .collect();

    for value in values {
        path = path.replace(&format!("{value}:"), "");
    }

    path
}

fn get_new_version(version: &Option<String>) -> String {
    if let Some(version) = version {
        version.to_string()
    } else {
        get_global_version()
    }
}

pub fn update_path_main(version: &Option<String>) -> io::Result<()> {
    let install_path = get_install_path();
    let new_version = get_new_version(version);
    let global_version_path =
        format!("{}/lilypond-{}/bin", &install_path, &new_version);
    let path = env::var("PATH").unwrap();

    if !Path::new(&global_version_path).exists()
        || path.contains(&global_version_path)
    {
        return Ok(());
    }

    let mut new_path = String::new();

    new_path.push_str(&format!("{}:", global_version_path));
    new_path.push_str(&clear_lilypond(path));
    new_path = new_path.replace(' ', "\\ ");

    let path_command = format!("PATH={new_path}");
    io::stdout().write_all(path_command.as_bytes()).unwrap();

    Ok(())
}
