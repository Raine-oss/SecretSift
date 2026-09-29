pub mod rules;

use crate::models::{Confidence, Language, SecretMatch, SecretType};
use rules::{get_high_confidence_rules, match_assignments};
use std::collections::HashMap;
use std::path::Path;

// Shannon Entropy

pub fn calculate_entropy(s: &str) -> f64 {
    if s.is_empty() {
        return 0.0;
    }
    let mut map = HashMap::new();
    for ch in s.chars() {
        *map.entry(ch).or_insert(0usize) += 1;
    }
    let len = s.chars().count() as f64;
    let mut entropy = 0.0;
    for &count in map.values() {
        let p = (count as f64) / len;
        entropy -= p * p.log2();
    }
    entropy
}

// Key Normalization

pub fn normalize_env_var_name(name: &str) -> String {
    let mut result = String::new();
    let chars: Vec<char> = name.chars().collect();
    for (i, &ch) in chars.iter().enumerate() {
        if ch == '-' || ch == '.' || ch == ' ' || ch == '_' {
            if !result.ends_with('_') && !result.is_empty() {
                result.push('_');
            }
        } else if ch.is_uppercase() {
            if i > 0
                && !chars[i - 1].is_uppercase()
                && chars[i - 1] != '_'
                && chars[i - 1] != '-'
                && !result.ends_with('_')
            {
                result.push('_');
            }
            result.push(ch);
        } else {
            result.push(ch.to_ascii_uppercase());
        }
    }
    if result.is_empty() {
        "SECRET_VALUE".to_string()
    } else {
        result
    }
}

// False Positive Filter

pub fn is_likely_false_positive(value: &str, var_name: &str) -> bool {
    let lower_val = value.to_lowercase();
    let lower_name = var_name.to_lowercase();

    let exact_placeholders = [
        "123456",
        "12345678",
        "123456789",
        "password",
        "password123",
        "secret",
        "mysecret",
        "my_secret",
        "xxxx",
        "xxxxxx",
        "todo",
    ];

    if exact_placeholders.iter().any(|&p| lower_val == p) {
        return true;
    }

    let substring_placeholders = [
        "your-api-key",
        "your_api_key",
        "yourapikey",
        "dummy",
        "placeholder",
        "example",
        "changeme",
        "test_secret",
        "test_token",
    ];

    for word in &substring_placeholders {
        if lower_val.contains(word) {
            return true;
        }
    }

    if (lower_name.contains("test") || lower_name.contains("mock") || lower_name.contains("sample"))
        && value.len() < 10
    {
        return true;
    }

    false
}

// Environment Accessor Check

pub fn is_line_already_using_env(line: &str) -> bool {
    let trimmed = line.trim();
    trimmed.contains("std::env::var(")
        || trimmed.contains("env::var(")
        || trimmed.contains("env!(")
        || trimmed.contains("System.getenv(")
        || trimmed.contains("process.env.")
        || trimmed.contains("process.env[")
        || trimmed.contains("os.getenv(")
        || trimmed.contains("os.environ.get(")
        || trimmed.contains("os.environ[")
        || trimmed.contains("os.Getenv(")
        || trimmed.contains("os.LookupEnv(")
        || trimmed.contains("Environment.GetEnvironmentVariable(")
        || trimmed.contains("getenv(")
        || trimmed.contains("$_ENV[")
        || trimmed.contains("$_SERVER[")
        || trimmed.contains("ENV[")
        || trimmed.contains("ENV.fetch(")
        || trimmed.contains("Platform.environment[")
        || trimmed.contains("ProcessInfo.processInfo.environment[")
        || trimmed.contains("import.meta.env.")
}

// Central Detector

pub fn detect_secrets_in_content(
    file_path: &Path,
    content: &str,
    language: Language,
) -> Vec<SecretMatch> {
    let mut findings = Vec::new();
    let high_rules = get_high_confidence_rules();

    for (line_idx, line) in content.lines().enumerate() {
        let line_number = line_idx + 1;

        if is_line_already_using_env(line) {
            continue;
        }

        let mut matched_spans: Vec<(usize, usize)> = Vec::new();

        for rule in &high_rules {
            for mat in rule.regex.find_iter(line) {
                let val = mat.as_str().to_string();
                if is_likely_false_positive(&val, rule.default_var_name) {
                    continue;
                }

                matched_spans.push((mat.start(), mat.end()));

                findings.push(SecretMatch {
                    file_path: file_path.to_path_buf(),
                    line_number,
                    column: mat.start() + 1,
                    matched_value: val,
                    raw_line: line.to_string(),
                    suggested_var_name: rule.default_var_name.to_string(),
                    secret_type: rule.secret_type.clone(),
                    confidence: rule.confidence,
                    language,
                });
            }
        }

        let assignments = match_assignments(line);
        for (raw_var_name, val, start, end) in assignments {
            let already_covered = matched_spans
                .iter()
                .any(|&(s, e)| (start >= s && start < e) || (end > s && end <= e));

            if already_covered {
                if let Some(last_match) = findings.last_mut() {
                    if last_match.line_number == line_number {
                        last_match.suggested_var_name = normalize_env_var_name(&raw_var_name);
                    }
                }
                continue;
            }

            if is_likely_false_positive(&val, &raw_var_name) {
                continue;
            }

            let lower_name = raw_var_name.to_lowercase();
            let is_secret_name = lower_name.contains("key")
                || lower_name.contains("secret")
                || lower_name.contains("token")
                || lower_name.contains("password")
                || lower_name.contains("passwd")
                || lower_name.contains("auth")
                || lower_name.contains("db_url")
                || lower_name.contains("database_url");

            if is_secret_name {
                let entropy = calculate_entropy(&val);
                let confidence = if entropy >= 3.2 && val.len() >= 12 {
                    Confidence::High
                } else if entropy >= 2.5 && val.len() >= 8 {
                    Confidence::Medium
                } else {
                    Confidence::Low
                };

                findings.push(SecretMatch {
                    file_path: file_path.to_path_buf(),
                    line_number,
                    column: start + 1,
                    matched_value: val,
                    raw_line: line.to_string(),
                    suggested_var_name: normalize_env_var_name(&raw_var_name),
                    secret_type: SecretType::GenericSecret,
                    confidence,
                    language,
                });
            }
        }
    }

    findings
}
