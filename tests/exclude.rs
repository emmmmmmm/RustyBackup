use std::fs::{self, File};
use std::io::Write;
use std::process::Command;
use tempfile::tempdir;

#[test]
fn test_scan_respects_exclude() {
    let tmp = tempdir().expect("tempdir");
    let include_dir = tmp.path().join("inc");
    let exclude_dir = include_dir.join("skip");
    fs::create_dir_all(&exclude_dir).unwrap();

    // create files
    File::create(include_dir.join("file.txt")).unwrap();
    File::create(exclude_dir.join("secret.txt")).unwrap();

    // Write config
    let config_content = format!(
        "[paths]\ninclude=[\"{}\"]\nexclude=[\"{}\"]\n\n[backup]\ndestination=\"/tmp\"\nkeep_versions=false\n",
        include_dir.display(),
        exclude_dir.display()
    );
    let config_path = tmp.path().join("config.toml");
    let mut f = File::create(&config_path).unwrap();
    f.write_all(config_content.as_bytes()).unwrap();

    let output = Command::new(env!("CARGO_BIN_EXE_rustybackup"))
        .arg("--config")
        .arg(&config_path)
        .arg("scan")
        .output()
        .expect("run binary");

    assert!(output.status.success());
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("file.txt"));
    assert!(!stdout.contains("secret.txt"));
}

#[test]
fn test_nested_children_excluded() {
    let tmp = tempdir().expect("tempdir");
    let include_dir = tmp.path().join("inc");
    let exclude_dir = include_dir.join("skip");
    let nested_dir = exclude_dir.join("deep");
    fs::create_dir_all(&nested_dir).unwrap();

    // create files
    File::create(include_dir.join("file.txt")).unwrap();
    File::create(nested_dir.join("very_secret.txt")).unwrap();

    let config_content = format!(
        "[paths]\ninclude=[\"{}\"]\nexclude=[\"{}\"]\n\n[backup]\ndestination=\"/tmp\"\nkeep_versions=false\n",
        include_dir.display(),
        exclude_dir.display()
    );

    let config_path = tmp.path().join("config.toml");
    let mut f = File::create(&config_path).unwrap();
    f.write_all(config_content.as_bytes()).unwrap();

    let output = Command::new(env!("CARGO_BIN_EXE_rustybackup"))
        .arg("--config")
        .arg(&config_path)
        .arg("scan")
        .output()
        .expect("run binary");

    assert!(output.status.success());
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("file.txt"));
    assert!(!stdout.contains("very_secret.txt"));
}
