use std::fmt;

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum Status {
    Open,
    Done,
}

impl fmt::Display for Status {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Status::Open => f.write_str("open"),
            Status::Done => f.write_str("done"),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Task {
    pub id: u64,
    pub text: String,
    pub status: Status,
    pub created_at: i64,
    pub completed_at: Option<i64>,
}

impl Task {
    pub fn is_open(&self) -> bool {
        self.status == Status::Open
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Ledger {
    next_id: u64,
    tasks: Vec<Task>,
}

impl Default for Ledger {
    fn default() -> Self {
        Self::new()
    }
}

impl Ledger {
    pub fn new() -> Self {
        Self {
            next_id: 1,
            tasks: Vec::new(),
        }
    }

    /// Append a new open task and return a reference to it.
    pub fn add(&mut self, text: String) -> &Task {
        let task = Task {
            id: self.next_id,
            text,
            status: Status::Open,
            created_at: now_secs(),
            completed_at: None,
        };
        self.next_id += 1;
        self.tasks.push(task);
        self.tasks.last().unwrap()
    }

    /// Mark an open task as done and return a reference to the updated task.
    /// Returns `None` when the id is missing or the task is already done.
    pub fn mark_done(&mut self, id: u64) -> Option<&Task> {
        let idx = self.tasks.iter().position(|t| t.id == id && t.is_open())?;
        self.tasks[idx].status = Status::Done;
        self.tasks[idx].completed_at = Some(now_secs());
        Some(&self.tasks[idx])
    }

    /// Permanently delete a task and return it.
    /// Returns `None` when the id is not found.
    pub fn remove(&mut self, id: u64) -> Option<Task> {
        let idx = self.tasks.iter().position(|t| t.id == id)?;
        Some(self.tasks.remove(idx))
    }

    /// Open tasks only, used by `list`.
    pub fn open_tasks(&self) -> Vec<&Task> {
        self.tasks.iter().filter(|t| t.is_open()).collect()
    }

    /// Every task in insertion order, used by `history`.
    pub fn all_tasks(&self) -> &[Task] {
        &self.tasks
    }
}

fn now_secs() -> i64 {
    use std::time::{SystemTime, UNIX_EPOCH};
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("system clock is before the Unix epoch")
        .as_secs() as i64
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new_ledger_is_empty() {
        let ledger = Ledger::new();
        assert!(ledger.all_tasks().is_empty());
        assert!(ledger.open_tasks().is_empty());
    }

    #[test]
    fn add_assigns_sequential_ids() {
        let mut ledger = Ledger::new();
        let id1 = ledger.add("first".to_string()).id;
        let id2 = ledger.add("second".to_string()).id;
        assert_eq!(id2, id1 + 1);
    }

    #[test]
    fn added_task_is_open() {
        let mut ledger = Ledger::new();
        let task = ledger.add("hello".to_string());
        assert!(task.is_open());
        assert_eq!(task.text, "hello");
    }

    #[test]
    fn mark_done_transitions_status() {
        let mut ledger = Ledger::new();
        let id = ledger.add("task".to_string()).id;
        let task = ledger.mark_done(id).expect("task should be found");
        assert_eq!(task.status, Status::Done);
        assert!(task.completed_at.is_some());
    }

    #[test]
    fn mark_done_is_safe_to_call_twice() {
        let mut ledger = Ledger::new();
        let id = ledger.add("task".to_string()).id;
        ledger.mark_done(id);
        assert!(
            ledger.mark_done(id).is_none(),
            "already-done task should return None"
        );
    }

    #[test]
    fn remove_deletes_task() {
        let mut ledger = Ledger::new();
        let id = ledger.add("task".to_string()).id;
        let removed = ledger.remove(id).expect("task should exist");
        assert_eq!(removed.id, id);
        assert!(ledger.all_tasks().is_empty());
    }

    #[test]
    fn open_tasks_excludes_done() {
        let mut ledger = Ledger::new();
        let id = ledger.add("first".to_string()).id;
        ledger.add("second".to_string());
        ledger.mark_done(id);
        let open = ledger.open_tasks();
        assert_eq!(open.len(), 1);
        assert_eq!(open[0].text, "second");
    }

    #[test]
    fn missing_id_returns_none() {
        let mut ledger = Ledger::new();
        assert!(ledger.mark_done(99).is_none());
        assert!(ledger.remove(99).is_none());
    }
}
