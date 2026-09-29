use secretsift::adapters::get_adapter_for_language;
use secretsift::models::{Confidence, Language, SecretMatch, SecretType};
use std::path::PathBuf;

// Helper Match Creator

fn create_match(val: &str, var_name: &str, lang: Language) -> SecretMatch {
    SecretMatch {
        file_path: PathBuf::from("test"),
        line_number: 1,
        column: 1,
        matched_value: val.to_string(),
        raw_line: String::new(),
        suggested_var_name: var_name.to_string(),
        secret_type: SecretType::GenericSecret,
        confidence: Confidence::High,
        language: lang,
    }
}

// Rust Adapter Tests

#[test]
fn test_rust_adapter_fn_argument() {
    let adapter = get_adapter_for_language(Language::Rust).unwrap();
    let m = create_match("sk-1234567890abcdef", "API_KEY", Language::Rust);

    let line = r#"let client = Client::new("sk-1234567890abcdef");"#;
    let rewritten = adapter.rewrite_line(&m, line).unwrap();
    assert_eq!(
        rewritten,
        r#"let client = Client::new(&std::env::var("API_KEY").unwrap_or_default());"#
    );
}

#[test]
fn test_rust_adapter_format_string() {
    let adapter = get_adapter_for_language(Language::Rust).unwrap();
    let m = create_match("sk-1234567890abcdef", "API_KEY", Language::Rust);

    let line = r#"let url = format!("https://example.com?key=sk-1234567890abcdef");"#;
    let rewritten = adapter.rewrite_line(&m, line).unwrap();
    assert_eq!(
        rewritten,
        r#"let url = format!("https://example.com?key={}", std::env::var("API_KEY").unwrap_or_default());"#
    );
}

// Java Adapter Tests

#[test]
fn test_java_adapter_various_contexts() {
    let adapter = get_adapter_for_language(Language::Java).unwrap();
    let m = create_match("supersecret123", "API_KEY", Language::Java);

    let line1 = r#"private static final String API_KEY = "supersecret123";"#;
    assert_eq!(
        adapter.rewrite_line(&m, line1).unwrap(),
        r#"private static final String API_KEY = System.getenv("API_KEY");"#
    );

    let line2 = r#"String token = "supersecret123";"#;
    assert_eq!(
        adapter.rewrite_line(&m, line2).unwrap(),
        r#"String token = System.getenv("API_KEY");"#
    );

    let line3 = r#"foo("supersecret123");"#;
    assert_eq!(
        adapter.rewrite_line(&m, line3).unwrap(),
        r#"foo(System.getenv("API_KEY"));"#
    );

    let line4 = r#"return "supersecret123";"#;
    assert_eq!(
        adapter.rewrite_line(&m, line4).unwrap(),
        r#"return System.getenv("API_KEY");"#
    );
}

// JS / TS Adapter Tests

#[test]
fn test_javascript_adapter() {
    let adapter = get_adapter_for_language(Language::JavaScript).unwrap();
    let m = create_match("secret_token_123", "API_KEY", Language::JavaScript);

    let line1 = r#"const API_KEY = "secret_token_123";"#;
    assert_eq!(
        adapter.rewrite_line(&m, line1).unwrap(),
        r#"const API_KEY = process.env.API_KEY || "";"#
    );

    let line2 = r#"    apiKey: "secret_token_123""#;
    assert_eq!(
        adapter.rewrite_line(&m, line2).unwrap(),
        r#"    apiKey: process.env.API_KEY || """#
    );
}

#[test]
fn test_typescript_adapter() {
    let adapter = get_adapter_for_language(Language::TypeScript).unwrap();
    let m = create_match("secret_token_456", "AUTH_TOKEN", Language::TypeScript);

    let line1 = r#"const token: string = 'secret_token_456';"#;
    assert_eq!(
        adapter.rewrite_line(&m, line1).unwrap(),
        r#"const token: string = process.env.AUTH_TOKEN || "";"#
    );
}

// Python Adapter Tests

#[test]
fn test_python_adapter_clean_import_placement() {
    let adapter = get_adapter_for_language(Language::Python).unwrap();
    let m = create_match("secret_val_123", "API_KEY", Language::Python);

    let raw_code = "import requests\nimport json\n\nAPI_KEY = \"secret_val_123\"\n";
    let rewritten_line = adapter
        .rewrite_line(&m, "API_KEY = \"secret_val_123\"")
        .unwrap();
    let intermediate = raw_code.replace("API_KEY = \"secret_val_123\"", &rewritten_line);

    let final_code = adapter.post_process_file_content(&intermediate);
    assert_eq!(
        final_code,
        "import os\nimport requests\nimport json\n\nAPI_KEY = os.getenv(\"API_KEY\")\n"
    );
}

// Go Adapter Tests

