use crate::models::{Confidence, SecretType};
use lazy_static::lazy_static;
use regex::Regex;

// Pattern Definition

pub struct PatternRule {
    #[allow(dead_code)]
    pub name: &'static str,
    pub regex: &'static Regex,
    pub secret_type: SecretType,
    pub confidence: Confidence,
    pub default_var_name: &'static str,
}

// Regex Rules

lazy_static! {
    static ref RE_OPENAI: Regex = Regex::new(r#"\b(sk-[a-zA-Z0-9_\-]{20,})\b"#).unwrap();
    static ref RE_AWS_KEY: Regex = Regex::new(r#"\b(AKIA[0-9A-Z]{16})\b"#).unwrap();
    static ref RE_GITHUB_PAT: Regex = Regex::new(r#"\b(ghp_[0-9a-zA-Z]{36}|github_pat_[0-9a-zA-Z_]{82})\b"#).unwrap();
    static ref RE_SLACK_TOKEN: Regex = Regex::new(r#"\b(xox[baprs]-(?:[0-9a-zA-Z]+)(?:-[0-9a-zA-Z]+){1,5})\b"#).unwrap();
    static ref RE_STRIPE_KEY: Regex = Regex::new(r#"\b(sk_(?:live|test|sift)_[0-9a-zA-Z]{24,})\b"#).unwrap();
    static ref RE_DB_URL: Regex = Regex::new(r#"(postgres|postgresql|mysql|mongodb(?:\+srv)?|redis)://[a-zA-Z0-9_\-\.]+:[^@\s"']+@[a-zA-Z0-9_\.\-]+(?::[0-9]+)?/[a-zA-Z0-9_\.\-]+"#).unwrap();
    static ref RE_PRIVATE_KEY: Regex = Regex::new(r#"-----BEGIN (?:RSA |EC |DSA |OPENSSH )?PRIVATE KEY-----"#).unwrap();
    static ref RE_ASSIGNMENT: Regex = Regex::new(r#"(?i)(const|let|var|val|String|final\s+String)?\s*([a-zA-Z0-9_]+)\s*[:=]\s*["']([^"'\r\n]{6,})["']"#).unwrap();
}

// Rule Registry

pub fn get_high_confidence_rules() -> Vec<PatternRule> {
    vec![
        PatternRule {
            name: "OpenAI API Key",
            regex: &RE_OPENAI,
            secret_type: SecretType::OpenAiApiKey,
            confidence: Confidence::High,
            default_var_name: "OPENAI_API_KEY",
        },
        PatternRule {
            name: "AWS Access Key",
            regex: &RE_AWS_KEY,
            secret_type: SecretType::AwsAccessKey,
            confidence: Confidence::High,
            default_var_name: "AWS_ACCESS_KEY_ID",
        },
        PatternRule {
            name: "GitHub Access Token",
            regex: &RE_GITHUB_PAT,
            secret_type: SecretType::GitHubToken,
            confidence: Confidence::High,
            default_var_name: "GITHUB_TOKEN",
        },
        PatternRule {
            name: "Slack Token",
            regex: &RE_SLACK_TOKEN,
            secret_type: SecretType::SlackToken,
            confidence: Confidence::High,
            default_var_name: "SLACK_TOKEN",
        },
        PatternRule {
            name: "Stripe API Key",
            regex: &RE_STRIPE_KEY,
            secret_type: SecretType::StripeKey,
            confidence: Confidence::High,
            default_var_name: "STRIPE_API_KEY",
        },
        PatternRule {
            name: "Database Connection URL",
            regex: &RE_DB_URL,
            secret_type: SecretType::DatabaseUrl,
            confidence: Confidence::High,
            default_var_name: "DATABASE_URL",
        },
        PatternRule {
            name: "Private Key",
            regex: &RE_PRIVATE_KEY,
            secret_type: SecretType::PrivateKey,
            confidence: Confidence::High,
            default_var_name: "PRIVATE_KEY",
        },
    ]
}

// Assignment Matcher

pub fn match_assignments(line: &str) -> Vec<(String, String, usize, usize)> {
    let mut results = Vec::new();
    for cap in RE_ASSIGNMENT.captures_iter(line) {
        if let (Some(name_match), Some(val_match)) = (cap.get(2), cap.get(3)) {
            let var_name = name_match.as_str().to_string();
            let value = val_match.as_str().to_string();
            let start = val_match.start();
            let end = val_match.end();
            results.push((var_name, value, start, end));
        }
    }
    results
}
