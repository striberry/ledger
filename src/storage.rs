use anyhow::{Context, Result};
use std::{fs, path::PathBuf};

use crate::core::Ledger;

fn ledger_path() -> Result<PathBuf> {
    let mut path = dirs::home_dir()
        .ok_or_else(|| anyhow::anyhow!("could not determine the home directory"))?;
    path.push(".ledger");
    fs::create_dir_all(&path).context("failed to create ~/.ledger directory")?;
    path.push("ledger.json");
    Ok(path)
}

pub fn load() -> Result<Ledger> {
    let path = ledger_path()?;

    if !path.exists() {
        return Ok(Ledger::new());
    }

    let data =
        fs::read_to_string(&path).with_context(|| format!("failed to read {}", path.display()))?;

    serde_json::from_str(&data).context("ledger file is corrupt or has an unexpected format")
}

pub fn save(ledger: &Ledger) -> Result<()> {
    let path = ledger_path()?;

    let data = serde_json::to_string_pretty(ledger).context("failed to serialize ledger")?;

    fs::write(&path, data).with_context(|| format!("failed to write {}", path.display()))
}

#[cfg(test)]
mod tests {
    use crate::core::{Ledger, Status};

    fn make_ledger() -> Ledger {
        let mut l = Ledger::new();
        l.add("test task".to_string());
        l
    }

    #[test]
    fn serialize_deserialize_roundtrip() {
        let ledger = make_ledger();

        let json = serde_json::to_string(&ledger).unwrap();
        let parsed: Ledger = serde_json::from_str(&json).unwrap();

        assert_eq!(parsed.all_tasks().len(), 1);
        assert_eq!(parsed.all_tasks()[0].text, "test task");
    }

    #[test]
    fn roundtrip_preserves_done_status() {
        let mut ledger = make_ledger();
        let id = ledger.all_tasks()[0].id;
        ledger.mark_done(id);

        let json = serde_json::to_string_pretty(&ledger).unwrap();
        let loaded: Ledger = serde_json::from_str(&json).unwrap();

        assert_eq!(loaded.all_tasks()[0].status, Status::Done);
        assert!(loaded.all_tasks()[0].completed_at.is_some());
    }

    #[test]
    fn roundtrip_after_remove() {
        let mut ledger = make_ledger();
        let id = ledger.all_tasks()[0].id;
        ledger.remove(id);

        let json = serde_json::to_string(&ledger).unwrap();
        let loaded: Ledger = serde_json::from_str(&json).unwrap();

        assert!(loaded.all_tasks().is_empty());
    }
}
