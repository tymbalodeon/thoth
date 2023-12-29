mod commands;
mod config;

use std::println;

use clap::Parser;
use commands::activate;
use commands::clean;
use commands::compile;
use commands::config as config_command;
use commands::create::{self, ScoreFileSettings};
use commands::edit;
use commands::helpers;
use commands::info;
use commands::lilypond;
use commands::list;
use commands::open;
use commands::sketch;
use commands::templates;
use commands::update_path;
use commands::update_version;
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
    match &Cli::parse().command {
        Some(Command::Activate { shell }) => activate::main(shell),
        Some(Command::UpdatePath { shell, version }) => {
            update_path::main(shell, version)
        }
        Some(Command::UpdateVersion {
            search_terms,
            version,
            artist,
            title,
            all,
            scores_directory,
        }) => update_version::main(
            search_terms,
            version,
            *artist,
            *title,
            *all,
            scores_directory,
        ),
        Some(Command::Clean {
            search_terms,
            artist,
            title,
            all,
            scores_directory,
            pdfs_directory,
        }) => clean::main(
            search_terms,
            *artist,
            *title,
            *all,
            scores_directory,
            pdfs_directory,
        ),
        Some(Command::Compile {
            search_terms,
            artist,
            title,
            all,
            lilypond_version,
            scores_directory,
            pdfs_directory,
            force,
        }) => compile::main(
            search_terms,
            *artist,
            *title,
            *all,
            lilypond_version,
            scores_directory,
            pdfs_directory,
            *force,
        ),
        Some(Command::Config {
            edit,
            path,
            key,
            set,
        }) => config_command::main(*edit, *path, key, set),
        Some(Command::Create {
            title,
            subtitle,
            composer,
            arranger,
            instrument,
            template,
            edit,
            lilypond_version,
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

            create::main(
                &settings,
                *edit,
                false,
                lilypond_version,
                scores_directory,
                pdfs_directory,
            );
        }
        Some(Command::Edit {
            search_terms,
            artist,
            title,
            all,
            lilypond_version,
            scores_directory,
            pdfs_directory,
        }) => {
            edit::main(
                search_terms,
                *artist,
                *title,
                *all,
                false,
                lilypond_version,
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
            info::main(search_term, *artist, *title, *all, scores_directory);
        }
        Some(Command::Lilypond { version, command }) => {
            lilypond::main(version, command);
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
            list::main(
                search_terms,
                *outdated,
                *compiled,
                *artist,
                *title,
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
            open::main(
                search_terms,
                *artist,
                *title,
                *all,
                file_type,
                scores_directory,
                pdfs_directory,
            );
        }
        Some(Command::Templates { command }) => templates::main(command),
        Some(Command::Helpers { command }) => helpers::main(command),
        Some(Command::Sketch { lilypond_version }) => {
            sketch::main(lilypond_version);
        }
        _ => {
            println!("Please choose a command.");
        }
    }
}
