use rustybackup::state::{State, LatestBackup, BackupStats};
use chrono::Local;
use tempfile::NamedTempFile;
use std::path::PathBuf;

#[test]
fn state_roundtrip() {
    let tmp = NamedTempFile::new().unwrap();
    let state = State {
        latest: LatestBackup {
            timestamp: Local::now(),
            snapshot_id: "abc123".into(),
            destination: PathBuf::from("/tmp/backup"),
        },
        stats: BackupStats {
            files_synced: 1,
            bytes_copied: 2,
            duration_ms: 3,
        },
    };

    state.save(tmp.path()).unwrap();
    let loaded = State::load(tmp.path()).unwrap();

    assert_eq!(state.latest.snapshot_id, loaded.latest.snapshot_id);
    assert_eq!(state.latest.destination, loaded.latest.destination);
    assert_eq!(state.stats.files_synced, loaded.stats.files_synced);
    assert_eq!(state.stats.bytes_copied, loaded.stats.bytes_copied);
    assert_eq!(state.stats.duration_ms, loaded.stats.duration_ms);
    assert_eq!(state.latest.timestamp.timestamp(), loaded.latest.timestamp.timestamp());
}
