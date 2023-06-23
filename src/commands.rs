pub mod clean;
pub mod compile;
pub mod config;
pub mod create;
pub mod edit;
pub mod helpers;
pub mod list;
pub mod open;
mod patterns;
mod scores;
mod table;
pub mod templates;

use std::fmt::{Display, Formatter, Result};

use clap::Subcommand;
use clap::ValueEnum;
use convert_case::{Case::Kebab, Casing};
use serde::Deserialize;
use shellexpand::tilde;

use crate::commands::helpers::Helper;
use crate::commands::templates::Template;
use crate::config::Config;

#[derive(Clone, Debug, Deserialize, ValueEnum)]
pub enum ConfigKey {
    Composer,
    Instrument,
    PDFSDirectory,
    ScoresDirectory,
    Template,
}

impl Display for ConfigKey {
    fn fmt(&self, formatter: &mut Formatter) -> Result {
        let display = format!("{self:?}").to_case(Kebab);
        write!(formatter, "{display}")
    }
}

#[derive(Subcommand)]
pub enum TemplateCommand {
    /// Show the template contents
    Show { template: Template },
}

#[derive(Subcommand)]
pub enum HelperCommand {
    /// Show helper file contents
    Show { helper: Helper },
}

#[derive(Subcommand)]
pub enum Command {
    /// Remove pdf(s)
    Clean {
        scores: Vec<String>,

        #[arg(long)]
        pdfs_directory: Option<String>,
    },

    /// Create pdf(s)
    Compile {
        scores: Vec<String>,

        #[arg(long)]
        pdfs_directory: Option<String>,
    },

    /// Display config
    Config {
        key: Option<ConfigKey>,

        #[arg(long)]
        set: Option<String>,

        /// Open config file in editor
        #[arg(long)]
        edit: bool,

        /// Display the config file path
        #[arg(long)]
        path: bool,
    },

    /// Create new score template
    Create {
        title: String,

        #[arg(long)]
        subtitle: Option<String>,

        #[arg(long)]
        composer: Option<String>,

        #[arg(long)]
        arranger: Option<String>,

        #[arg(long, value_enum)]
        template: Option<Template>,

        #[arg(long)]
        instrument: Option<String>,

        /// Open for editing after creating
        #[arg(long)]
        edit: bool,

        #[arg(long)]
        pdfs_directory: Option<String>,
    },

    /// Open <score> in editor and pdf viewer, recompiling on file changes
    Edit {
        score: String,

        #[arg(long)]
        pdfs_directory: Option<String>,
    },

    /// List pdf(s)
    List {
        scores: Vec<String>,

        #[arg(long)]
        compiled: bool,

        #[arg(long)]
        outdated: bool,

        #[arg(long)]
        pdfs_directory: Option<String>,
    },

    /// Open pdf(s)
    Open {
        scores: Vec<String>,

        #[arg(long)]
        pdfs_directory: Option<String>,
    },

    /// List template types
    Templates {
        #[command(subcommand)]
        command: Option<TemplateCommand>,
    },

    /// List helper files
    Helpers {
        #[command(subcommand)]
        command: Option<HelperCommand>,
    },
}

pub fn add_value_to_string_if_some(
    mut string: String,
    key: &str,
    value: &Option<String>,
) -> String {
    if let Some(value) = value {
        let line = format!("  {key} = \"{value}\"\n");
        string.push_str(&line);
    };

    string.to_string()
}

pub fn get_pdfs_directory_from_arg(pdfs_directory: &Option<String>) -> String {
    if let Some(path) = pdfs_directory {
        tilde(&path).to_string()
    } else {
        Config::get_pdfs_directory()
    }
}
