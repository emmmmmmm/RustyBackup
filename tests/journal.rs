use std::fs::{self, File};
use std::time::{SystemTime, Duration};
use tempfile::tempdir;
use rustybackup::journal;

#[test]
fn file_included_when_missing_in_destination() {
    let tmp = tempdir().unwrap();
    let include_dir = tmp.path().join("inc");
    fs::create_dir_all(&include_dir).unwrap();
    let src_file = include_dir.join("a.txt");
    File::create(&src_file).unwrap();

    // wait to ensure modification time < since
    std::thread::sleep(Duration::from_millis(10));
    let since = SystemTime::now();

    let dest_dir = tmp.path().join("dest");
    fs::create_dir_all(&dest_dir).unwrap();

    let changed = journal::changed_files(since, &[include_dir.clone()], &[], &dest_dir, true).unwrap();
    assert_eq!(changed, vec![src_file]);
}

#[test]
fn file_skipped_when_present_and_unchanged() {
    let tmp = tempdir().unwrap();
    let include_dir = tmp.path().join("inc");
    fs::create_dir_all(&include_dir).unwrap();
    let src_file = include_dir.join("b.txt");
    File::create(&src_file).unwrap();

    let dest_dir = tmp.path().join("dest");
    // replicate backup layout
    let root_label = include_dir.to_string_lossy().replace(':', "").replace('\\', "-").replace('/', "-");
    let dest_file = dest_dir.join(root_label).join("b.txt");
    fs::create_dir_all(dest_file.parent().unwrap()).unwrap();
    File::create(&dest_file).unwrap();

    std::thread::sleep(Duration::from_millis(10));
    let since = SystemTime::now();

    let changed = journal::changed_files(since, &[include_dir.clone()], &[], &dest_dir, true).unwrap();
    assert!(changed.is_empty());
}
