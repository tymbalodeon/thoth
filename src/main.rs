mod commands;
mod config;

use std::println;

use clap::Parser;
use commands::activate::activate_main;
use commands::clean::clean_main;
use commands::compile::compile_main;
use commands::config::config_main;
use commands::create::{create_main, ScoreFileSettings};
use commands::edit::edit_main;
use commands::helpers::helpers_main;
use commands::info::info_main;
use commands::lilypond::lilypond_main;
use commands::list::list_main;
use commands::open::open_main;
use commands::sketch::sketch_main;
use commands::templates::templates_main;
use commands::update_path::update_path_main;
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
        Some(Command::Activate) => activate_main(),
        Some(Command::UpdatePath) => update_path_main().unwrap(),
        Some(Command::Clean {
            search_terms,
            artist,
            title,
            all,
            scores_directory,
            pdfs_directory,
        }) => clean_main(
            search_terms,
            artist,
            title,
            all,
            scores_directory,
            pdfs_directory,
        ),
        Some(Command::Compile {
            search_terms,
            artist,
            title,
            all,
            scores_directory,
            pdfs_directory,
        }) => compile_main(
            search_terms,
            artist,
            title,
            all,
            scores_directory,
            pdfs_directory,
        ),
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
                title: title.to_owned(),
                subtitle: subtitle.to_owned(),
                composer: composer.to_owned(),
                arranger: arranger.to_owned(),
                template: template.to_owned(),
                instrument: instrument.to_owned(),
            };

            create_main(
                settings,
                edit,
                &false,
                scores_directory,
                pdfs_directory,
            );
        }
        Some(Command::Edit {
            search_terms,
            artist,
            title,
            all,
            scores_directory,
            pdfs_directory,
        }) => {
            edit_main(
                search_terms,
                artist,
                title,
                all,
                &false,
                scores_directory,
                pdfs_directory,
            );
        }
        Some(Command::Info {
            search_term,
            artist,
            title,
            all,
            scores_directory,
        }) => {
            info_main(search_term, artist, title, all, scores_directory);
        }
        Some(Command::Lilypond { version, command }) => {
            lilypond_main(version, command);
        }
        Some(Command::List {
            search_terms,
            outdated,
            compiled,
            artist,
            title,
            scores_directory,
            pdfs_directory,
        }) => {
            list_main(
                search_terms,
                outdated,
                compiled,
                artist,
                title,
                scores_directory,
                pdfs_directory,
            );
        }
        Some(Command::Open {
            search_terms,
            artist,
            title,
            all,
            file_type,
            scores_directory,
            pdfs_directory,
        }) => {
            open_main(
                search_terms,
                artist,
                title,
                all,
                file_type,
                scores_directory,
                pdfs_directory,
            );
        }
        Some(Command::Templates { command }) => templates_main(command),
        Some(Command::Helpers { command }) => helpers_main(command),
        Some(Command::Sketch {}) => sketch_main(),
        _ => {
            println!("Please choose a command.")
        }
    }
}
