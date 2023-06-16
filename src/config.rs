use serde::Deserialize;
use shellexpand::tilde;
use std::fs::read_to_string;
use toml::from_str;
use users::get_current_username;

pub static CONFIG_PATH: &str = "~/.config/thoth/config.toml";

#[derive(Debug, Default, Deserialize)]
pub struct ConfigFile {
    pub composer: Option<String>,
    pub scores_directory: Option<String>,
    pub pdfs_directory: Option<String>,
}

fn load_config() -> ConfigFile {
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

fn get_default_scores_directory() -> String {
    "~/scores".to_string()
}

fn get_default_pdfs_directory() -> String {
    "pdfs".to_string()
}

#[derive(Debug, Deserialize)]
pub struct Config {
    pub composer: String,
    pub scores_directory: String,
    pub pdfs_directory: String,
}

impl Default for Config {
    fn default() -> Self {
        let user_name = if let Some(user_name) = get_current_username() {
            user_name.to_str().unwrap().to_string()
        } else {
            "".to_string()
        };

        Config {
            composer: user_name,
            scores_directory: get_default_scores_directory(),
            pdfs_directory: get_default_pdfs_directory(),
        }
    }
}

impl Config {
    fn from_file(config_file: ConfigFile) -> Self {
        let composer = if let Some(composer) = config_file.composer {
            composer
        } else if let Some(username) = get_current_username() {
            username.to_str().unwrap().to_string()
        } else {
            "".to_string()
        };

        let scores_directory =
            if let Some(scores_directory) = config_file.scores_directory {
                tilde(&scores_directory).into_owned()
            } else {
                get_default_scores_directory()
            };

        let pdfs_directory =
            if let Some(pdfs_directory) = config_file.pdfs_directory {
                tilde(&pdfs_directory).into_owned()
            } else {
                let default_pdfs_directory = get_default_pdfs_directory();
                format!("{scores_directory}/{default_pdfs_directory}")
            };

        Config {
            composer,
            scores_directory,
            pdfs_directory,
        }
    }

    pub fn new() -> Self {
        let config_file = load_config();
        Config::from_file(config_file)
    }
}

pub fn get_composer() -> String {
    Config::new().composer
}
