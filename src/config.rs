use serde::{Deserialize, Serialize};
use shellexpand::tilde;
use std::fs;

#[derive(Serialize, Deserialize, Debug)]
struct Thoth {
    scores_directory: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
struct ConfigFile {
    thoth: Option<Thoth>,
}

#[derive(Debug)]
pub struct Config {
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
                scores_directory: Some(default_scores_directory),
            }),
        }
    }
}

impl Config {
    pub fn new() -> Self {
        let default_scores_directory = "scores".to_owned();
        let config_file = load_config_file();

        let scores_directory = if let Some(thoth) = config_file.thoth {
            if let Some(scores_directory) = thoth.scores_directory {
                scores_directory
            } else {
                println!("WARNING: Missing scores directory value.");
                default_scores_directory
            }
        } else {
            println!("WARNING: Missing table thoth.");
            default_scores_directory
        };

        Config { scores_directory }
    }
}