#[test]
fn test_go_adapter_rewrite_and_import() {
    let adapter = get_adapter_for_language(Language::Go).unwrap();
    let m = create_match("super_go_secret_123", "DB_PASSWORD", Language::Go);

    let line = r#"const dbPassword = "super_go_secret_123""#;
    let rewritten = adapter.rewrite_line(&m, line).unwrap();
    assert_eq!(rewritten, r#"const dbPassword = os.Getenv("DB_PASSWORD")"#);

    let go_code = "package main\n\nimport (\n\t\"fmt\"\n)\n\nfunc main() {\n\tconst p = os.Getenv(\"DB_PASSWORD\")\n}\n";
    let post_processed = adapter.post_process_file_content(go_code);
    assert!(post_processed.contains("\t\"os\""));
    assert!(post_processed.contains("\t\"fmt\""));
}

// C# Adapter Tests

#[test]
fn test_csharp_adapter_rewrite_and_using() {
    let adapter = get_adapter_for_language(Language::CSharp).unwrap();
    let m = create_match("cs_secret_key_789", "API_KEY", Language::CSharp);

    let line = r#"private static string ApiKey = "cs_secret_key_789";"#;
    let rewritten = adapter.rewrite_line(&m, line).unwrap();
    assert_eq!(
        rewritten,
        r#"private static string ApiKey = Environment.GetEnvironmentVariable("API_KEY") ?? "";"#
    );

    let cs_with_env = format!(
        "using App;\n\nnamespace App {{\n    class Program {{\n        string k = {};\n    }}\n}}\n",
        rewritten
    );
    let post_processed = adapter.post_process_file_content(&cs_with_env);
    assert!(post_processed.starts_with("using System;"));
}

// Kotlin Adapter Tests

#[test]
fn test_kotlin_adapter_rewrite() {
    let adapter = get_adapter_for_language(Language::Kotlin).unwrap();
    let m = create_match("kt_secret_val_321", "AUTH_KEY", Language::Kotlin);

    let line = r#"val authKey = "kt_secret_val_321""#;
    let rewritten = adapter.rewrite_line(&m, line).unwrap();
    assert_eq!(
        rewritten,
        r#"val authKey = System.getenv("AUTH_KEY") ?: """#
    );
}

// PHP Adapter Tests

#[test]
fn test_php_adapter_rewrite() {
    let adapter = get_adapter_for_language(Language::Php).unwrap();
    let m = create_match("php_secret_key_999", "STRIPE_SECRET", Language::Php);

    let line = r#"$stripeKey = "php_secret_key_999";"#;
    let rewritten = adapter.rewrite_line(&m, line).unwrap();
    assert_eq!(rewritten, r#"$stripeKey = getenv('STRIPE_SECRET') ?: '';"#);
}

// Ruby Adapter Tests

#[test]
fn test_ruby_adapter_rewrite() {
    let adapter = get_adapter_for_language(Language::Ruby).unwrap();
    let m = create_match("ruby_secret_key_888", "API_TOKEN", Language::Ruby);

    let line = r#"API_KEY = "ruby_secret_key_888""#;
    let rewritten = adapter.rewrite_line(&m, line).unwrap();
    assert_eq!(rewritten, r#"API_KEY = ENV['API_TOKEN'] || ''"#);
}

// C / C++ Adapter Tests

#[test]
fn test_cpp_adapter_rewrite_and_include() {
    let adapter_c = get_adapter_for_language(Language::C).unwrap();
    let adapter_cpp = get_adapter_for_language(Language::Cpp).unwrap();
    let m_c = create_match("c_secret_111", "C_SECRET", Language::C);
    let m_cpp = create_match("cpp_secret_222", "CPP_SECRET", Language::Cpp);

    let line_c = r#"const char* secret = "c_secret_111";"#;
    assert_eq!(
        adapter_c.rewrite_line(&m_c, line_c).unwrap(),
        r#"const char* secret = getenv("C_SECRET");"#
    );

    let line_cpp = r#"const char* secret = "cpp_secret_222";"#;
    assert_eq!(
        adapter_cpp.rewrite_line(&m_cpp, line_cpp).unwrap(),
        r#"const char* secret = getenv("CPP_SECRET");"#
    );

    let raw_c = "int main() { const char* s = getenv(\"C_SECRET\"); return 0; }\n";
    let post_c = adapter_c.post_process_file_content(raw_c);
    assert!(post_c.contains("#include <stdlib.h>"));

    let raw_cpp = "int main() { const char* s = getenv(\"CPP_SECRET\"); return 0; }\n";
    let post_cpp = adapter_cpp.post_process_file_content(raw_cpp);
    assert!(post_cpp.contains("#include <cstdlib>"));
}

// Dart Adapter Tests

#[test]
fn test_dart_adapter_rewrite_and_import() {
    let adapter = get_adapter_for_language(Language::Dart).unwrap();
    let m = create_match("dart_secret_333", "DART_KEY", Language::Dart);

    let line = r#"final apiKey = 'dart_secret_333';"#;
    let rewritten = adapter.rewrite_line(&m, line).unwrap();
    assert_eq!(
        rewritten,
        r#"final apiKey = Platform.environment['DART_KEY'] ?? '';"#
    );

    let raw_dart = "void main() {\n  final k = Platform.environment['DART_KEY'] ?? '';\n}\n";
    let post_dart = adapter.post_process_file_content(raw_dart);
    assert!(post_dart.starts_with("import 'dart:io';"));
}

// Swift Adapter Tests

#[test]
fn test_swift_adapter_rewrite_and_import() {
    let adapter = get_adapter_for_language(Language::Swift).unwrap();
    let m = create_match("swift_secret_444", "SWIFT_KEY", Language::Swift);

    let line = r#"let key = "swift_secret_444""#;
    let rewritten = adapter.rewrite_line(&m, line).unwrap();
    assert_eq!(
        rewritten,
        r#"let key = ProcessInfo.processInfo.environment["SWIFT_KEY"] ?? """#
    );

    let raw_swift =
        "let key = ProcessInfo.processInfo.environment[\"SWIFT_KEY\"] ?? \"\"\nprint(key)\n";
    let post_swift = adapter.post_process_file_content(raw_swift);
    assert!(post_swift.starts_with("import Foundation"));
}
