use anyhow::Result;

use crate::core::Ledger;
use crate::storage;

pub fn run(ledger: &mut Ledger, text: String) -> Result<()> {
    let task = ledger.add(text);
    println!("Added [#{}] {}", task.id, task.text);
    storage::save(ledger)
}
