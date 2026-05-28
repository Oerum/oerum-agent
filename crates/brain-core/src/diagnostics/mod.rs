use std::path::Path;

use serde::{Deserialize, Serialize};

use crate::state::BrainStore;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum DiagnosticStatus {
    Ok,
    Warn,
    Fail,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DiagnosticResult {
    pub check: String,
    pub status: DiagnosticStatus,
    pub detail: String,
    pub remediation: Option<String>,
}

fn ok(check: &str, detail: String) -> DiagnosticResult {
    DiagnosticResult {
        check: check.to_string(),
        status: DiagnosticStatus::Ok,
        detail,
        remediation: None,
    }
}

fn warn(check: &str, detail: String, remediation: &str) -> DiagnosticResult {
    DiagnosticResult {
        check: check.to_string(),
        status: DiagnosticStatus::Warn,
        detail,
        remediation: Some(remediation.to_string()),
    }
}

fn fail(check: &str, detail: String, remediation: &str) -> DiagnosticResult {
    DiagnosticResult {
        check: check.to_string(),
        status: DiagnosticStatus::Fail,
        detail,
        remediation: Some(remediation.to_string()),
    }
}

pub fn run_diagnostics(repo_root: &Path) -> Vec<DiagnosticResult> {
    let brain_dir = repo_root.join(".brain");
    let mut out = Vec::new();

    if !brain_dir.exists() {
        out.push(warn(
            "brain directory",
            ".brain missing".to_string(),
            "Run `brain init` in this repository.",
        ));
        return out;
    }
    out.push(ok("brain directory", ".brain exists".to_string()));

    let snapshot_path = brain_dir.join("snapshot.json");
    if !snapshot_path.exists() {
        out.push(fail(
            "snapshot",
            "snapshot.json missing".to_string(),
            "Run `brain init` to seed the snapshot.",
        ));
    } else {
        match BrainStore::new(repo_root).load_snapshot() {
            Ok(s) => out.push(ok(
                "snapshot",
                format!(
                    "schema v{}, {} decisions, last_event_seq={}",
                    s.schema_version,
                    s.decisions.len(),
                    s.last_event_seq
                ),
            )),
            Err(e) => out.push(fail(
                "snapshot",
                format!("snapshot unreadable: {e}"),
                "Restore from .brain/events/ or re-init the repository.",
            )),
        }
    }

    let events_dir = brain_dir.join("events");
    if events_dir.exists() {
        let count = std::fs::read_dir(&events_dir).map(|d| d.count()).unwrap_or(0);
        out.push(ok("event log", format!("{count} events on disk")));
    } else {
        out.push(warn(
            "event log",
            ".brain/events missing".to_string(),
            "Run `brain init` or any state-mutating command.",
        ));
    }

    let sync_cfg = brain_dir.join("sync.json");
    if sync_cfg.exists() {
        if std::env::var("BRAIN_SYNC_PASSPHRASE").is_ok() {
            out.push(ok(
                "sync",
                "configured; BRAIN_SYNC_PASSPHRASE is set".to_string(),
            ));
        } else {
            out.push(warn(
                "sync",
                "configured but BRAIN_SYNC_PASSPHRASE is not set".to_string(),
                "Export BRAIN_SYNC_PASSPHRASE before running `brain sync`.",
            ));
        }
    } else {
        out.push(ok("sync", "local mode (no sync.json)".to_string()));
    }

    out
}
