mod commands;
mod config;

use crate::commands::edit::edit_main;
use crate::commands::Command;
use clap::Parser;
use commands::clean::clean_main;
use commands::compile::compile_main;
use commands::config::config_main;
use commands::create::create_main;
use commands::list::list_main;
use commands::open::open_main;
use commands::templates::templates_main;
use std::println;

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
        Some(Command::Clean { scores }) => clean_main(scores),
        Some(Command::Compile {
            scores,
            pdfs_directory,
        }) => compile_main(scores, pdfs_directory),
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
            pdfs_directory,
        }) => {
            create_main(
                title,
                subtitle,
                composer,
                arranger,
                template,
                instrument,
                edit,
                pdfs_directory,
            );
        }
        Some(Command::Edit {
            score,
            pdfs_directory,
        }) => {
            edit_main(score, pdfs_directory);
        }
        Some(Command::List {
            scores,
            outdated,
            compiled,
        }) => list_main(scores, outdated, compiled),
        Some(Command::Open { scores }) => {
            open_main(scores);
        }
        Some(Command::Templates { command }) => templates_main(command),
        _ => {
            println!("Please choose a command.")
        }
    }
}
