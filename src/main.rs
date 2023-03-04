use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand)]
enum Commands {
    /// does testing things
    Clean {
        /// Remove pdf(s)
        score: String,
    },
    Compile {
        /// Create pdf(s)
        score: String,
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
        _ => {
            println!("Please choose a command.")
        }
    }
}
