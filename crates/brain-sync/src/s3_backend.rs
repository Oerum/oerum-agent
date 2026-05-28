use anyhow::{bail, Result};

use crate::config::SyncConfig;

/// Upload an encrypted blob to S3.
///
/// **Not yet implemented.** The MVP intentionally fails loud rather than
/// silently returning success — a no-op upload would cause cross-machine
/// continuity to fail without any user-visible signal.
pub async fn upload_blob(_cfg: &SyncConfig, _key: &str, _bytes: Vec<u8>) -> Result<()> {
    bail!("brain-sync S3 upload is not yet implemented; see ADR-003 for the rollout plan")
}

/// Download an encrypted blob from S3.
///
/// **Not yet implemented.** See [`upload_blob`].
pub async fn download_blob(_cfg: &SyncConfig, _key: &str) -> Result<Vec<u8>> {
    bail!("brain-sync S3 download is not yet implemented; see ADR-003 for the rollout plan")
}
