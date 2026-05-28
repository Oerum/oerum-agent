use std::path::Path;

use anyhow::Result;
use brain_core::BrainStore;

pub fn run(cwd: &Path, note: String) -> Result<()> {
    let store = BrainStore::new(cwd);
    let result = store.checkpoint(note)?;
    println!("{}", result.message);
    Ok(())
}
