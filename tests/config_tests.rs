#[derive(Debug)]
struct Config {
    paths: BackupPaths,
    backup: BackupOptions,
}

#[derive(Debug)]
struct BackupPaths {
    include: Vec<String>,
    exclude: Vec<String>,
}

#[derive(Debug)]
struct BackupOptions {
    destination: String,
    keep_versions: bool,
    max_versions: Option<u32>,
}

#[test]
fn test_struct_debug_output() {
    let config = Config {
        paths: BackupPaths {
            include: vec!["C:/Test".into()],
            exclude: vec!["*/Temp".into()],
        },
        backup: BackupOptions {
            destination: "Z:/Backups".into(),
            keep_versions: true,
            max_versions: Some(5),
        },
    };

    // Simple debug print to verify structure is sane
    println!("{:#?}", config);
    assert_eq!(config.backup.destination, "Z:/Backups");
}

