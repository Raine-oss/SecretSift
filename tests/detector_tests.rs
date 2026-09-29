use secretsift::detector::{calculate_entropy, detect_secrets_in_content, normalize_env_var_name};
use secretsift::models::{Confidence, Language, SecretType};
use std::path::Path;

// Shannon Entropy Tests

#[test]
fn test_calculate_entropy() {
    let low_entropy = calculate_entropy("aaaaaaa");
    assert_eq!(low_entropy, 0.0);

    let high_entropy = calculate_entropy("a8F9#kL2!zQ90w");
    assert!(high_entropy > 3.0);
}

// Variable Name Normalization Tests

#[test]
fn test_normalize_env_var_name() {
    assert_eq!(normalize_env_var_name("apiKey"), "API_KEY");
    assert_eq!(normalize_env_var_name("discord_token"), "DISCORD_TOKEN");
    assert_eq!(normalize_env_var_name("DATABASE-URL"), "DATABASE_URL");
}

// High Confidence Detection Tests

#[test]
fn test_detect_openai_and_db_url() {
    let dummy_path = Path::new("src/config.rs");
    let content = "const API_KEY = \"sk-abcdef1234567890abcdef1234567890\";\nlet db_url = \"postgres://admin:supersecretpassword@localhost:5432/mydb\";\n";

    let matches = detect_secrets_in_content(dummy_path, content, Language::Rust);
    assert_eq!(matches.len(), 2);

    let openai_match = matches
        .iter()
        .find(|m| m.secret_type == SecretType::OpenAiApiKey)
        .unwrap();
    assert_eq!(openai_match.confidence, Confidence::High);

    let db_match = matches
        .iter()
        .find(|m| m.secret_type == SecretType::DatabaseUrl)
        .unwrap();
    assert_eq!(db_match.confidence, Confidence::High);
}

// False Positive Filtering Tests

#[test]
fn test_ignore_placeholders() {
    let dummy_path = Path::new("src/config.py");
    let content = "API_KEY = \"your-api-key-here\"\nDUMMY_SECRET = \"placeholder_value\"\n";

    let matches = detect_secrets_in_content(dummy_path, content, Language::Python);
    assert_eq!(matches.len(), 0);
}
