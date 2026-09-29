use super::backup::create_backup;
use super::gitignore::ensure_env_in_gitignore;
use crate::models::FixPlan;
use std::fs;
use std::path::{Path, PathBuf};

// Atomic Applier

pub fn apply_fix_plan_atomically(root_path: &Path, plan: &FixPlan) -> Result<PathBuf, String> {
    let mut files_to_backup: Vec<PathBuf> = plan
        .file_changes
        .iter()
        .map(|c| c.file_path.clone())
        .collect();

    let env_path = root_path.join(".env");
    let example_path = root_path.join(".env.example");
    let gitignore_path = root_path.join(".gitignore");

    if env_path.exists() {
        files_to_backup.push(env_path.clone());
    }
    if example_path.exists() {
        files_to_backup.push(example_path.clone());
    }
    if gitignore_path.exists() {
        files_to_backup.push(gitignore_path.clone());
    }

    let backup_res = create_backup(root_path, &files_to_backup)
        .map_err(|e| format!("Failed to create backup: {}", e))?;

    let backup_dir = backup_res.backup_dir;

    let mut written_files: Vec<PathBuf> = Vec::new();

    let apply_result = (|| -> Result<(), std::io::Error> {
        for change in &plan.file_changes {
            fs::write(&change.file_path, &change.new_content)?;
            written_files.push(change.file_path.clone());
        }

        if !plan.env_entries.is_empty() {
            let mut env_content = String::new();
            for (k, v) in &plan.env_entries {
                env_content.push_str(&format!("{}={}\n", k, v));
            }
            fs::write(&env_path, env_content)?;
            written_files.push(env_path.clone());
        }

        if !plan.env_example_entries.is_empty() {
            let mut example_content = String::new();
            for k in &plan.env_example_entries {
                example_content.push_str(&format!("{}=\n", k));
            }
            fs::write(&example_path, example_content)?;
            written_files.push(example_path.clone());
        }

        if plan.gitignore_updated {
            ensure_env_in_gitignore(root_path)?;
            written_files.push(gitignore_path.clone());
        }

        Ok(())
    })();

    if let Err(write_err) = apply_result {
        eprintln!(
            "Write failure occurred: {}. Rolling back changes...",
            write_err
        );
        for target in &files_to_backup {
            let relative = target.strip_prefix(root_path).unwrap_or(target);
            let backup_source = backup_dir.join(relative);
            if backup_source.exists() {
                let _ = fs::copy(&backup_source, target);
            }
        }
        return Err(format!(
            "Transaction aborted and rolled back due to error: {}",
            write_err
        ));
    }

    Ok(backup_dir)
}
