use brain_core::BrainStore;
use std::fs;
use tempfile::tempdir;

#[test]
fn cross_machine_continuity_simulated_by_snapshot_copy() {
    let machine_a = tempdir().expect("tmp a");
    let machine_b = tempdir().expect("tmp b");

    let a = BrainStore::new(machine_a.path());
    a.init(None).expect("init");
    a.checkpoint("work from machine A".to_string())
        .expect("checkpoint");

    let a_snapshot = machine_a.path().join(".brain").join("snapshot.json");
    let b_dir = machine_b.path().join(".brain");
    fs::create_dir_all(&b_dir).expect("mkdir");
    fs::copy(a_snapshot, b_dir.join("snapshot.json")).expect("copy snapshot");
    fs::create_dir_all(b_dir.join("events")).expect("events");
    fs::create_dir_all(b_dir.join("locks")).expect("locks");

    let b = BrainStore::new(machine_b.path());
    let brief = b.resume().expect("resume");
    assert!(
        brief
            .top_decisions
            .iter()
            .any(|d| d.contains("work from machine A")),
        "expected note from machine A to survive snapshot transfer"
    );
}
