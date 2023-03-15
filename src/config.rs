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

fn read_config_file(mut config_path: PathBuf) -> ConfigFile {
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
        let scores_directory: String = if let Some(path) = home::home_dir() {
            let config_file = read_config_file(path);

            if let Some(thoth) = config_file.thoth {
                if let Some(scores_directory) = thoth.scores_directory {
                    scores_directory
                } else {
                    println!("WARNING: Missing scores directory value.");
                    "unknown".to_owned()
                }
            } else {
                println!("WARNING: Missing table thoth.");
                "unknown".to_owned()
            }
        } else {
            println!("WARNING: Missing config file.");
            "unknown".to_owned()
        };

        Config { scores_directory }
    }
}
