use std::path::Path;

use anyhow::Result;
use brain_core::BrainStore;

pub fn decision(cwd: &Path, value: String) -> Result<()> {
    let store = BrainStore::new(cwd);
    let result = store.record_decision(value)?;
    println!("{}", result.message);
    Ok(())
}

pub fn task(cwd: &Path, value: Option<String>) -> Result<()> {
    let store = BrainStore::new(cwd);
    let result = store.record_task(value)?;
    println!("{}", result.message);
    Ok(())
}

pub fn artifact(cwd: &Path, value: String) -> Result<()> {
    let store = BrainStore::new(cwd);
    let result = store.record_artifact(value)?;
    println!("{}", result.message);
    Ok(())
}
