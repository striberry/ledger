use anyhow::Result;

use crate::core::Ledger;

pub fn run(ledger: &Ledger) -> Result<()> {
    let tasks = ledger.open_tasks();

    if tasks.is_empty() {
        println!("No open tasks.");
        return Ok(());
    }

    let task_w = tasks.iter().map(|t| t.text.len()).max().unwrap_or(4).max(4);

    println!("{:>4}  Task", "ID");
    println!("{}", "─".repeat(4 + 2 + task_w));

    for task in tasks {
        println!("{:>4}  {}", task.id, task.text);
    }

    Ok(())
}
