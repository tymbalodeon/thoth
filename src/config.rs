use crate::commands::templates::Template;
use serde::Deserialize;
use shellexpand::tilde;
use std::{fs::read_to_string, process::Command};
use toml::from_str;
use users::get_current_username;

static CONFIG_PATH: &str = "~/.config/thoth/config.toml";

#[derive(Debug, Default, Deserialize)]
struct ConfigFile {
    composer: Option<String>,
    scores_directory: Option<String>,
    pdfs_directory: Option<String>,
    template: Option<Template>,
    instrument: Option<String>,
}

fn get_config_path() -> String {
    tilde(CONFIG_PATH).to_string()
}

fn load_config_file() -> ConfigFile {
    let config_path =
        if let Ok(config_path) = read_to_string(get_config_path()) {
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

fn get_default_template() -> Template {
    Template::Piano
}

fn get_default_instrument() -> String {
    "Instrument".to_string()
}

#[derive(Debug, Deserialize)]
pub struct Config {
    pub composer: String,
    pub scores_directory: String,
    pub pdfs_directory: String,
    pub template: Template,
    pub instrument: String,
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
            template: get_default_template(),
            instrument: get_default_instrument(),
        }
    }
}

impl Config {
    pub fn from_config_file() -> Self {
        let config_file = load_config_file();

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

        let template = if let Some(template) = config_file.template {
            template
        } else {
            get_default_template()
        };

        let instrument = if let Some(instrument) = config_file.instrument {
            instrument
        } else {
            get_default_instrument()
        };

        Config {
            composer,
            scores_directory,
            pdfs_directory,
            template,
            instrument,
        }
    }

    pub fn get_composer() -> String {
        Config::from_config_file().composer
    }

    pub fn get_instrument() -> String {
        Config::from_config_file().instrument
    }

    pub fn get_scores_directory() -> String {
        Config::from_config_file().scores_directory
    }

    pub fn get_pdfs_directory() -> String {
        Config::from_config_file().pdfs_directory
    }

    pub fn get_template() -> Template {
        Config::from_config_file().template
    }

    pub fn display() {
        let config = Config::from_config_file();

        let mut items = vec![
            format!("Composer = {}", &config.composer),
            format!("Scores directory = {}", &config.scores_directory),
            format!("PDFs directory = {}", &config.pdfs_directory),
            format!("Template = {:?}", &config.template),
            format!("Instrument = {}", &config.instrument),
        ];

        items.sort();

        for item in items {
            println!("{item}");
        }
    }

    pub fn display_path() {
        println!("{}", get_config_path());
    }

    pub fn edit() {
        Command::new("open")
            .arg(get_config_path())
            .output()
            .unwrap();
    }

    pub fn display_value(key: &str) {
        match key.replace('-', "_").to_lowercase().as_str() {
            "composer" => println!("{}", Config::get_composer()),
            "instrument" => {
                println!("{}", Config::get_instrument())
            }
            "scores_directory" => {
                println!("{}", Config::get_scores_directory())
            }
            "pdfs_directory" => {
                println!("{}", Config::get_pdfs_directory())
            }
            "template" => {
                println!("{:?}", Config::get_template())
            }
            _ => println!("\"{key}\" is not a recognized config key"),
        };
    }
}
