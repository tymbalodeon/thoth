mod commands;
mod config;

use clap::{Parser, Subcommand, ValueEnum};
use commands::create::create_score;
use commands::list::list_pdfs;
use config::{get_composer, Config};
use once_cell::sync::Lazy;

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
    Clean { score: String },

    /// Create pdf(s)
    Compile { score: String },

    /// Show config
    Config,

    /// Create new score template
    Create {
        #[arg(long, default_value_t = COMPOSER.to_string())]
        composer: String,

        #[arg(long)]
        arranger: Option<String>,

        #[arg(long, default_value_t = Template::Single, value_enum)]
        template: Template,

        #[arg(long)]
        title: String,

        #[arg(long)]
        subtitle: Option<String>,

        #[arg(long)]
        edit: bool,
    },

    /// Open <score> in editor and pdf viewer, recompiling on file changes.
    Edit { score: String },

    /// List pdf(s).
    List,

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
        Some(Commands::Clean { score }) => {
            println!("Removing {}.pdf...", score)
        }
        Some(Commands::Compile { score }) => {
            println!("Creating {}.pdf...", score)
        }

        Some(Commands::Config) => {
            let config: Config = Config::new();
            println!("{:?}", config);
        }

        Some(Commands::Create {
            composer,
            arranger: _,
            template,
            title,
            subtitle: _,
            edit: _,
        }) => {
            println!(
                "Creating ly file for \"{title}\" by {composer} of type {:?}...",
                template
            );
            create_score(template, composer, title);
        }

        Some(Commands::Edit { score }) => {
            println!("Opening {}.pdf for editing...", score)
        }

        Some(Commands::List) => list_pdfs(),

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
