use std::io::{self, Write};
use std::{env, path::Path};

use super::activate::Shell;
use super::lilypond::global::get_global_version;
use super::lilypond::install::get_install_path;

fn clear_lilypond(mut path: String) -> String {
    let values: &Vec<String> = &path
        .split(':')
        .filter(|value| value.contains("lilypond"))
        .map(ToString::to_string)
        .collect();

    for value in values {
        path = path.replace(&format!("{value}:"), "");
    }

    path
}

fn get_new_version(version: &Option<String>) -> String {
    version
        .as_ref()
        .map_or_else(get_global_version, ToString::to_string)
}

pub fn main(shell: &Shell, version: &Option<String>) {
    let install_path = get_install_path();
    let new_version = get_new_version(version);
    let global_version_path =
        format!("{}/lilypond-{}/bin", &install_path, &new_version);

    let path = env::var("PATH").expect("Failed to read PATH.");

    if !Path::new(&global_version_path).exists()
        || path.contains(&global_version_path)
    {
        return;
    }

    match shell {
        Shell::Nu => {
            io::stdout()
                .write_all(global_version_path.as_bytes())
                .expect("Failed to write global version path to stdout.");
            return;
        }
        Shell::Zsh => (),
    }

    let mut new_path = String::new();

    new_path.push_str(&format!("{global_version_path}:"));
    new_path.push_str(&clear_lilypond(path));
    new_path = new_path.replace(' ', "\\ ");

    io::stdout()
        .write_all(new_path.as_bytes())
        .expect("Failed to write updated PATH to stdout.");
}
