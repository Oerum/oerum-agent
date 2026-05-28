use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StateSnapshot {
    pub schema_version: u32,
    pub active_task: Option<String>,
    pub decisions: Vec<String>,
    pub artifacts: Vec<String>,
    pub git_head: Option<String>,
    pub updated_at: DateTime<Utc>,
    /// Highest event sequence number materialised into this snapshot.
    /// Used to avoid an O(N) scan of the events directory on every append.
    #[serde(default)]
    pub last_event_seq: u64,
}

/// Maximum number of decisions/artifacts retained in a snapshot before
/// older entries are trimmed. The full history remains in the event log.
pub const SNAPSHOT_HISTORY_CAP: usize = 100;

impl Default for StateSnapshot {
    fn default() -> Self {
        Self {
            schema_version: 1,
            active_task: None,
            decisions: Vec::new(),
            artifacts: Vec::new(),
            git_head: None,
            updated_at: Utc::now(),
            last_event_seq: 0,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ResumeBrief {
    pub active_task: Option<String>,
    pub top_decisions: Vec<String>,
    pub next_actions: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CommandResult {
    pub message: String,
    pub trace_id: String,
}
