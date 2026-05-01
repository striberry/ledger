use anyhow::Result;
use std::{fs, path::PathBuf};

use crate::core::Ledger;

pub fn ledger_path() -> Result<PathBuf> {
    let mut path = dirs::home_dir().ok_or_else(|| anyhow::anyhow!("home directory not found"))?;

    path.push(".ledger");
    fs::create_dir_all(&path)?;

    path.push("ledger.json");
    Ok(path)
}

pub fn load() -> Result<Ledger> {
    let path = ledger_path()?;

    if !path.exists() {
        return Ok(Ledger {
            next_id: 1,
            tasks: vec![],
        });
    }

    let data = fs::read_to_string(path)?;
    let ledger: Ledger = serde_json::from_str(&data)?;

    Ok(ledger)
}

pub fn save(ledger: &Ledger) -> Result<()> {
    let path = ledger_path()?;

    let data = serde_json::to_string_pretty(ledger)?;
    fs::write(path, data)?;

    Ok(())
}

// tests below!! nothing else!!

#[cfg(test)]
mod tests {
    use super::*;
    use crate::core::{Ledger, Status, Task};
    use std::fs;

    fn sample_ledger() -> Ledger {
        Ledger {
            next_id: 2,
            tasks: vec![Task {
                id: 1,
                text: "test task".to_string(),
                status: Status::Open,
                created_at: 0,
                completed_at: None,
            }],
        }
    }

    #[test]
    fn serialize_deserialize_roundtrip() {
        let ledger = sample_ledger();

        let json = serde_json::to_string(&ledger).unwrap();
        let parsed: Ledger = serde_json::from_str(&json).unwrap();

        assert_eq!(parsed.next_id, ledger.next_id);
        assert_eq!(parsed.tasks.len(), 1);
    }

    #[test]
    fn save_and_load_temp_file() {
        let dir = std::env::temp_dir().join("ledger_test");
        let file = dir.join("ledger.json");

        fs::create_dir_all(&dir).unwrap();

        let ledger = sample_ledger();

        let json = serde_json::to_string_pretty(&ledger).unwrap();
        fs::write(&file, json).unwrap();

        let data = fs::read_to_string(&file).unwrap();
        let loaded: Ledger = serde_json::from_str(&data).unwrap();

        assert_eq!(loaded.next_id, 2);

        fs::remove_dir_all(&dir).ok();
    }
}
