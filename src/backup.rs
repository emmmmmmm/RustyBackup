use crate::config::Config;
use crate::journal;
use crate::state::load_or_init_state;
use serde::{Deserialize, Serialize};
use anyhow::{bail, Context, Result};
use std::fs;
use std::path::{Path, PathBuf};
use std::time::Instant;
use std::time::SystemTime;
use std::time::Duration;
use std::collections::{HashMap, HashSet};
use chrono::{DateTime, Local};
use chrono::{NaiveDateTime};
use chrono::TimeZone;
use indicatif::{ProgressBar, ProgressStyle};
use regex::Regex;


#[derive(Serialize, Deserialize)]
pub struct Status {
    pub state: String,
}

#[derive(Serialize, Deserialize)]
pub struct FileList {
   pub files: Vec<PathBuf>,
}
impl FromIterator<PathBuf> for FileList {
    fn from_iter<I: IntoIterator<Item = PathBuf>>(iter: I) -> Self {
        FileList { files: iter.into_iter().collect() }
    }
}
#[derive(Serialize, Deserialize)]
pub struct TempBackup {
    pub status: Status,
    pub incomplete: FileList,
    pub completed: FileList,
    pub failed: FileList,
    #[serde(default)]
    pub bytes_copied: u64,
    pub duration:  Duration,
    pub timestamp: DateTime<Local>,
    pub snapshot_id: u64,
}

impl TempBackup {
    fn new(files: Vec<PathBuf>) -> Self {
        Self {
            status: Status {
                state: "in_progress".to_string(),
            },
            incomplete: FileList { files },
            completed: FileList { files: Vec::new() },
            failed: FileList { files: Vec::new() },
            bytes_copied: 0,
            duration: Duration::ZERO,
            timestamp: DateTime::<Local>::from(SystemTime::UNIX_EPOCH),
            snapshot_id: 0,
        }
    }

    fn load(path: &Path) -> anyhow::Result<Self> {
        let data = fs::read_to_string(path)?;
        Ok(toml::from_str(&data)?)
    }

    fn save(&self, path: &Path) -> anyhow::Result<()> {
        let data = toml::to_string_pretty(self)?;
        fs::write(path, data)?;
        Ok(())
    }
}

/// Scan and print all files under the configured include paths.
///
/// Entries matching any of the configured exclude patterns will be skipped.
pub fn scan(config: &Config) -> anyhow::Result<()> {
    let includes: Vec<PathBuf> = config
        .paths
        .include
        .iter()
        .map(PathBuf::from)
        .collect();

    let state_path = PathBuf::from(&config.backup.destination).join("state.toml");
    let state = load_or_init_state(&state_path)?;
    let since: SystemTime = state.latest.timestamp.into();

    let dest_root = PathBuf::from(&config.backup.destination);
    let files = journal::changed_files(since, &includes, &config.paths.exclude, &dest_root)?;
    let total_bytes: u64 = files
        .iter()
        .filter_map(|p| fs::metadata(p).ok().map(|m| m.len()))
        .sum();
    let total_mb = total_bytes as f64 / (1024.0 * 1024.0);

    println!(
        "Found {} changed files since {}",
        files.len(),
        state.latest.timestamp
    );

    for path in &files {
        println!("{}", path.display());
    }

    println!(
        "Scan complete. {} files, {:.2} MB total.",
        files.len(),
        total_mb
    );

    Ok(())
}


