use std::path::Path;

use anyhow::Result;

pub fn run(_cwd: &Path) -> Result<()> {
    println!("brain install complete");
    println!("next: run `brain init` in your repository");
    Ok(())
}
