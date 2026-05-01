mod add;
mod done;
mod history;
mod list;
mod remove;

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
    /// Add a new task
    Add {
        /// Task description
        text: String,
    },
    /// List all open tasks
    List,
    /// Mark a task as done
    Done {
        /// Task ID
        id: u64,
    },
    /// Remove a task permanently
    Remove {
        /// Task ID
        id: u64,
    },
    /// Show every task including completed ones
    History,
}

pub fn run() -> Result<()> {
    let cli = Cli::parse();
    let mut ledger = crate::storage::load()?;

    match cli.command {
        Commands::Add { text } => add::run(&mut ledger, text)?,
        Commands::List => list::run(&ledger)?,
        Commands::Done { id } => done::run(&mut ledger, id)?,
        Commands::Remove { id } => remove::run(&mut ledger, id)?,
        Commands::History => history::run(&ledger)?,
    }

    Ok(())
}
