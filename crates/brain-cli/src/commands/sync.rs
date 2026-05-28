use std::path::Path;

use anyhow::{Context, Result};
use brain_sync::{config::SyncConfig, crypto::encrypt, s3_backend::upload_blob};

pub async fn run(cwd: &Path) -> Result<()> {
    let cfg_path = cwd.join(".brain").join("sync.json");
    if !cfg_path.exists() {
        println!("sync not configured. local mode is active.");
        return Ok(());
    }
    let cfg: SyncConfig = serde_json::from_slice(&std::fs::read(&cfg_path)?)
        .with_context(|| format!("parsing sync config at {}", cfg_path.display()))?;
    let snapshot_path = cwd.join(".brain").join("snapshot.json");
    let payload = std::fs::read(&snapshot_path)
        .with_context(|| format!("reading snapshot at {}", snapshot_path.display()))?;
    let passphrase = std::env::var("BRAIN_SYNC_PASSPHRASE")
        .context("BRAIN_SYNC_PASSPHRASE must be set for secure sync")?;
    let encrypted = encrypt(&payload, &passphrase)?;
    // upload_blob currently bails — see ADR-003. Propagating the error means
    // operators see "not implemented" instead of a fake "sync complete".
    upload_blob(&cfg, "snapshot.enc", encrypted).await?;
    println!("sync complete");
    Ok(())
}
