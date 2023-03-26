use serde::{Deserialize, Serialize};
use shellexpand::tilde;
use std::fs;

#[derive(Serialize, Deserialize, Debug)]
struct Thoth {
    composer: Option<String>,
    scores_directory: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
struct ConfigFile {
    thoth: Option<Thoth>,
}

#[derive(Debug)]
pub struct Config {
    pub composer: String,
    pub scores_directory: String,
}

fn get_default_scores_directory() -> String {
    "scores".to_owned()
}

fn load_config_file() -> ConfigFile {
    let config_path = tilde("~/.config/thoth/config.toml");

    let contents = if let Ok(result) = fs::read_to_string(config_path.as_ref()) {
        result
    } else {
        "".to_owned()
    };

    if let Ok(config_file) = toml::from_str(&contents) {
        config_file
    } else {
        println!("WARNING: Missing config file. Using default.");
        let default_scores_directory = get_default_scores_directory();

        ConfigFile {
            thoth: Some(Thoth {
                composer: Some("".to_owned()),
                scores_directory: Some(default_scores_directory),
            }),
        }
    }
}

impl Config {
    pub fn new() -> Self {
        let default_scores_directory = get_default_scores_directory();
        let config_file = load_config_file();

        if let Some(thoth) = config_file.thoth {
            let composer = if let Some(composer) = thoth.composer {
                composer
            } else {
                println!("WARNING: Missing composer value.");
                "".to_owned()
            };

            let scores_directory = if let Some(scores_directory) = thoth.scores_directory {
                scores_directory
            } else {
                println!("WARNING: Missing scores directory value.");
                default_scores_directory
            };

            Config {
                composer,
                scores_directory,
            }
        } else {
            println!("WARNING: Missing table thoth.");

            Config {
                composer: "".to_owned(),
                scores_directory: default_scores_directory,
            }
        }
    }
}

pub fn get_composer() -> String {
    let config: Config = Config::new();
    tilde(&config.composer).into_owned()
}

pub fn get_scores_directory() -> String {
    let config: Config = Config::new();
    tilde(&config.scores_directory).into_owned()
}
