mod commands;
mod config;

use std::println;

use clap::Parser;
use commands::clean::clean_main;
use commands::compile::compile_main;
use commands::config::config_main;
use commands::create::create_main;
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
            pdfs_directory,
        }) => list_main(scores, outdated, compiled, pdfs_directory),
        Some(Command::Open {
            scores,
            pdfs_directory,
        }) => {
            open_main(scores, pdfs_directory);
        }
        Some(Command::Templates { command }) => templates_main(command),
        Some(Command::Helpers { command }) => helpers_main(command),
        _ => {
            println!("Please choose a command.")
        }
    }
}
