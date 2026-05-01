use anyhow::Result;
use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(name = "ledger")]
#[command(about = "A lightweight local CLI task tracker", long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    Add { text: String },
    List,
    Done { id: u64 },
    Remove { id: u64 },
    History,
}

fn main() -> Result<()> {
    let cli = Cli::parse();

    match cli.command {
        Commands::Add { text } => {
            println!("ADD: {text}");
        }

        Commands::List => {
            println!("LIST");
        }

        Commands::Done { id } => {
            println!("DONE: {id}");
        }

        Commands::Remove { id } => {
            println!("REMOVE: {id}");
        }

        Commands::History => {
            println!("HISTORY");
        }
    }

    Ok(())
}
