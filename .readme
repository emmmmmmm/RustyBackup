# rustybackup

`rustybackup` started as a small learning scaffold and now demonstrates a simple
incremental backup workflow implemented in Rust.

The command line interface can scan configured directories, copy changed files to
a destination, manage historical versions and record the most recent backup
state. Configuration is loaded from a TOML file.

## Features

- Command line interface built with [`clap`](https://crates.io/crates/clap)
- Configuration parsed with `serde` and `toml`
- `scan` shows files changed since the last backup while honoring exclude patterns
- `backup` copies changed files, keeps prior versions in a `History` folder and can resume if interrupted
- `vacuum` and `status` subcommands are placeholders for future features
- Persistent `state.toml` tracks snapshot IDs and transfer statistics
- Stub module for reading the NTFS USN change journal on Windows

## Building

1. Install Rust from [rustup.rs](https://rustup.rs).
2. Build the project with `cargo build`.
3. Run a command, e.g. `cargo run -- scan --config config.toml`.
4. Execute the test suite with `cargo test`.

## Repository Layout

- `src/main.rs` - entry point and CLI definitions
- `src/lib.rs` - library exports used by tests
- `src/config.rs` - configuration structures
- `src/state.rs` - persistent backup state tracking
- `src/backup.rs` - scanning and backup logic
- `src/journal.rs` - changed file detection helpers
- `tests/` - integration and unit tests

