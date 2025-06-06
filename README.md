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

## Configuration

Settings are loaded from a `config.toml` file. The file is divided into
`[paths]` and `[backup]` sections:

```toml
[paths]
include = ["src"]          # directories to scan
exclude = ["*/temp"]      # optional glob patterns to ignore

[backup]
destination = "backups"   # where backup data and state are stored
keep_versions = true       # keep previous versions under a `History` folder
max_versions = 5           # limit history depth when set
```

Field descriptions:

- **paths.include**: directories that will be scanned for changed files.
- **paths.exclude**: list of patterns to skip during scanning.
- **backup.destination**: directory that receives the synchronized files and
  `state.toml`.
- **backup.keep_versions**: if `true`, prior versions of modified files are
  preserved in a `History` folder.
- **backup.max_versions**: optional maximum number of versions to keep when
  `keep_versions` is enabled.

See `tests/test_config.toml` for a minimal working example.

