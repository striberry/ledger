use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Task {
    pub id: u64,
    pub text: String,
    pub status: Status,
    pub created_at: i64,
    pub completed_at: Option<i64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Status {
    Open,
    Done,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Ledger {
    pub next_id: u64,
    pub tasks: Vec<Task>,
}

// tests below!! nothing else!!

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn task_creation_works() {
        let task = Task {
            id: 1,
            text: "hello".to_string(),
            status: Status::Open,
            created_at: 0,
            completed_at: None,
        };

        assert_eq!(task.id, 1);
    }

    #[test]
    fn ledger_initial_state() {
        let ledger = Ledger {
            next_id: 1,
            tasks: vec![],
        };

        assert_eq!(ledger.next_id, 1);
        assert!(ledger.tasks.is_empty());
    }
}
