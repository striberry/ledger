use anyhow::Result;

use crate::core::Ledger;

pub fn run(ledger: &Ledger) -> Result<()> {
    let tasks = ledger.all_tasks();

    if tasks.is_empty() {
        println!("No tasks yet.");
        return Ok(());
    }

    let task_w = tasks.iter().map(|t| t.text.len()).max().unwrap_or(4).max(4);

    println!("{:>4}  {:<6}  Task", "ID", "Status");
    println!("{}", "─".repeat(4 + 2 + 6 + 2 + task_w));

    for task in tasks {
        println!("{:>4}  {:<6}  {}", task.id, task.status, task.text);
    }

    Ok(())
}
