use secretsift::fix::build_fix_plan;
use secretsift::models::Confidence;
use secretsift::safety::apply_fix_plan_atomically;
use secretsift::scanner::ProjectScanner;
use std::fs;
use std::path::PathBuf;

// Idempotency Integration Test

#[test]
fn test_fix_scan_fix_idempotency() {
    let temp_root = PathBuf::from("target/test_idempotency_repo");
    let _ = fs::remove_dir_all(&temp_root);
    fs::create_dir_all(temp_root.join("src")).unwrap();

    let rust_file = temp_root.join("src/config.rs");
    let py_file = temp_root.join("src/app.py");
    let js_file = temp_root.join("src/client.js");
    let java_file = temp_root.join("src/Main.java");

    fs::write(
        &rust_file,
        "let api_key = \"sk-test_1234567890abcdef1234567890abcdef\";\n",
    )
    .unwrap();

    fs::write(
        &py_file,
        "import json\n\ndb_url = \"postgres://user:pass12345678@localhost:5432/app\"\n",
    )
    .unwrap();

    fs::write(
        &js_file,
        "const stripe_key = \"sk_sift_51ABCDEF1234567890abcdef\";\n",
    )
    .unwrap();

    fs::write(
        &java_file,
        "public class Main {\n    private static final String GITHUB_TOKEN = \"ghp_111111111122222222223333333333444444\";\n}\n",
    )
    .unwrap();

    let scanner = ProjectScanner::new(Confidence::High);

    // Initial Scan
    let matches_initial = scanner.scan_path(&temp_root).unwrap();
    assert_eq!(matches_initial.len(), 4);

    // First Fix
    let plan1 = build_fix_plan(&temp_root, &matches_initial).unwrap();
    assert_eq!(plan1.file_changes.len(), 4);
    assert_eq!(plan1.env_entries.len(), 4);

    let backup_dir = apply_fix_plan_atomically(&temp_root, &plan1).unwrap();
    assert!(backup_dir.exists());

    // Second Scan Immediately After Fix
    let matches_after_fix = scanner.scan_path(&temp_root).unwrap();
    assert_eq!(
        matches_after_fix.len(),
        0,
        "Scan after fix must find 0 secrets!"
    );

    // Second Fix Attempt
    let plan2 = build_fix_plan(&temp_root, &matches_after_fix).unwrap();
    assert_eq!(
        plan2.file_changes.len(),
        0,
        "Second fix must produce 0 file changes!"
    );

    let env_content = fs::read_to_string(temp_root.join(".env")).unwrap();
    let env_lines: Vec<&str> = env_content.lines().collect();
    assert_eq!(env_lines.len(), 4);
    assert!(env_content.contains("API_KEY="));
    assert!(env_content.contains("DB_URL="));
    assert!(env_content.contains("STRIPE_KEY="));
    assert!(env_content.contains("GITHUB_TOKEN="));
    assert!(!env_content.contains("API_KEY_2"));

    let _ = fs::remove_dir_all(&temp_root);
}
