use anyhow::Result;
use brain_core::{AdapterEventRef, BrainStore, CommandResult};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AdapterEvent {
    pub tool: String,
    pub action: String,
    pub note: String,
}

impl AdapterEvent {
    pub fn borrow(&self) -> AdapterEventRef<'_> {
        AdapterEventRef {
            tool: self.tool.as_str(),
            action: self.action.as_str(),
            note: self.note.as_str(),
        }
    }

    /// Persist this event into the given store as a decision-typed entry.
    pub fn persist(&self, store: &BrainStore) -> Result<CommandResult> {
        store.record_adapter_event(&self.borrow())
    }
}

pub trait BrainAdapter: Send + Sync {
    fn name(&self) -> &'static str;
    fn detect(&self) -> bool;
    fn checkpoint_event(&self, note: &str) -> Result<AdapterEvent>;
}
