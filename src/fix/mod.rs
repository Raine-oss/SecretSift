pub mod diff;
pub mod env;

use crate::adapters::get_adapter_for_language;
use crate::models::{FileChange, FixPlan, LineReplacement, SecretMatch};
use env::generate_env_files;
use std::collections::HashMap;
use std::fs;
use std::path::{Path, PathBuf};

// Fix Plan Builder

pub fn build_fix_plan(root_path: &Path, matches: &[SecretMatch]) -> Result<FixPlan, String> {
    if matches.is_empty() {
        return Ok(FixPlan {
            file_changes: Vec::new(),
            env_entries: Vec::new(),
            env_example_entries: Vec::new(),
            gitignore_updated: false,
        });
    }

    let mut file_matches: HashMap<PathBuf, Vec<SecretMatch>> = HashMap::new();
    for m in matches {
        file_matches
            .entry(m.file_path.clone())
            .or_default()
            .push(m.clone());
    }

    let mut file_changes = Vec::new();
    let mut all_extracted_entries = Vec::new();

    for (file_path, mut items) in file_matches {
        items.sort_by_key(|m| m.line_number);

        let original_content = match fs::read_to_string(&file_path) {
            Ok(c) => c,
            Err(e) => {
                return Err(format!(
                    "Could not read file {}: {}",
                    file_path.display(),
                    e
                ))
            }
        };

        let lang = items
            .first()
            .map(|m| m.language)
            .unwrap_or(crate::models::Language::Unknown);
        let adapter = match get_adapter_for_language(lang) {
            Some(a) => a,
            None => continue,
        };

        let lines: Vec<&str> = original_content.lines().collect();
        let mut new_lines: Vec<String> = lines.iter().map(|&s| s.to_string()).collect();
        let mut replacements = Vec::new();

        for item in &items {
            let idx = item.line_number.saturating_sub(1);
            if idx < new_lines.len() {
                let current_line = &new_lines[idx];
                if let Some(rewritten) = adapter.rewrite_line(item, current_line) {
                    replacements.push(LineReplacement {
                        line_number: item.line_number,
                        original_line: current_line.clone(),
                        new_line: rewritten.clone(),
                        env_var_name: item.suggested_var_name.clone(),
                        secret_value: item.matched_value.clone(),
                    });
                    new_lines[idx] = rewritten;
                    all_extracted_entries
                        .push((item.suggested_var_name.clone(), item.matched_value.clone()));
                }
            }
        }

        if !replacements.is_empty() {
            let mut final_content = new_lines.join("\n");
            if original_content.ends_with('\n') {
                final_content.push('\n');
            }
            final_content = adapter.post_process_file_content(&final_content);

            file_changes.push(FileChange {
                file_path,
                original_content,
                new_content: final_content,
                replacements,
            });
        }
    }

    let env_result = generate_env_files(root_path, &all_extracted_entries);

    let mut env_entries = Vec::new();
    for line in env_result.env_content.lines() {
        if let Some((k, v)) = line.split_once('=') {
            env_entries.push((k.to_string(), v.to_string()));
        }
    }

    let mut env_example_entries = Vec::new();
    for line in env_result.example_content.lines() {
        if let Some((k, _)) = line.split_once('=') {
            env_example_entries.push(k.to_string());
        }
    }

    let gitignore_path = root_path.join(".gitignore");
    let gitignore_content = fs::read_to_string(&gitignore_path).unwrap_or_default();
    let gitignore_has_env = gitignore_content
        .lines()
        .any(|l| l.trim() == ".env" || l.trim() == "/.env" || l.trim() == "*.env");

    let gitignore_updated = !gitignore_has_env;

    let plan = FixPlan {
        file_changes,
        env_entries,
        env_example_entries,
        gitignore_updated,
    };

    validate_fix_plan(&plan)?;

    Ok(plan)
}

// Fix Plan Validator

pub fn validate_fix_plan(plan: &FixPlan) -> Result<(), String> {
    for change in &plan.file_changes {
        if change.new_content.trim().is_empty() && !change.original_content.trim().is_empty() {
            return Err(format!(
                "Validation error: transformation resulted in empty content for {}",
                change.file_path.display()
            ));
        }
    }
    Ok(())
}
