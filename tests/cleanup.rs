use std::fs::{self, File};
use tempfile::tempdir;
use rustybackup::{backup, config::{Config, BackupPaths, BackupOptions}};

#[test]
fn removed_files_are_moved_to_history() {
    let tmp = tempdir().unwrap();
    let src = tmp.path().join("src");
    fs::create_dir_all(&src).unwrap();
    let dest = tmp.path().join("dest");
    let file_path = src.join("file.txt");
    File::create(&file_path).unwrap();

    let config = Config {
        paths: BackupPaths {
            include: vec![src.to_string_lossy().to_string()],
            exclude: vec![],
        },
        backup: BackupOptions {
            destination: dest.to_string_lossy().to_string(),
            max_versions: Some(1),
        },
    };

    backup::run_backup(&config).unwrap();

    fs::remove_file(&file_path).unwrap();

    backup::run_backup(&config).unwrap();

    let label = src.to_string_lossy().replace(':', "").replace('\\', "-").replace('/', "-");
    let primary = dest.join(&label).join("file.txt");
    assert!(!primary.exists());

    let hist_dir = dest.join("History").join(&label);
    let entries: Vec<_> = fs::read_dir(&hist_dir).unwrap().filter_map(|e| e.ok()).collect();
    assert_eq!(entries.len(), 1);
    let name = entries[0].file_name();
    let name = name.to_string_lossy();
    assert!(name.starts_with("file_"));
}
