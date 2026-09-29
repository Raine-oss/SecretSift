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
fn test_js_ts_adapter_object_and_fallback() {
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
