mod cli;
mod core;
mod storage;

use anyhow::Result;

fn main() -> Result<()> {
    cli::run()
}
