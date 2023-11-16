use std::{
    fs::{create_dir_all, read_to_string, write},
    path::Path,
    process::Command,
};

use crate::commands::table::print_table;
use crate::commands::{templates::Template, ConfigKey};
use owo_colors::OwoColorize;
use serde::{Deserialize, Serialize};
use shellexpand::tilde;
use toml::{from_str, to_string};
use users::get_current_username;

static CONFIG_PATH: &str = "~/.config/thoth/config.toml";

#[derive(Debug, Default, Deserialize, Serialize)]
struct ConfigFile {
    composer: Option<String>,
    scores_directory: Option<String>,
    pdfs_directory: Option<String>,
    template: Option<Template>,
    instrument: Option<String>,
}

impl ConfigFile {
    pub fn from_config(config: Config) -> Self {
        ConfigFile {
            composer: Some(config.composer),
            scores_directory: Some(config.scores_directory),
            pdfs_directory: Some(config.pdfs_directory),
            template: Some(config.template),
            instrument: Some(config.instrument),
        }
    }
}

fn get_config_path() -> String {
    tilde(CONFIG_PATH).to_string()
}

fn create_config_file() {
    let config_path = get_config_path();
    let path = Path::new(&config_path);
    create_dir_all(path.parent().unwrap()).unwrap();
    let contents = to_string(&ConfigFile::default()).unwrap();
    write(config_path, contents).expect("Unable to write config");
}

fn load_config_file() -> ConfigFile {
    let config_path_name = get_config_path();
    let config_path = Path::new(&config_path_name);

    if !config_path.exists() {
        create_config_file();
    }

    let config = if let Ok(config_path) = read_to_string(config_path_name) {
        config_path
    } else {
        "".to_owned()
    };

    if let Ok(config_file) = from_str(&config) {
        config_file
    } else {
        create_config_file();

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

fn get_template_from_string(value: String) -> Option<Template> {
    match value.as_str() {
        "form" => Some(Template::Form),
        "lead" => Some(Template::Lead),
        "piano" => Some(Template::Piano),
        "single" => Some(Template::Single),
        _ => None,
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

        let header =
            vec!["Key".italic().to_string(), "Value".italic().to_string()];

        let rows = vec![
            vec![
                "composer".yellow().to_string(),
                config.composer.bold().to_string(),
            ],
            vec![
                "instrument".yellow().to_string(),
                config.instrument.bold().to_string(),
            ],
            vec![
                "scores_directory".yellow().to_string(),
                config.scores_directory.bold().to_string(),
            ],
            vec![
                "pdfs_directory".yellow().to_string(),
                config.pdfs_directory.bold().to_string(),
            ],
            vec![
                "template".yellow().to_string(),
                format!("{:?}", config.template).bold().to_string(),
            ],
        ];

        print_table(header, rows);
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

    pub fn display_value(key: &ConfigKey) {
        match key.to_string().replace('-', "_").to_lowercase().as_str() {
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

    pub fn set_value(key: &ConfigKey, value: String) {
        let mut config = Config::from_config_file();

        match key {
            ConfigKey::Composer => config.composer = value,
            ConfigKey::ScoresDirectory => config.scores_directory = value,
            ConfigKey::PDFSDirectory => config.pdfs_directory = value,
            ConfigKey::Template => {
                let value = get_template_from_string(value);
                if let Some(template) = value {
                    config.template = template;
                }
            }
            ConfigKey::Instrument => config.instrument = value,
        };

        let contents = to_string(&ConfigFile::from_config(config)).unwrap();
        let config_path_name = get_config_path();
        let config_path = Path::new(&config_path_name);
        write(config_path, contents).expect("Unable to write config");
        Config::display();
    }
}
