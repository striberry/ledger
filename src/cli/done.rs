use anyhow::{Result, bail};

use crate::core::Ledger;
use crate::storage;

pub fn run(ledger: &mut Ledger, id: u64) -> Result<()> {
    // Extract the text while the borrow from mark_done is still live,
    // then release it so we can pass ledger to save().
    let text = ledger.mark_done(id).map(|t| t.text.clone());

    match text {
        Some(text) => {
            println!("Done [#{}] {}", id, text);
            storage::save(ledger)
        }
        None => bail!("No open task with ID {id}."),
    }
}
