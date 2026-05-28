use std::fs;
use std::io::Write;
use std::path::{Path, PathBuf};

use anyhow::{Context, Result};
use chrono::Utc;
use fs4::FileExt;
use uuid::Uuid;
// NOTE: We pin to fs4 1.x with the `sync` feature. The trait method is
// `lock()` (renamed from `lock_exclusive()` in 1.0.0) and `FileExt` lives
// at the crate root rather than under `fs_std`.

use super::events::{EventKind, StateEvent};
use super::migrate::ensure_supported;
use super::schema::{CommandResult, ResumeBrief, StateSnapshot, SNAPSHOT_HISTORY_CAP};



#[derive(Debug, Clone)]
pub struct BrainStore {
    repo_root: PathBuf,
    brain_dir: PathBuf,
}

/// RAII guard that releases the exclusive file lock when dropped, including
/// on error paths.
struct LockGuard {
    file: fs::File,
}

impl Drop for LockGuard {
    fn drop(&mut self) {
        let _ = FileExt::unlock(&self.file);
    }
}

impl BrainStore {
    pub fn new(repo_root: impl AsRef<Path>) -> Self {
        let repo_root = repo_root.as_ref().to_path_buf();
        let brain_dir = repo_root.join(".brain");
        Self {
            repo_root,
            brain_dir,
        }
    }

    pub fn init(&self, git_head: Option<String>) -> Result<CommandResult> {
        fs::create_dir_all(self.brain_dir.join("events"))?;
        fs::create_dir_all(self.brain_dir.join("locks"))?;
        // Seed snapshot only if absent — repeated `brain init` is idempotent.
        if !self.brain_dir.join("snapshot.json").exists() {
            let snapshot = StateSnapshot {
                git_head: git_head.clone(),
                ..StateSnapshot::default()
            };
            self.write_snapshot(&snapshot)?;
        }
        self.with_state_mutation(|s| {
            s.git_head = git_head.clone();
            Ok(EventKind::Init { git_head })
        })?;
        Ok(CommandResult {
            message: "brain initialized".to_string(),
            trace_id: Uuid::new_v4().to_string(),
        })
    }

    pub fn checkpoint(&self, note: String) -> Result<CommandResult> {
        self.with_state_mutation(|s| {
            push_capped(&mut s.decisions, note.clone());
            Ok(EventKind::Checkpoint { note })
        })?;
        Ok(CommandResult {
            message: "checkpoint saved".to_string(),
            trace_id: Uuid::new_v4().to_string(),
        })
    }

    /// Record a decision made during the session. Surfaced first in the
    /// resume brief.
    pub fn record_decision(&self, value: String) -> Result<CommandResult> {
        self.with_state_mutation(|s| {
            push_capped(&mut s.decisions, value.clone());
            Ok(EventKind::Decision { value })
        })?;
        Ok(CommandResult {
            message: "decision recorded".to_string(),
            trace_id: Uuid::new_v4().to_string(),
        })
    }

    /// Set or replace the currently active task. Pass `None` to clear it.
    pub fn record_task(&self, value: Option<String>) -> Result<CommandResult> {
        self.with_state_mutation(|s| {
            s.active_task = value.clone();
            Ok(match value {
                Some(val) => EventKind::Task { value: val },
                None => EventKind::Task {
                    value: "<cleared>".to_string(),
                },
            })
        })?;
        Ok(CommandResult {
            message: "task updated".to_string(),
            trace_id: Uuid::new_v4().to_string(),
        })
    }

    /// Record an artefact reference (file path, URL, PR link, ...).
    pub fn record_artifact(&self, value: String) -> Result<CommandResult> {
        self.with_state_mutation(|s| {
            push_capped(&mut s.artifacts, value.clone());
            Ok(EventKind::Artifact { value })
        })?;
        Ok(CommandResult {
            message: "artifact recorded".to_string(),
            trace_id: Uuid::new_v4().to_string(),
        })
    }



    pub fn resume(&self) -> Result<ResumeBrief> {
        let snapshot = self.load_snapshot()?;
        let next_actions = if snapshot.active_task.is_some() {
            vec![
                "Continue active task".to_string(),
                "Create checkpoint after progress".to_string(),
            ]
        } else {
            vec!["Set current task with adapter checkpoint".to_string()]
        };
        Ok(ResumeBrief {
            active_task: snapshot.active_task,
            top_decisions: snapshot.decisions.into_iter().rev().take(3).collect(),
            next_actions,
        })
    }

