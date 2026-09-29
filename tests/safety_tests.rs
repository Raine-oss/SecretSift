use secretsift::models::{FileChange, FixPlan, LineReplacement};
use secretsift::safety::{apply_fix_plan_atomically, ensure_env_in_gitignore};
use std::fs;
use std::path::PathBuf;

// Gitignore Safety Tests

#[test]
fn test_ensure_env_in_gitignore() {
    let temp_dir = PathBuf::from("target/test_temp_gitignore");
    let _ = fs::remove_dir_all(&temp_dir);
    fs::create_dir_all(&temp_dir).unwrap();

    let updated = ensure_env_in_gitignore(&temp_dir).unwrap();
    assert!(updated);

    let content = fs::read_to_string(temp_dir.join(".gitignore")).unwrap();
    assert!(content.contains(".env"));

    let updated_second = ensure_env_in_gitignore(&temp_dir).unwrap();
    assert!(!updated_second);

    let _ = fs::remove_dir_all(&temp_dir);
}

// Atomic Fix and Backup Tests

#[test]
fn test_atomic_apply_and_backup() {
    let temp_dir = PathBuf::from("target/test_temp_atomic");
    let _ = fs::remove_dir_all(&temp_dir);
    fs::create_dir_all(&temp_dir).unwrap();

    let file_path = temp_dir.join("sample.rs");
    let original = "let key = \"secret12345678\";\n";
    fs::write(&file_path, original).unwrap();

    let plan = FixPlan {
        file_changes: vec![FileChange {
            file_path: file_path.clone(),
            original_content: original.to_string(),
            new_content: "let key = std::env::var(\"API_KEY\").unwrap_or_default();\n".to_string(),
            replacements: vec![LineReplacement {
                line_number: 1,
                original_line: "let key = \"secret12345678\";".to_string(),
                new_line: "let key = std::env::var(\"API_KEY\").unwrap_or_default();".to_string(),
                env_var_name: "API_KEY".to_string(),
                secret_value: "secret12345678".to_string(),
            }],
        }],
        env_entries: vec![("API_KEY".to_string(), "secret12345678".to_string())],
        env_example_entries: vec!["API_KEY".to_string()],
        gitignore_updated: true,
    };

    let backup_dir = apply_fix_plan_atomically(&temp_dir, &plan).unwrap();
    assert!(backup_dir.exists());

    let modified = fs::read_to_string(&file_path).unwrap();
    assert!(modified.contains("std::env::var(\"API_KEY\")"));

    let env_content = fs::read_to_string(temp_dir.join(".env")).unwrap();
    assert!(env_content.contains("API_KEY=secret12345678"));

    let _ = fs::remove_dir_all(&temp_dir);
}
