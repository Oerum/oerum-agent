use anyhow::Result;

use crate::adapter::{AdapterEvent, BrainAdapter};

pub struct CopilotAdapter;

impl BrainAdapter for CopilotAdapter {
    fn name(&self) -> &'static str {
        "copilot"
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
