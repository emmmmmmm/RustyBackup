use std::process::Command;
use std::fs;
use std::path::PathBuf;
use tempfile::tempdir;
use std::time::SystemTime;
use rustybackup::journal;

fn binary() -> Command {
    Command::new(env!("CARGO_BIN_EXE_rustybackup"))
}

#[test]
fn help_displays() {
    let output = binary().arg("--help").output().expect("failed to run");
    assert!(output.status.success());
}

#[test]
fn scan_subcommand_runs() {
    let tmp = tempdir().unwrap();
    let dest = tmp.path().join("dest");
    let config_path = tmp.path().join("config.toml");
    let content = format!(
        "[paths]\ninclude=[\"src\"]\nexclude=[]\n\n[backup]\ndestination=\"{}\"\nkeep_versions=true\nmax_versions=1\n",
        dest.display()
    );
    fs::write(&config_path, content).unwrap();

    let includes = vec![PathBuf::from("src")];
    let expected = journal::changed_files(SystemTime::UNIX_EPOCH, &includes, &[], &dest)
        .expect("collect changed files");
    let mut expected: Vec<String> = expected.iter().map(|p| p.display().to_string()).collect();
    expected.sort();

    let output = binary()
        .args(["--config", config_path.to_str().unwrap(), "scan"])
        .output()
        .expect("run binary");
    assert!(output.status.success());
    let stdout = String::from_utf8_lossy(&output.stdout);
    let mut iter = stdout.lines();
    iter.next(); // skip config dump
    let summary = iter.next().unwrap_or("");
    assert!(summary.starts_with("Found "));
    let mut lines: Vec<String> = iter.map(|s| s.to_string()).collect();
    lines.sort();
    assert_eq!(lines, expected);
}

#[test]
fn backup_subcommand_runs() {
    let tmp = tempdir().unwrap();
    let dest = tmp.path().join("dest");
    let config_path = tmp.path().join("config.toml");
    let content = format!(
        "[paths]\ninclude=[\"src\"]\nexclude=[]\n\n[backup]\ndestination=\"{}\"\nkeep_versions=true\nmax_versions=1\n",
        dest.display()
    );
    fs::write(&config_path, content).unwrap();

    let output = binary()
        .args(["--config", config_path.to_str().unwrap(), "backup"])
        .output()
        .expect("run binary");
    assert!(output.status.success());
}

#[test]
fn vacuum_subcommand_runs() {
    let tmp = tempdir().unwrap();
    let dest = tmp.path().join("dest");
    let config_path = tmp.path().join("config.toml");
    let content = format!(
        "[paths]\ninclude=[\"src\"]\nexclude=[]\n\n[backup]\ndestination=\"{}\"\nkeep_versions=true\nmax_versions=1\n",
        dest.display()
    );
    fs::write(&config_path, content).unwrap();

    let output = binary()
        .args(["--config", config_path.to_str().unwrap(), "vacuum"])
        .output()
        .expect("run binary");
    assert!(output.status.success());
}

#[test]
fn status_subcommand_runs() {
    let tmp = tempdir().unwrap();
    let dest = tmp.path().join("dest");
    let config_path = tmp.path().join("config.toml");
    let content = format!(
        "[paths]\ninclude=[\"src\"]\nexclude=[]\n\n[backup]\ndestination=\"{}\"\nkeep_versions=true\nmax_versions=1\n",
        dest.display()
    );
    fs::write(&config_path, content).unwrap();

    let output = binary()
        .args(["--config", config_path.to_str().unwrap(), "status"])
        .output()
        .expect("run binary");
    assert!(output.status.success());
}
