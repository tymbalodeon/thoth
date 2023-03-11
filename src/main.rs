use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(about, long_about = None, version)]
struct Cli {
    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand)]
enum Commands {
    /// Remove pdf(s)
    Clean { score: String },

    /// Create pdf(s)
    Compile { score: String },

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

        Some(Commands::Create {
            composer,
            template,
            title,
            edit: _,
        }) => {
            println!(
                "Creating ly file for \"{}\" by {} of type {}...",
                title, composer, template
            )
        }

        _ => {
            println!("Please choose a command.")
        }
    }
}
