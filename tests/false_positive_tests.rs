use secretsift::detector::detect_secrets_in_content;
use secretsift::models::Language;
use std::path::Path;

// Normal Strings Test

#[test]
fn test_normal_strings_not_detected() {
    let dummy_path = Path::new("src/utils.rs");
    let content = r#"
        let app_title = "SecretSift Developer Security Suite";
        let message = "Successfully processed repository without errors.";
        let endpoint = "/api/v1/users/profile";
        let status = "ACTIVE_PENDING_CONFIRMATION";
    "#;

    let matches = detect_secrets_in_content(dummy_path, content, Language::Rust);
    assert_eq!(matches.len(), 0);
}

// Existing Env Variables Test

#[test]
fn test_existing_env_calls_not_detected() {
    let rs_path = Path::new("src/config.rs");
    let rs_content = r#"
        let api_key = std::env::var("API_KEY").unwrap_or_default();
        let db_url = env!("DATABASE_URL");
    "#;
    let rs_matches = detect_secrets_in_content(rs_path, rs_content, Language::Rust);
    assert_eq!(rs_matches.len(), 0);

    let js_path = Path::new("src/config.js");
    let js_content = r#"
        const API_KEY = process.env.API_KEY || "";
        const config = {
            dbUrl: process.env.DATABASE_URL
        };
    "#;
    let js_matches = detect_secrets_in_content(js_path, js_content, Language::JavaScript);
    assert_eq!(js_matches.len(), 0);

    let py_path = Path::new("src/config.py");
    let py_content = r#"
        import os

        API_KEY = os.getenv("API_KEY")
        SECRET = os.environ.get("SECRET_KEY")
    "#;
    let py_matches = detect_secrets_in_content(py_path, py_content, Language::Python);
    assert_eq!(py_matches.len(), 0);

    let java_path = Path::new("src/Config.java");
    let java_content = r#"
        public class Config {
            public static final String API_KEY = System.getenv("API_KEY");
        }
    "#;
    let java_matches = detect_secrets_in_content(java_path, java_content, Language::Java);
    assert_eq!(java_matches.len(), 0);

    let go_path = Path::new("main.go");
    let go_content = r#"
        apiKey := os.Getenv("API_KEY")
    "#;
    let go_matches = detect_secrets_in_content(go_path, go_content, Language::Go);
    assert_eq!(go_matches.len(), 0);

    let cs_path = Path::new("Program.cs");
    let cs_content = r#"
        string key = Environment.GetEnvironmentVariable("API_KEY") ?? "";
    "#;
    let cs_matches = detect_secrets_in_content(cs_path, cs_content, Language::CSharp);
    assert_eq!(cs_matches.len(), 0);

    let kt_path = Path::new("Main.kt");
    let kt_content = r#"
        val key = System.getenv("API_KEY") ?: ""
    "#;
    let kt_matches = detect_secrets_in_content(kt_path, kt_content, Language::Kotlin);
    assert_eq!(kt_matches.len(), 0);

    let php_path = Path::new("config.php");
    let php_content = r#"
        $key = getenv("API_KEY") ?: "";
        $sec = $_ENV["SECRET_KEY"] ?? "";
    "#;
    let php_matches = detect_secrets_in_content(php_path, php_content, Language::Php);
    assert_eq!(php_matches.len(), 0);

    let rb_path = Path::new("config.rb");
    let rb_content = r#"
        API_KEY = ENV["API_KEY"] || ""
    "#;
    let rb_matches = detect_secrets_in_content(rb_path, rb_content, Language::Ruby);
    assert_eq!(rb_matches.len(), 0);

    let dart_path = Path::new("main.dart");
    let dart_content = r#"
        final key = Platform.environment['API_KEY'] ?? '';
    "#;
    let dart_matches = detect_secrets_in_content(dart_path, dart_content, Language::Dart);
    assert_eq!(dart_matches.len(), 0);

    let swift_path = Path::new("main.swift");
    let swift_content = r#"
        let key = ProcessInfo.processInfo.environment["API_KEY"] ?? ""
    "#;
    let swift_matches = detect_secrets_in_content(swift_path, swift_content, Language::Swift);
    assert_eq!(swift_matches.len(), 0);
}

// Fake and Placeholder Tokens Test

#[test]
fn test_fake_and_placeholder_tokens_filtered() {
    let dummy_path = Path::new("src/sample.rs");
    let content = r#"
        let key1 = "your-api-key-here";
        let key2 = "12345678";
        let key3 = "placeholder";
        let key4 = "changeme";
        let key5 = "test_token";
    "#;

    let matches = detect_secrets_in_content(dummy_path, content, Language::Rust);
    assert_eq!(matches.len(), 0);
}
