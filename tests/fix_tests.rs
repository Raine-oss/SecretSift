use secretsift::fix::env::generate_env_files;
use std::fs;
use std::path::PathBuf;

// Environment Generation Tests

#[test]
fn test_generate_env_and_example() {
    let temp_dir = PathBuf::from("target/test_temp_env");
    let _ = fs::remove_dir_all(&temp_dir);
    fs::create_dir_all(&temp_dir).unwrap();

    let extracted = vec![
        ("API_KEY".to_string(), "abc123secret".to_string()),
        (
            "DATABASE_URL".to_string(),
            "postgres://user:pass@localhost/db".to_string(),
        ),
    ];

    let result = generate_env_files(&temp_dir, &extracted);

    assert!(result.env_content.contains("API_KEY=abc123secret"));
    assert!(result
        .env_content
        .contains("DATABASE_URL=postgres://user:pass@localhost/db"));

    assert!(result.example_content.contains("API_KEY="));
    assert!(result.example_content.contains("DATABASE_URL="));

    let _ = fs::remove_dir_all(&temp_dir);
}
