pub mod clean;
pub mod compile;
pub mod create;
pub mod edit;
pub mod list;
pub mod open;
mod patterns;
mod table;
pub mod templates;

use crate::commands::templates::Template;
use clap::Subcommand;

#[derive(Subcommand)]
pub enum Commands {
    /// Remove pdf(s)
    Clean { scores: Vec<String> },

    /// Create pdf(s)
    Compile {
        scores: Vec<String>,

        #[arg(long)]
        pdfs_directory: Option<String>,
    },

    /// Display config
    Config {
        key: Option<String>,

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
    },

    /// Open pdf(s)
    Open { scores: Vec<String> },

    /// List template types
    Templates,
}
