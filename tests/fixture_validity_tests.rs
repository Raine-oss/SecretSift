use secretsift::fix::build_fix_plan;
use secretsift::models::Confidence;
use secretsift::safety::apply_fix_plan_atomically;
use secretsift::scanner::ProjectScanner;
use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;

// Recursive Directory Copy Helper

fn copy_dir_all(src: &Path, dst: &Path) -> std::io::Result<()> {
    fs::create_dir_all(dst)?;
    for entry in fs::read_dir(src)? {
        let entry = entry?;
        let file_type = entry.file_type()?;
        let dest_path = dst.join(entry.file_name());
        if file_type.is_dir() {
            copy_dir_all(&entry.path(), &dest_path)?;
        } else {
            fs::copy(entry.path(), dest_path)?;
        }
    }
    Ok(())
}

// Rust Project Fixture Test

#[test]
fn test_rust_fixture_validity_and_compilation() {
    let fixture_src = PathBuf::from("tests/fixtures/rust_project");
    let sandbox = PathBuf::from("target/test_sandbox/rust_project");

    let _ = fs::remove_dir_all(&sandbox);
    copy_dir_all(&fixture_src, &sandbox).unwrap();

    let scanner = ProjectScanner::new(Confidence::High);

    let initial_matches = scanner.scan_path(&sandbox).unwrap();
    assert!(
        !initial_matches.is_empty(),
        "Should detect secrets in initial Rust fixture"
    );

    let plan = build_fix_plan(&sandbox, &initial_matches).unwrap();
    apply_fix_plan_atomically(&sandbox, &plan).unwrap();

    let after_matches = scanner.scan_path(&sandbox).unwrap();
    assert_eq!(
        after_matches.len(),
        0,
        "No secrets should remain after fix in Rust fixture"
    );

    let output = Command::new("cargo")
        .arg("check")
        .arg("--manifest-path")
        .arg(sandbox.join("Cargo.toml"))
        .output()
        .expect("Failed to execute cargo check");

    assert!(
        output.status.success(),
        "cargo check failed on refactored Rust code! stderr: {}",
        String::from_utf8_lossy(&output.stderr)
    );

    let _ = fs::remove_dir_all(&sandbox);
}

// Java Project Fixture Test

#[test]
fn test_java_fixture_validity_and_compilation() {
    let fixture_src = PathBuf::from("tests/fixtures/java_project");
    let sandbox = PathBuf::from("target/test_sandbox/java_project");

    let _ = fs::remove_dir_all(&sandbox);
    copy_dir_all(&fixture_src, &sandbox).unwrap();

    let scanner = ProjectScanner::new(Confidence::High);

    let initial_matches = scanner.scan_path(&sandbox).unwrap();
    assert!(
        !initial_matches.is_empty(),
        "Should detect secrets in initial Java fixture"
    );

    let plan = build_fix_plan(&sandbox, &initial_matches).unwrap();
    apply_fix_plan_atomically(&sandbox, &plan).unwrap();

    let after_matches = scanner.scan_path(&sandbox).unwrap();
    assert_eq!(
        after_matches.len(),
        0,
        "No secrets should remain after fix in Java fixture"
    );

    let output = Command::new("javac")
        .arg(sandbox.join("src/Config.java"))
        .arg(sandbox.join("src/App.java"))
        .output()
        .expect("Failed to execute javac");

    assert!(
        output.status.success(),
        "javac compilation failed on refactored Java code! stderr: {}",
        String::from_utf8_lossy(&output.stderr)
    );

    let _ = fs::remove_dir_all(&sandbox);
}

// Python Project Fixture Test

#[test]
fn test_python_fixture_validity_and_compilation() {
    let fixture_src = PathBuf::from("tests/fixtures/python_project");
    let sandbox = PathBuf::from("target/test_sandbox/python_project");

    let _ = fs::remove_dir_all(&sandbox);
    copy_dir_all(&fixture_src, &sandbox).unwrap();

    let scanner = ProjectScanner::new(Confidence::High);

    let initial_matches = scanner.scan_path(&sandbox).unwrap();
    assert!(
        !initial_matches.is_empty(),
        "Should detect secrets in initial Python fixture"
    );

    let plan = build_fix_plan(&sandbox, &initial_matches).unwrap();
    apply_fix_plan_atomically(&sandbox, &plan).unwrap();

    let after_matches = scanner.scan_path(&sandbox).unwrap();
    assert_eq!(
        after_matches.len(),
        0,
        "No secrets should remain after fix in Python fixture"
    );

    let output = Command::new("python3")
        .arg("-m")
        .arg("py_compile")
        .arg(sandbox.join("main.py"))
        .arg(sandbox.join("config.py"))
        .arg(sandbox.join("services/client.py"))
        .output()
        .expect("Failed to execute python3 py_compile");

    assert!(
        output.status.success(),
        "Python py_compile failed on refactored Python code! stderr: {}",
        String::from_utf8_lossy(&output.stderr)
    );

    let _ = fs::remove_dir_all(&sandbox);
}

// JavaScript Project Fixture Test

#[test]
fn test_javascript_fixture_validity_and_compilation() {
    let fixture_src = PathBuf::from("tests/fixtures/javascript_project");
    let sandbox = PathBuf::from("target/test_sandbox/javascript_project");

    let _ = fs::remove_dir_all(&sandbox);
    copy_dir_all(&fixture_src, &sandbox).unwrap();

    let scanner = ProjectScanner::new(Confidence::High);

    let initial_matches = scanner.scan_path(&sandbox).unwrap();
    assert!(
        !initial_matches.is_empty(),
        "Should detect secrets in initial JS fixture"
    );

    let plan = build_fix_plan(&sandbox, &initial_matches).unwrap();
    apply_fix_plan_atomically(&sandbox, &plan).unwrap();

    let after_matches = scanner.scan_path(&sandbox).unwrap();
    assert_eq!(
        after_matches.len(),
        0,
        "No secrets should remain after fix in JS fixture"
    );

    let output = Command::new("node")
        .arg("-c")
        .arg(sandbox.join("index.js"))
        .arg(sandbox.join("config.js"))
        .arg(sandbox.join("api/client.js"))
        .output()
        .expect("Failed to execute node -c");

    assert!(
        output.status.success(),
        "node syntax check failed on refactored JavaScript code! stderr: {}",
        String::from_utf8_lossy(&output.stderr)
    );

    let _ = fs::remove_dir_all(&sandbox);
}

// TypeScript Project Fixture Test

#[test]
fn test_typescript_fixture_validity() {
    let fixture_src = PathBuf::from("tests/fixtures/typescript_project");
    let sandbox = PathBuf::from("target/test_sandbox/typescript_project");

    let _ = fs::remove_dir_all(&sandbox);
    copy_dir_all(&fixture_src, &sandbox).unwrap();

    let scanner = ProjectScanner::new(Confidence::High);

    let initial_matches = scanner.scan_path(&sandbox).unwrap();
    assert!(
        !initial_matches.is_empty(),
        "Should detect secrets in initial TS fixture"
    );

    let plan = build_fix_plan(&sandbox, &initial_matches).unwrap();
    apply_fix_plan_atomically(&sandbox, &plan).unwrap();

    let after_matches = scanner.scan_path(&sandbox).unwrap();
    assert_eq!(
        after_matches.len(),
        0,
        "No secrets should remain after fix in TS fixture"
    );

    let config_content = fs::read_to_string(sandbox.join("src/config.ts")).unwrap();
    assert!(config_content.contains("process.env.OPENAI_KEY || \"\""));

    let _ = fs::remove_dir_all(&sandbox);
}
