use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum EventKind {
    Init { git_head: Option<String> },
    Checkpoint { note: String },
    Decision { value: String },
    Task { value: String },
    Artifact { value: String },
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StateEvent {
    pub seq: u64,
    pub at: DateTime<Utc>,
    pub kind: EventKind,
}
