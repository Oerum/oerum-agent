use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SyncConfig {
    pub bucket: String,
    pub prefix: String,
    pub region: String,
}
