use anyhow::Result;

use crate::adapter::{AdapterEvent, BrainAdapter};

pub struct OpenCodeAdapter;

impl BrainAdapter for OpenCodeAdapter {
    fn name(&self) -> &'static str {
        "opencode"
    }

    fn detect(&self) -> bool {
        true
    }

    fn checkpoint_event(&self, note: &str) -> Result<AdapterEvent> {
        Ok(AdapterEvent {
            tool: self.name().to_string(),
            action: "checkpoint".to_string(),
            note: note.to_string(),
        })
    }
}
