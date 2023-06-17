#[macro_use]
extern crate prettytable;

mod commands;
mod config;

use crate::commands::edit::edit_main;
use crate::commands::Commands;
use clap::Parser;
use commands::clean::clean_main;
use commands::compile::compile_main;
use commands::create::create_main;
use commands::list::list_main;
use commands::open::open_main;
use commands::templates::templates_main;
use config::config_main;
use std::println;

pub fn add_value_to_string_if_some(
    mut string: String,
    key: &str,
    value: &Option<String>,
) -> String {
    if let Some(value) = value {
        let line = format!("{key} = \"{value}\"\n");
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
    command: Option<Commands>,
}

fn main() {
    let cli = Cli::parse();

    match &cli.command {
        Some(Commands::Clean { scores }) => clean_main(scores),
        Some(Commands::Compile { scores }) => compile_main(scores),
        Some(Commands::Config { edit, path, key }) => {
            config_main(edit, path, key)
        }
        Some(Commands::Create {
            title,
            subtitle,
            composer,
            arranger,
            instrument,
            template,
            edit,
        }) => {
            create_main(
                title, subtitle, composer, arranger, instrument, template,
                edit,
            );
        }
        Some(Commands::Edit { score }) => {
            edit_main(score);
        }
        Some(Commands::List { scores }) => list_main(scores),
        Some(Commands::Open { scores }) => {
            open_main(scores);
        }
        Some(Commands::Templates) => templates_main(),

        _ => {
            println!("Please choose a command.")
        }
    }
}
