use std::process::Command;
use std::fs;
use std::path::PathBuf;
use std::time::SystemTime;
use rustybackup::config::Config;
use rustybackup::journal;
use toml;

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
    // Load config using the library helper
    let data = fs::read_to_string("tests/test_config.toml").expect("read config");
    let config: Config = toml::from_str(&data).expect("parse config");

    // Determine expected files using the journal module
    let includes: Vec<PathBuf> = config.paths.include.iter().map(PathBuf::from).collect();
    let expected = journal::changed_files(SystemTime::UNIX_EPOCH, &includes)
        .expect("collect changed files");
    let mut expected: Vec<String> = expected
        .iter()
        .map(|p| p.display().to_string())
        .collect();
    expected.sort();

    let output = binary()
        .args(["--config", "tests/test_config.toml", "scan"])
        .output()
        .expect("failed to run");
    assert!(output.status.success());
    let stdout = String::from_utf8_lossy(&output.stdout);
    let mut lines: Vec<String> = stdout
        .lines()
        .skip(1) // skip config dump
        .map(|s| s.to_string())
        .collect();
    lines.sort();
    assert_eq!(lines, expected);
}

#[test]
fn backup_subcommand_runs() {
    let output = binary()
        .args(["--config", "tests/test_config.toml", "backup"])
        .output()
        .expect("failed to run");
    assert!(output.status.success());
}

#[test]
fn vacuum_subcommand_runs() {
    let output = binary()
        .args(["--config", "tests/test_config.toml", "vacuum"])
        .output()
        .expect("failed to run");
    assert!(output.status.success());
}

#[test]
fn status_subcommand_runs() {
    let output = binary()
        .args(["--config", "tests/test_config.toml", "status"])
        .output()
        .expect("failed to run");
    assert!(output.status.success());
}