/// Run the backup operation.
pub fn run_backup(config: &Config) -> Result<()> {
    let start_time = Instant::now();
    let dest = PathBuf::from(&config.backup.destination);

    // Ensure the destination directory exists or create it
    if !dest.exists() {
        fs::create_dir_all(&dest)
            .with_context(|| format!("Failed to create or access backup destination. Is the network drive mounted? Path: {}", dest.display()))?;

    }

    // Now canonicalize the existing directory
    let dest = dest.canonicalize().context("Invalid or inaccessible backup destination path")?;
    if !dest.is_dir() {
        bail!("Backup destination must be a directory: {}", dest.display());
    }

    let state_file = dest.join("state.toml");
    let mut state = load_or_init_state(&state_file)?;
    let since: SystemTime = state.latest.timestamp.into();

    // Create path to progress file
    let temp_state_file = dest.join(".incomplete");

    let mut progress = if temp_state_file.exists() {
        let tmp = TempBackup::load(&temp_state_file)?;
        if tmp.status.state == "in_progress" {
            tmp
        } else {
            let includes: Vec<PathBuf> = config
                .paths
                .include
                .iter()
                .map(PathBuf::from)
                .collect();
            let dest_root = PathBuf::from(&config.backup.destination);
            let changed = journal::changed_files(since, &includes, &config.paths.exclude, &dest_root)?;
            TempBackup::new(changed)
        }
    } else {    
        let includes: Vec<PathBuf> = config
            .paths
            .include
            .iter()
            .map(PathBuf::from)
            .collect();
        let dest_root = PathBuf::from(&config.backup.destination);
        let  changed = journal::changed_files(since, &includes, &config.paths.exclude, &dest_root)?;
        TempBackup::new(changed)
    };

    // Set or increment snapshot id based on the previously stored state
    if progress.snapshot_id == 0 {
        let last_id = state
            .latest
            .snapshot_id
            .parse::<u64>()
            .unwrap_or(0);
        progress.snapshot_id = last_id.saturating_add(1);
    }









    // Backup each file individually
    let mut completed: HashSet<PathBuf> = progress.completed.files.iter().cloned().collect();
    let mut failed: HashSet<PathBuf> = progress.failed.files.iter().cloned().collect();


    let total_files = progress.incomplete.files.len() as u64;
    let pb = ProgressBar::new(total_files);
    pb.set_style(
        ProgressStyle::default_bar()
            .template("{spinner:.green} [{elapsed_precise}] [{bar:40.cyan/blue}] {pos}/{len} files ({eta})")
            .unwrap()
            .progress_chars("##-")
    );



    for path in &progress.incomplete.files {
        pb.inc(1);
        pb.set_message(path.display().to_string());
        if completed.contains(path) {
            continue;
        }

        // Validate source file
        if !path.exists() || !path.is_file() {
            eprintln!("Skipping invalid path: {}", path.display());
            failed.insert(path.clone());
            continue;
        }

        // Create relative path and normalized root from the first matching include path
        let (normalized_root, relative) = config
            .paths
            .include
            .iter()
            .find_map(|root| {
                let root_path = Path::new(root);
                path.strip_prefix(root_path).ok().map(|rel| {
                    let root_label = root_path
                        .to_string_lossy()
                        .replace(':', "")
                        .replace('\\', "-")
                        .replace('/', "-");
                    (root_label, rel)
                })
            })
            .unwrap_or_else(|| ("UnknownSource".into(), path));

       let final_file = dest.join(&normalized_root).join(relative);
        let temp_file = final_file.with_extension("part");

        // Ensure parent directory exists
        if let Some(parent) = final_file.parent() {
            fs::create_dir_all(parent).with_context(|| format!("Failed to create directory: {}", parent.display()))?;
        }

        
        // if the target file already exists, move it to history folder, and postfix it with a timestamp before the extension
        if final_file.exists() {
            let history_dir = dest
                .join("History")
                .join(&normalized_root)
                .join(relative.parent().unwrap_or_else(|| Path::new("")));
            fs::create_dir_all(&history_dir)
                .with_context(|| format!("Failed to create history directory: {}", history_dir.display()))?;

            let timestamp = Local::now().format("%Y-%m-%dT%H-%M-%S");

            let file_stem = final_file
                .file_stem()
                .and_then(|s| s.to_str())
                .unwrap_or("file");

            let extension = final_file.extension().and_then(|e| e.to_str());

            let filename = match extension {
                Some(ext) => format!("{}_{}.{}", file_stem, timestamp, ext),
                None => format!("{}_{}", file_stem, timestamp),
            };

            let history_path = history_dir.join(filename);
            fs::rename(&final_file, &history_path)
                .with_context(|| format!("Failed to move existing file to history: {}", history_path.display()))?;
        }

        // Perform the copy
        match fs::copy(&path, &temp_file) {
            Ok(size) => {
                progress.bytes_copied = progress.bytes_copied.saturating_add(size);
                fs::rename(&temp_file, &final_file)
                    .with_context(|| format!("Failed to rename: {}", final_file.display()))?;
                completed.insert(path.clone());
            }
            Err(e) => {
                eprintln!("Failed to copy {}: {e}", path.display());
                failed.insert(path.clone());
            }
        }

        // Update progress after each file
        progress.completed = completed.iter().cloned().collect();
        progress.failed = failed.iter().cloned().collect();
        progress.save(&temp_state_file)?;
    }
    // Finalize backup status
    progress.status = Status { state: "in_progress".to_string(),            };
    progress.duration = start_time.elapsed(); // todo add field to progress
    progress.timestamp = Local::now();
    progress.save(&temp_state_file)?;

    // Update global state
    state.record_backup(&progress, &config); 
    state.save(&state_file)?;

    // Remove .incomplete marker
    fs::remove_file(&temp_state_file).ok();
    
    pb.finish_with_message("Backup completed.");
    //println!("Backup completed.");
    Ok(())
}


