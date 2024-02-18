use std::{
    fs::{create_dir_all, read_to_string, write},
    path::Path,
    process::Command,
};

use owo_colors::OwoColorize;
use serde::{Deserialize, Serialize};
use shellexpand::tilde;
use toml::{from_str, to_string};
use users::get_current_username;

use crate::commands::table;
use crate::commands::{templates::Template, ConfigKey};

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
        Self {
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
    create_dir_all(
        path.parent()
            .expect("Failed to get config path parent directory."),
    )
    .expect("Failed to created config path parent directory.");
    let contents = to_string(&ConfigFile::default())
        .expect("Failed to create default config.");
    write(config_path, contents).expect("Unable to write config");
}

fn load_config_file() -> ConfigFile {
    let config_path_name = get_config_path();
    let config_path = Path::new(&config_path_name);

    if !config_path.exists() {
        create_config_file();
    }

    let config =
        read_to_string(config_path_name).unwrap_or_else(|_| String::new());

    from_str(&config).unwrap_or_else(|_| {
        create_config_file();

        ConfigFile::default()
    })
}

fn get_default_scores_directory() -> String {
    "~/scores".to_string()
}

fn get_default_pdfs_directory() -> String {
    "pdfs".to_string()
}

const fn get_default_template() -> Template {
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
        let username =
            get_current_username().map_or_else(String::new, |username| {
                username
                    .to_str()
                    .expect("Failed to parse username from config.")
                    .to_string()
            });

        Self {
            composer: username,
            scores_directory: get_default_scores_directory(),
            pdfs_directory: get_default_pdfs_directory(),
            template: get_default_template(),
            instrument: get_default_instrument(),
        }
    }
}

fn get_template_from_string(value: &str) -> Option<Template> {
    match value {
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

        let composer = config_file.composer.map_or_else(
            || {
                get_current_username().map_or_else(String::new, |username| {
                    username
                        .to_str()
                        .expect("Failed to parse username from config.")
                        .to_string()
                })
            },
            |composer| composer,
        );

        let scores_directory = config_file
            .scores_directory
            .map_or_else(get_default_scores_directory, |scores_directory| {
                tilde(&scores_directory).into_owned()
            });

        let pdfs_directory = config_file.pdfs_directory.map_or_else(
            || {
                let default_pdfs_directory = get_default_pdfs_directory();
                format!("{scores_directory}/{default_pdfs_directory}")
            },
            |pdfs_directory| tilde(&pdfs_directory).into_owned(),
        );

        let template = config_file
            .template
            .map_or_else(get_default_template, |template| template);

        let instrument = config_file
            .instrument
            .map_or_else(get_default_instrument, |instrument| instrument);

        Self {
            composer,
            scores_directory,
            pdfs_directory,
            template,
            instrument,
        }
    }

    pub fn get_composer() -> String {
        Self::from_config_file().composer
    }

    pub fn get_instrument() -> String {
        Self::from_config_file().instrument
    }

    pub fn get_scores_directory() -> String {
        Self::from_config_file().scores_directory
    }

    pub fn get_pdfs_directory() -> String {
        Self::from_config_file().pdfs_directory
    }

    pub fn get_template() -> Template {
        Self::from_config_file().template
    }

    fn style_key(key: &str) -> String {
        key.yellow().to_string()
    }

    fn style_value(value: &str) -> String {
        value.bold().to_string()
    }

    fn style_key_value(key: &str, value: &str) -> Vec<String> {
        vec![Self::style_key(key), Self::style_value(value)]
    }

    pub fn display() {
        let config = Self::from_config_file();

        let rows = vec![
            Self::style_key_value("composer", config.composer.as_str()),
            Self::style_key_value("instrument", config.instrument.as_str()),
            Self::style_key_value(
                "scores_directory",
                config.scores_directory.as_str(),
            ),
            Self::style_key_value(
                "pdfs_directory",
                config.pdfs_directory.as_str(),
            ),
            Self::style_key_value(
                "template",
                config.template.to_string().as_str(),
            ),
        ];

        table::print(&[], rows);
    }

    pub fn display_path() {
        println!("{}", get_config_path());
    }

    pub fn edit() {
        Command::new("open")
            .arg(get_config_path())
            .output()
            .expect("Failed to open config path.");
    }

    pub fn display_value(key: &ConfigKey) {
        match key.to_string().replace('-', "_").to_lowercase().as_str() {
            "composer" => println!("{}", Self::get_composer()),
            "instrument" => {
                println!("{}", Self::get_instrument());
            }
            "scores_directory" => {
                println!("{}", Self::get_scores_directory());
            }
            "pdfs_directory" => {
                println!("{}", Self::get_pdfs_directory());
            }
            "template" => {
                println!("{:?}", Self::get_template());
            }
            _ => println!("\"{key}\" is not a recognized config key"),
        };
    }

    pub fn set_value(key: &ConfigKey, value: String) {
        let mut config = Self::from_config_file();

        match key {
            ConfigKey::Composer => config.composer = value,
            ConfigKey::ScoresDirectory => config.scores_directory = value,
            ConfigKey::PDFSDirectory => config.pdfs_directory = value,
            ConfigKey::Template => {
                let value = get_template_from_string(&value);
                if let Some(template) = value {
                    config.template = template;
                }
            }
            ConfigKey::Instrument => config.instrument = value,
        };

        let contents = to_string(&ConfigFile::from_config(config))
            .expect("Failed to read config.");
        let config_path_name = get_config_path();
        let config_path = Path::new(&config_path_name);
        write(config_path, contents).expect("Unable to write config.");
        Self::display();
    }
}
