#[macro_use]
extern crate prettytable;

mod commands;
mod config;

use clap::{Parser, Subcommand, ValueEnum};
use commands::clean::clean_pdfs;
use commands::compile::compile_pdfs;
use commands::create::create_score;
use commands::edit::edit_score;
use commands::list::list_scores;
use commands::open::open_pdf;
use config::{get_composer, Config};
use once_cell::sync::Lazy;
use prettytable::{format, Cell, Row, Table};
use std::path::PathBuf;
use std::println;

static COMPOSER: Lazy<String> = Lazy::new(get_composer);

#[derive(Debug, ValueEnum, Clone)]
pub enum Template {
    Form,
    Lead,
    Piano,
    Single,
}

#[derive(Subcommand)]
enum Commands {
    /// Remove pdf(s)
    Clean { scores: Vec<String> },

    /// Create pdf(s)
    Compile { scores: Vec<String> },

    /// Display config
    Config {
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

        #[arg(long, default_value_t = COMPOSER.to_owned())]
        composer: String,

        #[arg(long)]
        arranger: Option<String>,

        #[arg(long, default_value_t = Template::Single, value_enum)]
        template: Template,

        #[arg(long)]
        subtitle: Option<String>,

        /// Open for editing after creating
        #[arg(long)]
        edit: bool,
    },

    /// Open <score> in editor and pdf viewer, recompiling on file changes
    Edit { score: String },

    /// List pdf(s)
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

fn main() {
    let cli = Cli::parse();

    match &cli.command {
        Some(Commands::Clean { scores }) => clean_pdfs(scores),
        Some(Commands::Compile { scores }) => compile_pdfs(scores),
        Some(Commands::Config { edit, path }) => {
            if *edit {
                Config::edit();
            } else if *path {
                Config::display_path();
            } else {
                Config::display();
            }
        }

        Some(Commands::Create {
            composer,
            arranger: _,
            template,
            title,
            subtitle: _,
            edit,
        }) => {
            let files = create_score(template, composer, title);

            println!(
                "Created score for \"{title}\" by {composer} using template {:?}:",
                template
            );

            for file in files {
                println!("{file}");

                if *edit {
                    let file_stem = PathBuf::from(&file)
                        .file_stem()
                        .unwrap()
                        .to_str()
                        .unwrap()
                        .to_string();

                    edit_score(&file_stem)
                }
            }
        }

        Some(Commands::Edit { score }) => {
            compile_pdfs(&vec![score.to_string()]);
            edit_score(score);
        }

        Some(Commands::List { scores }) => list_scores(scores),

        Some(Commands::Open { scores }) => {
            open_pdf(scores);
        }

        Some(Commands::Templates) => {
            let mut table = Table::new();

            table.set_format(*format::consts::FORMAT_NO_BORDER_LINE_SEPARATOR);
            table.set_titles(row!["NAME", "DESCRIPTION"]);

            let values = vec![
                ["form", "Form chart with separate sections and form summary at the bottom."],
                ["lead", "Lead sheet showing melody and chords."],
                ["piano", "Piano staff score."],
                ["single", "Score for a single staff instrument."],
            ];

            for value in values {
                let cells: Vec<Cell> =
                    value.iter().map(|item| Cell::new(item)).collect();
                table.add_row(Row::new(cells));
            }

            println!();
            table.printstd();
        }

        _ => {
            println!("Please choose a command.")
        }
    }
}
