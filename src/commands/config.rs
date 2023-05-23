use shellexpand::tilde;
use std::process::Command;

use crate::config::{Config, CONFIG_PATH};

pub fn edit_config() {
    Command::new("open")
        .arg(tilde(CONFIG_PATH).to_string())
        .output()
        .unwrap();
}

pub fn display_config_path() {
    println!("{}", tilde(CONFIG_PATH));
}

pub fn display_config() {
    let config: Config = Config::new();

    let composer = &config.composer;
    let scores_directory = &config.scores_directory();
    let pdfs_directory = &config.pdfs_directory();

    println!("Composer = {composer}");
    println!("Scores directory = {scores_directory}");
    println!("pdfs directory = {pdfs_directory}");
}
