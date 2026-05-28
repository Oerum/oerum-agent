use brain_adapters::{claude::ClaudeAdapter, cursor::CursorAdapter, BrainAdapter};
use brain_core::BrainStore;
use tempfile::tempdir;

#[test]
fn cursor_to_claude_checkpoint_continuity() {
    let dir = tempdir().expect("tempdir");
    let store = BrainStore::new(dir.path());
    store.init(None).expect("init");

    let cursor = CursorAdapter;
    let claude = ClaudeAdapter;
    let c_event = cursor
        .checkpoint_event("start in cursor")
        .expect("cursor event");
    let q_event = claude
        .checkpoint_event("continue in claude")
        .expect("claude event");

    c_event.persist(&store).expect("record cursor event");
    q_event.persist(&store).expect("record claude event");

    let brief = store.resume().expect("resume");
    assert!(
        brief
            .top_decisions
            .iter()
            .any(|d| d.contains("cursor") && d.contains("start in cursor")),
        "expected cursor note in top_decisions, got: {:?}",
        brief.top_decisions
    );
    assert!(
        brief
            .top_decisions
            .iter()
            .any(|d| d.contains("claude") && d.contains("continue in claude")),
        "expected claude note in top_decisions, got: {:?}",
        brief.top_decisions
    );
}
