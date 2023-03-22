mod config;

use clap::{Parser, Subcommand};
use config::Config;
use shellexpand::tilde;
use std::borrow::Cow;
use walkdir::{DirEntry, WalkDir};

#[derive(Subcommand)]
enum Commands {
    /// Remove pdf(s)
    Clean { score: String },

    /// Create pdf(s)
    Compile { score: String },

    /// Show config
    Config,

    /// Create new score template
    Create {
        #[arg(long)]
        composer: String,

        #[arg(long)]
        template: String,

        #[arg(long)]
        title: String,

        #[arg(long)]
        edit: bool,
    },

    /// Open <score> in editor and pdf viewer, recompiling on file changes.
    Edit { score: String },

    /// List pdf(s).
    List { scores: Vec<String> },

    /// Open pdf(s)
    Open { scores: Vec<String> },

    /// List template types
    Templates,
}

#[derive(Parser)]
#[command(about, long_about = None)]
#[command(version)]
#[command(arg_required_else_help(true))]
struct Cli {
    #[command(subcommand)]
    command: Option<Commands>,
}

fn is_hidden(entry: &DirEntry) -> bool {
    entry
        .file_name()
        .to_str()
        .map(|name| name.starts_with('.'))
        .unwrap_or(false)
}

fn print_contents(directory: Cow<str>) {
    println!("Default scores directory: {directory}");

    let directory_walker = WalkDir::new(directory.as_ref()).into_iter();

    for entry in directory_walker.filter_entry(|entry| !is_hidden(entry)) {
        println!("\t{}", entry.unwrap().path().display());
    }
}

fn main() {
    let cli = Cli::parse();

    match &cli.command {
        Some(Commands::Clean { score }) => {
            println!("Removing {}.pdf...", score)
        }
        Some(Commands::Compile { score }) => {
            println!("Creating {}.pdf...", score)
        }

        Some(Commands::Config) => {
            let config: Config = Config::new();
            let scores_directory = tilde(&config.scores_directory);

            print_contents(scores_directory);
        }

        Some(Commands::Create {
            composer,
            template,
            title,
            edit: _,
        }) => {
            println!("Creating ly file for \"{title}\" by {composer} of type {template}...")
        }

        Some(Commands::Edit { score }) => {
            println!("Opening {}.pdf for editing...", score)
        }

        Some(Commands::List { scores }) => {
            println!("Available scores...");
            for score in scores {
                println!("{score}")
            }
        }

        Some(Commands::Open { scores }) => {
            println!("Opening scores...");
            for score in scores {
                println!("{score}")
            }
        }

        Some(Commands::Templates) => {
            println!("Listing templates...");
            println!(
                "    form    # Form chart with separate sections \
    and form summary at the bottom.
    lead    # Lead sheet showing melody and chords.
    piano    # Piano staff score.
    single    # Score for a single staff instrument."
            )
        }

        _ => {
            println!("Please choose a command.")
        }
    }
}
