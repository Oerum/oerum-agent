use std::path::Path;

use anyhow::Result;
use brain_core::{git::link::git_head, BrainStore};

pub fn run(cwd: &Path) -> Result<()> {
    let store = BrainStore::new(cwd);
    let result = store.init(git_head(cwd))?;
    println!("{}", result.message);
    Ok(())
}
