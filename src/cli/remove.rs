use anyhow::{Result, bail};

use crate::core::Ledger;
use crate::storage;

pub fn run(ledger: &mut Ledger, id: u64) -> Result<()> {
    match ledger.remove(id) {
        Some(task) => {
            println!("Removed [#{}] {}", task.id, task.text);
            storage::save(ledger)
        }
        None => bail!("No task with ID {id}."),
    }
}
