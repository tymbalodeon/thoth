use serde::{Deserialize, Serialize};
use shellexpand::tilde;
use std::fs::read_to_string;
use toml::from_str;

pub static CONFIG_PATH: &str = "~/.config/thoth/config.toml";

fn get_default_scores_directory() -> String {
    "~/scores".to_string()
}

fn get_default_pdfs_directory() -> String {
    "pdfs".to_string()
}

#[derive(Serialize, Deserialize, Debug)]
struct Thoth {
    composer: Option<String>,
    scores_directory: Option<String>,
    pdfs_directory: Option<String>,
}

impl Default for Thoth {
    fn default() -> Self {
        Thoth {
            composer: Some("".to_string()),
            scores_directory: Some(get_default_scores_directory()),
            pdfs_directory: Some(get_default_pdfs_directory()),
        }
    }
}

#[derive(Serialize, Deserialize, Debug)]
struct ConfigFile {
    thoth: Option<Thoth>,
}

impl Default for ConfigFile {
    fn default() -> Self {
        ConfigFile {
            thoth: Some(Thoth::default()),
        }
    }
}

fn load_config_file() -> ConfigFile {
    let config_path =
        if let Ok(config_path) = read_to_string(tilde(CONFIG_PATH).as_ref()) {
            config_path
        } else {
            "".to_owned()
        };

    if let Ok(config_file) = from_str(&config_path) {
        config_file
    } else {
        ConfigFile::default()
    }
}

fn get_composer_or_default(thoth: &Thoth) -> String {
    if let Some(composer) = &thoth.composer {
        composer.to_string()
    } else {
        "".to_string()
    }
}

#[derive(Debug)]
pub struct Config {
    pub composer: String,
    pub scores_directory: String,
    pub pdfs_directory: String,
}

impl Default for Config {
    fn default() -> Self {
        Config {
            composer: "".to_string(),
            scores_directory: get_default_scores_directory(),
            pdfs_directory: get_default_pdfs_directory(),
        }
    }
}

impl Config {
    pub fn new() -> Self {
        let config_file = load_config_file();

        if let Some(thoth) = config_file.thoth {
            let composer = get_composer_or_default(&thoth);
            let scores_directory =
                if let Some(scores_directory) = thoth.scores_directory {
                    scores_directory
                } else {
                    get_default_scores_directory()
                };

            let pdfs_directory =
                if let Some(pdfs_directory) = thoth.pdfs_directory {
                    pdfs_directory
                } else {
                    let default_pdfs_directory = get_default_pdfs_directory();
                    format!("{scores_directory}/{default_pdfs_directory}")
                };

            Config {
                composer,
                scores_directory,
                pdfs_directory,
            }
        } else {
            Config::default()
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