/// Vacuum old versions from the history folder, keeping only the most recent `max_versions`.


pub fn vacuum(config: &Config) -> Result<()> {
    println!("Vacuuming old backups...");

    let history_root = PathBuf::from(&config.backup.destination).join("History");
    if !history_root.exists() {
        println!("No history folder found.");
        return Ok(());
    }

    // Collect all file entries so we know the scan length for the progress bar
    let entries: Vec<_> = walkdir::WalkDir::new(&history_root)
        .into_iter()
        .filter_map(Result::ok)
        .filter(|e| e.file_type().is_file())
        .collect();

    let scan_pb = ProgressBar::new(entries.len() as u64);
    scan_pb.set_style(
        ProgressStyle::default_bar()
            .template("{spinner:.green} [{elapsed_precise}] [{bar:40.cyan/blue}] {pos}/{len} files ({eta})")
            .unwrap()
            .progress_chars("##-")
    );

    let mut file_versions: HashMap<PathBuf, Vec<(DateTime<Local>, PathBuf)>> = HashMap::new();
    let re = Regex::new(r"^(.*)_((?:\d{4}-\d{2}-\d{2}T\d{2}-\d{2}-\d{2}))(\..+)?$").unwrap();

    for entry in entries {
        scan_pb.inc(1);
        scan_pb.set_message(entry.path().display().to_string());
        let full_path = normalize_path(entry.path()).to_path_buf();
        let filename = full_path.file_name().unwrap().to_string_lossy();
        if let Some(caps) = re.captures(&filename) {
            let base = caps.get(1).unwrap().as_str();
            let ts_str = caps.get(2).unwrap().as_str();
            let ext = caps.get(3).map(|e| e.as_str()).unwrap_or("");

            if let Ok(naive) = NaiveDateTime::parse_from_str(ts_str, "%Y-%m-%dT%H-%M-%S") {
                let mut canonical = full_path.clone();
                let local_dt = Local.from_local_datetime(&naive).unwrap();
                canonical.set_file_name(format!("{}{}", base, ext));
                let canonical = normalize_path(&canonical);

                file_versions
                    .entry(canonical)
                    .or_default()
                    .push((local_dt, full_path.clone()));
            }
        }
    }
    scan_pb.finish_with_message("Scan complete.");

    let mut total_candidates = 0;
    let mut delete_candidates: Vec<PathBuf> = Vec::new();
    for (base_path, mut versions) in file_versions {
        versions.sort_by_key(|(ts, _)| std::cmp::Reverse(*ts));
        let keep = config.backup.max_versions.unwrap_or(0) as usize;
        let to_prune = &versions[keep.min(versions.len())..];
        if !to_prune.is_empty() {
            println!("Found {} prune candidates for {}:", to_prune.len(), base_path.display());
            for (_ts, path) in to_prune {
                println!("{} ", path.display());
                delete_candidates.push(path.clone());
                total_candidates += 1;
            }
        }
    }

    if delete_candidates.is_empty() {
        println!("Nothing to vacuum.");
        return Ok(());
    }

    let delete_pb = ProgressBar::new(delete_candidates.len() as u64);
    delete_pb.set_style(
        ProgressStyle::default_bar()
            .template("{spinner:.green} [{elapsed_precise}] [{bar:40.cyan/blue}] {pos}/{len} files ({eta})")
            .unwrap()
            .progress_chars("##-")
    );

    for path in delete_candidates {
        delete_pb.inc(1);
        delete_pb.set_message(path.display().to_string());
        fs::remove_file(&path).with_context(|| format!("Failed to delete {}", path.display()))?;
    }
    delete_pb.finish_with_message("Vacuum complete.");

    println!("Removed {} outdated file(s).", total_candidates);

    Ok(())
}


/// Placeholder status implementation.
pub fn status(_config: &Config) -> anyhow::Result<()> {
    println!("Backup status: OK");
    Ok(())
}


fn normalize_path(path: &Path) -> PathBuf {
    PathBuf::from(path.to_string_lossy().replace('\\', "/"))
}