    pub fn load_snapshot(&self) -> Result<StateSnapshot> {
        let path = self.brain_dir.join("snapshot.json");
        let raw = fs::read_to_string(&path)
            .with_context(|| format!("missing snapshot at {}", path.display()))?;
        let snapshot: StateSnapshot = serde_json::from_str(&raw)?;
        ensure_supported(snapshot.schema_version)?;
        Ok(snapshot)
    }

    pub fn repo_root(&self) -> &Path {
        &self.repo_root
    }

    // ---- internals ---------------------------------------------------------

    /// Run a state mutation under exclusive lock. The closure mutates the
    /// in-memory snapshot and returns the [`EventKind`] to append. The
    /// snapshot's `updated_at` and `last_event_seq` are advanced atomically
    /// with the event write.
    fn with_state_mutation<F>(&self, mutate: F) -> Result<u64>
    where
        F: FnOnce(&mut StateSnapshot) -> Result<EventKind>,
    {
        fs::create_dir_all(self.brain_dir.join("events"))?;
        fs::create_dir_all(self.brain_dir.join("locks"))?;

        let lock_file_path = self.brain_dir.join("locks").join("events.lock");
        let lock_file = fs::OpenOptions::new()
            .read(true)
            .write(true)
            .create(true)
            .truncate(false)
            .open(&lock_file_path)?;
        FileExt::lock(&lock_file)?;
        let _guard = LockGuard { file: lock_file };

        // Inside the lock: read → mutate → write. No other process can
        // interleave between read and write. A missing or corrupted snapshot
        // here would be a real bug — `init()` is responsible for ensuring
        // `.brain/snapshot.json` exists before any other mutation runs.
        let mut snapshot = self.load_snapshot().with_context(|| {
            "snapshot unreadable; run `brain init` or restore from .brain/events/"
        })?;
        let kind = mutate(&mut snapshot)?;

        let mut seq = snapshot.last_event_seq + 1;
        let events_dir = self.brain_dir.join("events");
        while events_dir.join(format!("{seq:08}.json")).exists() {
            seq += 1;
        }
        let event = StateEvent {
            seq,
            at: Utc::now(),
            kind,
        };
        self.atomic_write_event(&events_dir, seq, &event)?;

        snapshot.last_event_seq = seq;
        snapshot.updated_at = Utc::now();
        self.write_snapshot(&snapshot)?;

        Ok(seq)
    }

    /// Atomically write the snapshot via tmp-file + `rename`. `rename` over an
    /// existing file is atomic on all supported platforms, so an interrupted
    /// write leaves the previous snapshot intact rather than truncated.
    fn write_snapshot(&self, snapshot: &StateSnapshot) -> Result<()> {
        fs::create_dir_all(&self.brain_dir)?;
        let body = serde_json::to_vec_pretty(snapshot)?;
        let final_path = self.brain_dir.join("snapshot.json");
        let tmp_path = self
            .brain_dir
            .join(format!("snapshot.json.tmp.{}", Uuid::new_v4()));
        {
            let mut f = fs::File::create(&tmp_path)
                .with_context(|| format!("creating tmp snapshot at {}", tmp_path.display()))?;
            f.write_all(&body)?;
            f.sync_all()?;
        }
        fs::rename(&tmp_path, &final_path).with_context(|| {
            format!(
                "renaming {} -> {}",
                tmp_path.display(),
                final_path.display()
            )
        })?;
        Ok(())
    }

    fn atomic_write_event(
        &self,
        events_dir: &Path,
        seq: u64,
        event: &StateEvent,
    ) -> Result<()> {
        let event_path = events_dir.join(format!("{seq:08}.json"));
        let tmp_path = events_dir.join(format!("{seq:08}.json.tmp.{}", Uuid::new_v4()));
        {
            let mut f = fs::File::create(&tmp_path)?;
            f.write_all(&serde_json::to_vec_pretty(event)?)?;
            f.sync_all()?;
        }
        fs::rename(&tmp_path, &event_path)?;
        Ok(())
    }
}

fn push_capped(vec: &mut Vec<String>, value: String) {
    vec.push(value);
    if vec.len() > SNAPSHOT_HISTORY_CAP {
        let drop = vec.len() - SNAPSHOT_HISTORY_CAP;
        vec.drain(..drop);
    }
}
