use rustybackup::config::Config;
use toml; // might require network access to fetch dependencies

#[test]
fn test_parse_config() {
    let sample = r#"
        [paths]
        include = ["C:/Test"]
        exclude = ["*/Temp"]

        [backup]
        destination = "Z:/Backups"
        max_versions = 5
    "#;

    let config: Config = toml::from_str(sample).expect("Failed to parse config");
    assert_eq!(config.backup.destination, "Z:/Backups");
    assert_eq!(config.paths.include.len(), 1);
}
