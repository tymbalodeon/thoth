use serde::{Deserialize, Serialize};
use std::fs;
use std::path::PathBuf;
use toml;

#[derive(Serialize, Deserialize, Debug)]
struct ConfigFile {
    thoth: Option<Thoth>,
}

#[derive(Serialize, Deserialize, Debug)]
struct Thoth {
    scores_directory: Option<String>,
}

#[derive(Debug)]
pub struct Config {
    pub scores_directory: String,
}

fn load_config_file(mut config_path: PathBuf) -> ConfigFile {
    config_path.push(".config/thoth/config.toml");
    let content = if let Ok(result) = fs::read_to_string(config_path) {
        result
    } else {
        "".to_owned()
    };
    if let Ok(config_file) = toml::from_str(&content) {
        config_file
    } else {
        ConfigFile { thoth: None }
    }
}

impl Config {
    pub fn new() -> Self {
        let default_scores_directory = "scores".to_owned();
        let scores_directory: String = if let Some(path) = home::home_dir() {
            let config_file = load_config_file(path);
            if let Some(thoth) = config_file.thoth {
                if let Some(scores_directory) = thoth.scores_directory {
                    scores_directory
                } else {
                    println!("WARNING: Missing scores directory value.");
                    default_scores_directory
                }
            } else {
                println!("WARNING: Missing table thoth.");
                default_scores_directory
            }
        } else {
            println!("WARNING: Missing config file.");
            default_scores_directory
        };
        Config { scores_directory }
    }
}
