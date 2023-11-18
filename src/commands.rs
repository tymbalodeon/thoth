pub mod clean;
pub mod compile;
pub mod config;
pub mod create;
pub mod edit;
pub mod helpers;
pub mod info;
pub mod lilypond;
pub mod list;
pub mod open;
mod patterns;
mod scores;
pub mod sketch;
pub mod table;
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

#[derive(Clone, Debug, Deserialize, ValueEnum)]
pub enum ScoreFileType {
    Both,
    Lilypond,
    Pdf,
}

#[derive(Debug, Subcommand)]
pub enum LilypondCommand {
    /// Show or set the global lilypond version
    Global { version: Option<String> },

    /// Install lilypond version(s)
    Install { version: Option<String> },

    /// List lilypond version(s)
    List {
        /// List all versions available for download
        #[arg(long)]
        remote: bool,

        version_regex: Option<String>,
    },
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
        search_terms: Vec<String>,

        /// Match search terms against artist field only
        #[arg(long)]
        artist: bool,

        /// Match search terms against title field only
        #[arg(long)]
        title: bool,

        /// Use all matching scores without prompting
        #[arg(long)]
        all: bool,

        #[arg(long)]
        scores_directory: Option<String>,

        #[arg(long)]
        pdfs_directory: Option<String>,
    },

    /// Create pdf(s)
    Compile {
        search_terms: Vec<String>,

        /// Match search terms against artist field only
        #[arg(long)]
        artist: bool,

        /// Match search terms against title field only
        #[arg(long)]
        title: bool,

        /// Use all matching scores without prompting
        #[arg(long)]
        all: bool,

        #[arg(long)]
        scores_directory: Option<String>,

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
        scores_directory: Option<String>,

        #[arg(long)]
        pdfs_directory: Option<String>,
    },

    /// Open <score> in editor and pdf viewer, recompiling on file changes
    Edit {
        search_terms: String,

        /// Match search terms against artist field only
        #[arg(long)]
        artist: bool,

        /// Match search terms against title field only
        #[arg(long)]
        title: bool,

        /// Use all matching scores without prompting
        #[arg(long)]
        all: bool,

        #[arg(long)]
        scores_directory: Option<String>,

        #[arg(long)]
        pdfs_directory: Option<String>,
    },

    /// Display <score> info
    Info {
        search_term: String,

        /// Match search terms against artist field only
        #[arg(long)]
        artist: bool,

        /// Match search terms against title field only
        #[arg(long)]
        title: bool,

        /// Use all matching scores without prompting
        #[arg(long)]
        all: bool,

        #[arg(long)]
        scores_directory: Option<String>,
    },

    /// Manage lilypond installation(s)
    Lilypond {
        #[command(subcommand)]
        command: Option<LilypondCommand>,
    },

    /// List pdf(s)
    List {
        search_terms: Vec<String>,

        #[arg(long)]
        compiled: bool,

        #[arg(long)]
        outdated: bool,

        /// Match search terms against artist field only
        #[arg(long)]
        artist: bool,

        /// Match search terms against title field only
        #[arg(long)]
        title: bool,

        #[arg(long)]
        scores_directory: Option<String>,

        #[arg(long)]
        pdfs_directory: Option<String>,
    },

    /// Open score(s)
    Open {
        search_terms: Vec<String>,

        /// Match search terms against artist field only
        #[arg(long)]
        artist: bool,

        /// Match search terms against title field only
        #[arg(long)]
        title: bool,

        /// Use all matching scores without prompting
        #[arg(long)]
        all: bool,

        #[arg(long)]
        file_type: Option<ScoreFileType>,

        #[arg(long)]
        scores_directory: Option<String>,

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

    /// Open temporary sketch file
    Sketch {},
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

pub fn get_composer_from_arg(composer: &Option<String>) -> String {
    if let Some(path) = composer {
        tilde(&path).to_string()
    } else {
        Config::get_composer()
    }
}

pub fn get_template_from_arg(template: &Option<Template>) -> Template {
    if let Some(template) = template {
        template.to_owned()
    } else {
        Config::get_template()
    }
}

pub fn get_scores_directory_from_arg(
    scores_directory: &Option<String>,
) -> String {
    if let Some(path) = scores_directory {
        tilde(&path).to_string()
    } else {
        Config::get_scores_directory()
    }
}

pub fn get_pdfs_directory_from_arg(pdfs_directory: &Option<String>) -> String {
    if let Some(path) = pdfs_directory {
        tilde(&path).to_string()
    } else {
        Config::get_pdfs_directory()
    }
}
