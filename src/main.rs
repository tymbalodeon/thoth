mod commands;
mod config;

use std::println;

use clap::Parser;
use commands::clean::clean_main;
use commands::compile::compile_main;
use commands::config::config_main;
use commands::create::{create_main, ScoreFileSettings};
use commands::edit::edit_main;
use commands::helpers::helpers_main;
use commands::list::list_main;
use commands::open::open_main;
use commands::templates::templates_main;
use commands::Command;

#[derive(Parser)]
#[command(about, long_about = None)]
#[command(version)]
#[command(arg_required_else_help(true))]
struct Cli {
    #[command(subcommand)]
    command: Option<Command>,
}

fn main() {
    let cli = Cli::parse();

    match &cli.command {
        Some(Command::Clean {
            scores,
            pdfs_directory,
        }) => clean_main(scores, pdfs_directory),
        Some(Command::Compile {
            scores,
            scores_directory,
            pdfs_directory,
        }) => compile_main(scores, scores_directory, pdfs_directory),
        Some(Command::Config {
            edit,
            path,
            key,
            set,
        }) => config_main(edit, path, key, set),
        Some(Command::Create {
            title,
            subtitle,
            composer,
            arranger,
            instrument,
            template,
            edit,
            scores_directory,
            pdfs_directory,
        }) => {
            let settings = ScoreFileSettings {
                title,
                subtitle,
                composer,
                arranger,
                template,
                instrument,
            };

            create_main(settings, edit, scores_directory, pdfs_directory);
        }
        Some(Command::Edit {
            score,
            scores_directory,
            pdfs_directory,
        }) => {
            edit_main(score, scores_directory, pdfs_directory);
        }
        Some(Command::List {
            scores,
            outdated,
            compiled,
            artist,
            title,
            scores_directory,
            pdfs_directory,
        }) => list_main(
            scores,
            outdated,
            compiled,
            artist,
            title,
            scores_directory,
            pdfs_directory,
        ),
        Some(Command::Open {
            scores,
            file_type,
            scores_directory,
            pdfs_directory,
        }) => {
            open_main(scores, file_type, scores_directory, pdfs_directory);
        }
        Some(Command::Templates { command }) => templates_main(command),
        Some(Command::Helpers { command }) => helpers_main(command),
        _ => {
            println!("Please choose a command.")
        }
    }
}
