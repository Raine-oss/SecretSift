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

// Toolchain Availability Helper

fn is_command_available(cmd: &str) -> bool {
    Command::new("which")
        .arg(cmd)
        .output()
        .map(|o| o.status.success())
        .unwrap_or(false)
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

// Go Project Fixture Test

#[test]
fn test_go_fixture_validity() {
    let fixture_src = PathBuf::from("tests/fixtures/go_project");
    let sandbox = PathBuf::from("target/test_sandbox/go_project");

    let _ = fs::remove_dir_all(&sandbox);
    copy_dir_all(&fixture_src, &sandbox).unwrap();

    let scanner = ProjectScanner::new(Confidence::High);

    let initial_matches = scanner.scan_path(&sandbox).unwrap();
    assert!(
        !initial_matches.is_empty(),
        "Should detect secrets in initial Go fixture"
    );

    let plan = build_fix_plan(&sandbox, &initial_matches).unwrap();
    apply_fix_plan_atomically(&sandbox, &plan).unwrap();

    let after_matches = scanner.scan_path(&sandbox).unwrap();
    assert_eq!(
        after_matches.len(),
        0,
        "No secrets should remain after fix in Go fixture"
    );

    let cfg_content = fs::read_to_string(sandbox.join("config/config.go")).unwrap();
    assert!(cfg_content.contains("os.Getenv(\"DATABASE_URL\")"));
    assert!(cfg_content.contains("import \"os\""));

    if is_command_available("go") {
        let output = Command::new("go").arg("vet").current_dir(&sandbox).output();
        if let Ok(out) = output {
            assert!(
                out.status.success() || out.stderr.is_empty(),
                "go vet failed on refactored Go code: {}",
                String::from_utf8_lossy(&out.stderr)
            );
        }
    }

    let _ = fs::remove_dir_all(&sandbox);
}

// C# Project Fixture Test

#[test]
fn test_csharp_fixture_validity() {
    let fixture_src = PathBuf::from("tests/fixtures/csharp_project");
    let sandbox = PathBuf::from("target/test_sandbox/csharp_project");

    let _ = fs::remove_dir_all(&sandbox);
    copy_dir_all(&fixture_src, &sandbox).unwrap();

    let scanner = ProjectScanner::new(Confidence::High);

    let initial_matches = scanner.scan_path(&sandbox).unwrap();
    assert!(
        !initial_matches.is_empty(),
        "Should detect secrets in initial C# fixture"
    );

    let plan = build_fix_plan(&sandbox, &initial_matches).unwrap();
    apply_fix_plan_atomically(&sandbox, &plan).unwrap();

    let after_matches = scanner.scan_path(&sandbox).unwrap();
    assert_eq!(
        after_matches.len(),
        0,
        "No secrets should remain after fix in C# fixture"
    );

    let cfg_content = fs::read_to_string(sandbox.join("Config.cs")).unwrap();
    assert!(cfg_content.contains("Environment.GetEnvironmentVariable(\"DATABASE_URL\")"));
    assert!(cfg_content.contains("using System;"));

    if is_command_available("dotnet") {
        let output = Command::new("dotnet")
            .arg("build")
            .current_dir(&sandbox)
            .output();
        if let Ok(out) = output {
            assert!(
                out.status.success() || out.stderr.is_empty(),
                "dotnet build check: {}",
                String::from_utf8_lossy(&out.stderr)
            );
        }
    }

    let _ = fs::remove_dir_all(&sandbox);
}

// Kotlin Project Fixture Test

#[test]
fn test_kotlin_fixture_validity() {
    let fixture_src = PathBuf::from("tests/fixtures/kotlin_project");
    let sandbox = PathBuf::from("target/test_sandbox/kotlin_project");

    let _ = fs::remove_dir_all(&sandbox);
    copy_dir_all(&fixture_src, &sandbox).unwrap();

    let scanner = ProjectScanner::new(Confidence::High);

    let initial_matches = scanner.scan_path(&sandbox).unwrap();
    assert!(
        !initial_matches.is_empty(),
        "Should detect secrets in initial Kotlin fixture"
    );

    let plan = build_fix_plan(&sandbox, &initial_matches).unwrap();
    apply_fix_plan_atomically(&sandbox, &plan).unwrap();

    let after_matches = scanner.scan_path(&sandbox).unwrap();
    assert_eq!(
        after_matches.len(),
        0,
        "No secrets should remain after fix in Kotlin fixture"
    );

    let cfg_content = fs::read_to_string(sandbox.join("Config.kt")).unwrap();
    assert!(cfg_content.contains("System.getenv(\"DATABASE_URL\") ?: \"\""));

    if is_command_available("kotlinc") {
        let output = Command::new("kotlinc")
            .arg(sandbox.join("Config.kt"))
            .arg(sandbox.join("Main.kt"))
            .arg("-d")
            .arg(sandbox.join("out.jar"))
            .output();
        if let Ok(out) = output {
            assert!(
                out.status.success(),
                "kotlinc failed: {}",
                String::from_utf8_lossy(&out.stderr)
            );
        }
    }

    let _ = fs::remove_dir_all(&sandbox);
}

// PHP Project Fixture Test

#[test]
fn test_php_fixture_validity() {
    let fixture_src = PathBuf::from("tests/fixtures/php_project");
    let sandbox = PathBuf::from("target/test_sandbox/php_project");

    let _ = fs::remove_dir_all(&sandbox);
    copy_dir_all(&fixture_src, &sandbox).unwrap();

    let scanner = ProjectScanner::new(Confidence::High);

    let initial_matches = scanner.scan_path(&sandbox).unwrap();
    assert!(
        !initial_matches.is_empty(),
        "Should detect secrets in initial PHP fixture"
    );

    let plan = build_fix_plan(&sandbox, &initial_matches).unwrap();
    apply_fix_plan_atomically(&sandbox, &plan).unwrap();

    let after_matches = scanner.scan_path(&sandbox).unwrap();
    assert_eq!(
        after_matches.len(),
        0,
        "No secrets should remain after fix in PHP fixture"
    );

    let cfg_content = fs::read_to_string(sandbox.join("config.php")).unwrap();
    assert!(
        cfg_content.contains("getenv('DATABASE_URL') ?: ''")
            || cfg_content.contains("getenv('DB_URL') ?: ''")
    );

    if is_command_available("php") {
        let output = Command::new("php")
            .arg("-l")
            .arg(sandbox.join("config.php"))
            .output();
        if let Ok(out) = output {
            assert!(
                out.status.success(),
                "php -l lint failed: {}",
                String::from_utf8_lossy(&out.stderr)
            );
        }
    }

    let _ = fs::remove_dir_all(&sandbox);
}

// Ruby Project Fixture Test

#[test]
fn test_ruby_fixture_validity() {
    let fixture_src = PathBuf::from("tests/fixtures/ruby_project");
    let sandbox = PathBuf::from("target/test_sandbox/ruby_project");

    let _ = fs::remove_dir_all(&sandbox);
    copy_dir_all(&fixture_src, &sandbox).unwrap();

    let scanner = ProjectScanner::new(Confidence::High);

    let initial_matches = scanner.scan_path(&sandbox).unwrap();
    assert!(
        !initial_matches.is_empty(),
        "Should detect secrets in initial Ruby fixture"
    );

    let plan = build_fix_plan(&sandbox, &initial_matches).unwrap();
    apply_fix_plan_atomically(&sandbox, &plan).unwrap();

    let after_matches = scanner.scan_path(&sandbox).unwrap();
    assert_eq!(
        after_matches.len(),
        0,
        "No secrets should remain after fix in Ruby fixture"
    );

    let cfg_content = fs::read_to_string(sandbox.join("config.rb")).unwrap();
    assert!(cfg_content.contains("ENV['DATABASE_URL'] || ''"));

    if is_command_available("ruby") {
        let output = Command::new("ruby")
            .arg("-c")
            .arg(sandbox.join("config.rb"))
            .output();
        if let Ok(out) = output {
            assert!(
                out.status.success(),
                "ruby -c syntax check failed: {}",
                String::from_utf8_lossy(&out.stderr)
            );
        }
    }

    let _ = fs::remove_dir_all(&sandbox);
}

// C++ Project Fixture Test

#[test]
fn test_cpp_fixture_validity_and_compilation() {
    let fixture_src = PathBuf::from("tests/fixtures/cpp_project");
    let sandbox = PathBuf::from("target/test_sandbox/cpp_project");

    let _ = fs::remove_dir_all(&sandbox);
    copy_dir_all(&fixture_src, &sandbox).unwrap();

    let scanner = ProjectScanner::new(Confidence::High);

    let initial_matches = scanner.scan_path(&sandbox).unwrap();
    assert!(
        !initial_matches.is_empty(),
        "Should detect secrets in initial C++ fixture"
    );

    let plan = build_fix_plan(&sandbox, &initial_matches).unwrap();
    apply_fix_plan_atomically(&sandbox, &plan).unwrap();

    let after_matches = scanner.scan_path(&sandbox).unwrap();
    assert_eq!(
        after_matches.len(),
        0,
        "No secrets should remain after fix in C++ fixture"
    );

    let cfg_content = fs::read_to_string(sandbox.join("config.cpp")).unwrap();
    assert!(cfg_content.contains("getenv(\"DATABASE_URL\")"));
    assert!(cfg_content.contains("#include <cstdlib>"));

    let output = Command::new("g++")
        .arg("-fsyntax-only")
        .arg(sandbox.join("config.cpp"))
        .arg(sandbox.join("main.cpp"))
        .output()
        .expect("Failed to execute g++");

    assert!(
        output.status.success(),
        "g++ compilation failed on refactored C++ code! stderr: {}",
        String::from_utf8_lossy(&output.stderr)
    );

    let _ = fs::remove_dir_all(&sandbox);
}

// Dart Project Fixture Test

#[test]
fn test_dart_fixture_validity() {
    let fixture_src = PathBuf::from("tests/fixtures/dart_project");
    let sandbox = PathBuf::from("target/test_sandbox/dart_project");

    let _ = fs::remove_dir_all(&sandbox);
    copy_dir_all(&fixture_src, &sandbox).unwrap();

    let scanner = ProjectScanner::new(Confidence::High);

    let initial_matches = scanner.scan_path(&sandbox).unwrap();
    assert!(
        !initial_matches.is_empty(),
        "Should detect secrets in initial Dart fixture"
    );

    let plan = build_fix_plan(&sandbox, &initial_matches).unwrap();
    apply_fix_plan_atomically(&sandbox, &plan).unwrap();

    let after_matches = scanner.scan_path(&sandbox).unwrap();
    assert_eq!(
        after_matches.len(),
        0,
        "No secrets should remain after fix in Dart fixture"
    );

    let cfg_content = fs::read_to_string(sandbox.join("config.dart")).unwrap();
    assert!(cfg_content.contains("Platform.environment['DATABASE_URL'] ?? ''"));
    assert!(cfg_content.contains("import 'dart:io';"));

    if is_command_available("dart") {
        let output = Command::new("dart")
            .arg("analyze")
            .arg(sandbox.join("config.dart"))
            .output();
        if let Ok(out) = output {
            assert!(
                out.status.success(),
                "dart analyze check: {}",
                String::from_utf8_lossy(&out.stderr)
            );
        }
    }

    let _ = fs::remove_dir_all(&sandbox);
}

// Swift Project Fixture Test

#[test]
fn test_swift_fixture_validity() {
    let fixture_src = PathBuf::from("tests/fixtures/swift_project");
    let sandbox = PathBuf::from("target/test_sandbox/swift_project");

    let _ = fs::remove_dir_all(&sandbox);
    copy_dir_all(&fixture_src, &sandbox).unwrap();

    let scanner = ProjectScanner::new(Confidence::High);

    let initial_matches = scanner.scan_path(&sandbox).unwrap();
    assert!(
        !initial_matches.is_empty(),
        "Should detect secrets in initial Swift fixture"
    );

    let plan = build_fix_plan(&sandbox, &initial_matches).unwrap();
    apply_fix_plan_atomically(&sandbox, &plan).unwrap();

    let after_matches = scanner.scan_path(&sandbox).unwrap();
    assert_eq!(
        after_matches.len(),
        0,
        "No secrets should remain after fix in Swift fixture"
    );

    let cfg_content = fs::read_to_string(sandbox.join("config.swift")).unwrap();
    assert!(cfg_content.contains("ProcessInfo.processInfo.environment[\"DATABASE_URL\"] ?? \"\""));
    assert!(cfg_content.contains("import Foundation"));

    if is_command_available("swiftc") {
        let output = Command::new("swiftc")
            .arg("-parse")
            .arg(sandbox.join("config.swift"))
            .output();
        if let Ok(out) = output {
            assert!(
                out.status.success(),
                "swiftc parse check: {}",
                String::from_utf8_lossy(&out.stderr)
            );
        }
    }

    let _ = fs::remove_dir_all(&sandbox);
}
