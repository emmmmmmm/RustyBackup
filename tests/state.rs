use rustybackup::state::{BackupState, LatestBackup, BackupStats, load_or_init_state};
use chrono::Local;
use tempfile::NamedTempFile;
use std::path::PathBuf;

#[test]
fn state_roundtrip() {
    let tmp = NamedTempFile::new().unwrap();
    let now = Local::now();
    let state = BackupState {
        latest: LatestBackup {
            timestamp: now,
            snapshot_id: "abc123".into(),
            destination: PathBuf::from("/tmp/backup"),
        },
        stats: vec![BackupStats {
            timestamp: now,
            files_synced: 1,
            bytes_copied: 2,
            duration_ms: 3,
        }],
    };

    state.save(tmp.path()).unwrap();
    let loaded = BackupState::load(tmp.path()).unwrap();

    assert_eq!(state.latest.snapshot_id, loaded.latest.snapshot_id);
    assert_eq!(state.latest.destination, loaded.latest.destination);
    assert_eq!(state.stats[0].files_synced, loaded.stats[0].files_synced);
    assert_eq!(state.stats[0].bytes_copied, loaded.stats[0].bytes_copied);
    assert_eq!(state.stats[0].duration_ms, loaded.stats[0].duration_ms);
    assert_eq!(state.latest.timestamp.timestamp(), loaded.latest.timestamp.timestamp());
}

#[test]
fn init_creates_default_state() {
    let dir = tempfile::tempdir().unwrap();
    let path = dir.path().join("state.toml");
    assert!(!path.exists());

    let state = load_or_init_state(&path).unwrap();
    assert!(path.exists());
    let loaded = BackupState::load(&path).unwrap();
    assert_eq!(state.latest.timestamp.timestamp(), loaded.latest.timestamp.timestamp());
    assert_eq!(state.stats.len(), loaded.stats.len());
}
