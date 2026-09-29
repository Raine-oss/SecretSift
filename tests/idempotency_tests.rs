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
    let go_file = temp_root.join("src/service.go");
    let cs_file = temp_root.join("src/Program.cs");
    let kt_file = temp_root.join("src/App.kt");
    let php_file = temp_root.join("src/index.php");
    let rb_file = temp_root.join("src/app.rb");
    let cpp_file = temp_root.join("src/main.cpp");
    let dart_file = temp_root.join("src/main.dart");
    let swift_file = temp_root.join("src/main.swift");

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

    fs::write(
        &go_file,
        "package main\n\nconst GoSecret = \"sk-sift-go-token-1234567890abcdef1234567890\"\n",
    )
    .unwrap();

    fs::write(
        &cs_file,
        "namespace App {\n    public static class C {\n        public static string CsKey = \"sk_sift_cs_12345678901234567890\";\n    }\n}\n",
    )
    .unwrap();

    fs::write(
        &kt_file,
        "package app\n\nval ktKey = \"sk_sift_kt_12345678901234567890\"\n",
    )
    .unwrap();

    fs::write(
        &php_file,
        "<?php\n$phpKey = 'sk_sift_php_12345678901234567890';\n",
    )
    .unwrap();

    fs::write(&rb_file, "RB_KEY = \"sk_sift_rb_12345678901234567890\"\n").unwrap();

    fs::write(
        &cpp_file,
        "const char* cppKey = \"sk_sift_cpp_12345678901234567890\";\n",
    )
    .unwrap();

    fs::write(
        &dart_file,
        "final dartKey = 'sk_sift_dart_12345678901234567890';\n",
    )
    .unwrap();

    fs::write(
        &swift_file,
        "let swiftKey = \"sk_sift_swift_12345678901234567890\"\n",
    )
    .unwrap();

    let scanner = ProjectScanner::new(Confidence::High);

    // Initial Scan
    let matches_initial = scanner.scan_path(&temp_root).unwrap();
    assert_eq!(matches_initial.len(), 12);

    // First Fix
    let plan1 = build_fix_plan(&temp_root, &matches_initial).unwrap();
    assert_eq!(plan1.file_changes.len(), 12);
    assert_eq!(plan1.env_entries.len(), 12);

    let backup_dir = apply_fix_plan_atomically(&temp_root, &plan1).unwrap();
    assert!(backup_dir.exists());

    // Second Scan Immediately After Fix
    let matches_after_fix = scanner.scan_path(&temp_root).unwrap();
    assert_eq!(
        matches_after_fix.len(),
        0,
        "Scan after fix must find 0 secrets across all 12 files!"
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
    assert_eq!(env_lines.len(), 12);
    assert!(!env_content.contains("_2="));

    let _ = fs::remove_dir_all(&temp_root);
}
