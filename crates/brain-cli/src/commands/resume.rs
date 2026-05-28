use std::path::Path;

use anyhow::Result;
use brain_core::BrainStore;

pub fn run(cwd: &Path) -> Result<()> {
    let store = BrainStore::new(cwd);
    let brief = store.resume()?;
    println!(
        "active task: {}",
        brief.active_task.unwrap_or_else(|| "<none>".to_string())
    );
    println!("decisions:");
    for d in brief.top_decisions {
        println!("- {d}");
    }
    println!("next actions:");
    for a in brief.next_actions {
        println!("- {a}");
    }
    Ok(())
}
