use std::process::Command;

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
    let output = binary()
        .args(["scan", "--config", "tests/test_config.toml"])
        .output()
        .expect("failed to run");
    assert!(output.status.success());
}

#[test]
fn backup_subcommand_runs() {
    let output = binary()
        .args(["backup", "--config", "tests/test_config.toml"])
        .output()
        .expect("failed to run");
    assert!(output.status.success());
}

#[test]
fn vacuum_subcommand_runs() {
    let output = binary()
        .args(["vacuum", "--config", "tests/test_config.toml"])
        .output()
        .expect("failed to run");
    assert!(output.status.success());
}

#[test]
fn status_subcommand_runs() {
    let output = binary()
        .args(["status", "--config", "tests/test_config.toml"])
        .output()
        .expect("failed to run");
    assert!(output.status.success());
}
