use serde::{Deserialize, Serialize};
use std::fs;
use std::io::Error as IoError;
use std::path::PathBuf;
use toml;

#[derive(Serialize, Deserialize, Debug)]
struct ConfigToml {
    thoth: Option<ConfigTomlThoth>,
}

#[derive(Serialize, Deserialize, Debug)]
struct ConfigTomlThoth {
    scores_directory: Option<String>,
}

#[derive(Debug)]
pub struct Config {
    pub scores_directory: String,
}

impl Config {
    pub fn new() -> Self {
        let mut path = match home::home_dir() {
            Some(path) => path,
            None => PathBuf::from(""),
        };

        path.push("config.toml");

        let config_filepaths = [path];

        let mut content: String = "".to_owned();

        for filepath in config_filepaths {
            let result: Result<String, IoError> = fs::read_to_string(filepath);

            if result.is_ok() {
                content = result.unwrap();
                break;
            }
        }

        let config_toml: ConfigToml = toml::from_str(&content).unwrap_or_else(|_| {
            println!("Failed to create ConfigToml Object out of config file.");
            ConfigToml { thoth: None }
        });

        let scores_directory: String = match config_toml.thoth {
            Some(thoth) => {
                let thoth_scores_directory: String = thoth.scores_directory.unwrap_or_else(|| {
                    println!("Missing field username in table thoth.");
                    "unknown".to_owned()
                });

                thoth_scores_directory
            }
            None => {
                println!("Missing table thoth.");
                "unknown".to_owned()
            }
        };

        Config { scores_directory }
    }
}
