use serde::{Deserialize, Serialize};
use shellexpand::tilde;
use std::fs::read_to_string;
use toml::from_str;

pub static CONFIG_PATH: &str = "~/.config/thoth/config.toml";

#[derive(Serialize, Deserialize, Debug)]
struct Thoth {
    composer: Option<String>,
    scores_directory: Option<String>,
    pdfs_directory: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
struct ConfigFile {
    thoth: Option<Thoth>,
}

#[derive(Debug)]
pub struct Config {
    pub composer: String,
    pub scores_directory: String,
    pub pdfs_directory: String,
}

fn get_default_scores_directory() -> String {
    "~/scores".to_owned()
}

fn get_default_pdfs_directory() -> String {
    "pdfs".to_owned()
}

fn get_config_path() -> String {
    let config_path = tilde(CONFIG_PATH);

    if let Ok(result) = read_to_string(config_path.as_ref()) {
        result
    } else {
        "".to_owned()
    }
}

fn load_config_file() -> ConfigFile {
    let config_path = get_config_path();

    if let Ok(config_file) = from_str(&config_path) {
        config_file
    } else {
        ConfigFile {
            thoth: Some(Thoth {
                composer: Some("".to_owned()),
                scores_directory: Some(get_default_scores_directory()),
                pdfs_directory: Some(get_default_pdfs_directory()),
            }),
        }
    }
}

impl Config {
    pub fn new() -> Self {
        let default_scores_directory = get_default_scores_directory();
        let default_pdfs_directory = get_default_pdfs_directory();
        let config_file = load_config_file();

        if let Some(thoth) = config_file.thoth {
            let composer = if let Some(composer) = thoth.composer {
                composer
            } else {
                "".to_owned()
            };

            let scores_directory =
                if let Some(scores_directory) = thoth.scores_directory {
                    scores_directory
                } else {
                    default_scores_directory
                };

            let pdfs_directory =
                if let Some(pdfs_directory) = thoth.pdfs_directory {
                    pdfs_directory
                } else {
                    format!("{scores_directory}/{default_pdfs_directory}")
                };

            Config {
                composer,
                scores_directory,
                pdfs_directory,
            }
        } else {
            Config {
                composer: "".to_owned(),
                scores_directory: default_scores_directory,
                pdfs_directory: default_pdfs_directory,
            }
        }
    }

    pub fn scores_directory(&self) -> String {
        tilde(&self.scores_directory).into_owned()
    }

    pub fn pdfs_directory(&self) -> String {
        tilde(&self.pdfs_directory).into_owned()
    }
}

pub fn get_composer() -> String {
    Config::new().composer
}
