use chrono::Local;
use std::fs;
use std::path::{Path, PathBuf};

// Backup Manager

pub struct BackupResult {
    pub backup_dir: PathBuf,
}

pub fn create_backup(
    root_path: &Path,
    files_to_backup: &[PathBuf],
) -> Result<BackupResult, std::io::Error> {
    let timestamp = Local::now().format("%Y%m%d_%H%M%S").to_string();
    let backup_dir = root_path.join(".secretsift-backup").join(timestamp);

    fs::create_dir_all(&backup_dir)?;

    for file_path in files_to_backup {
        if file_path.exists() {
            let relative = file_path.strip_prefix(root_path).unwrap_or(file_path);
            let dest = backup_dir.join(relative);

            if let Some(parent) = dest.parent() {
                fs::create_dir_all(parent)?;
            }

            fs::copy(file_path, dest)?;
        }
    }

    Ok(BackupResult { backup_dir })
}
