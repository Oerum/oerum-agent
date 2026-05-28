use brain_core::BrainStore;
use tempfile::tempdir;

#[test]
fn init_and_resume_are_deterministic() {
    let dir = tempdir().expect("tempdir");
    let store = BrainStore::new(dir.path());
    store.init(None).expect("init");
    let brief = store.resume().expect("resume");
    assert!(brief.active_task.is_none());
    assert!(!brief.next_actions.is_empty());
}

#[test]
fn checkpoint_surfaces_note_in_resume_brief() {
    let dir = tempdir().expect("tempdir");
    let store = BrainStore::new(dir.path());
    store.init(None).expect("init");
    store
        .checkpoint("decided to use Rust workspace".to_string())
        .expect("checkpoint");
    let brief = store.resume().expect("resume");
    assert!(
        brief
            .top_decisions
            .iter()
            .any(|d| d.contains("decided to use Rust workspace")),
        "expected checkpoint note in top_decisions, got: {:?}",
        brief.top_decisions
    );
}

#[test]
fn snapshot_write_is_atomic_across_repeated_writes() {
    let dir = tempdir().expect("tempdir");
    let store = BrainStore::new(dir.path());
    store.init(None).expect("init");
    for i in 0..10 {
        store.checkpoint(format!("note {i}")).expect("checkpoint");
        let brief = store.resume().expect("resume");
        assert!(!brief.top_decisions.is_empty(), "iteration {i}");
    }
}
