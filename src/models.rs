use serde::{Deserialize, Serialize};
use std::path::PathBuf;

// Confidence

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub enum Confidence {
    Low = 1,
    Medium = 2,
    High = 3,
}

impl std::fmt::Display for Confidence {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Confidence::High => write!(f, "HIGH"),
            Confidence::Medium => write!(f, "MEDIUM"),
            Confidence::Low => write!(f, "LOW"),
        }
    }
}

// Secret Type

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum SecretType {
    ApiKey,
    DatabaseUrl,
    OAuthToken,
    PrivateKey,
    AwsAccessKey,
    OpenAiApiKey,
    GitHubToken,
    SlackToken,
    StripeKey,
    GenericSecret,
}

impl std::fmt::Display for SecretType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            SecretType::ApiKey => write!(f, "API key pattern"),
            SecretType::DatabaseUrl => write!(f, "Database connection string"),
            SecretType::OAuthToken => write!(f, "OAuth token"),
            SecretType::PrivateKey => write!(f, "Private cryptographic key"),
            SecretType::AwsAccessKey => write!(f, "AWS Access Key"),
            SecretType::OpenAiApiKey => write!(f, "OpenAI API key"),
            SecretType::GitHubToken => write!(f, "GitHub Access Token"),
            SecretType::SlackToken => write!(f, "Slack Token"),
            SecretType::StripeKey => write!(f, "Stripe API Key"),
            SecretType::GenericSecret => write!(f, "Possible secret / token"),
        }
    }
}

// Supported Language

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum Language {
    Rust,
    Java,
    JavaScript,
    TypeScript,
    Python,
    Go,
    CSharp,
    Kotlin,
    Php,
    Ruby,
    C,
    Cpp,
    Dart,
    Swift,
    Unknown,
}

impl Language {
    pub fn from_extension(ext: &str) -> Self {
        match ext {
            "rs" => Language::Rust,
            "java" => Language::Java,
            "js" | "jsx" | "mjs" | "cjs" => Language::JavaScript,
            "ts" | "tsx" | "mts" | "cts" => Language::TypeScript,
            "py" => Language::Python,
            "go" => Language::Go,
            "cs" => Language::CSharp,
            "kt" | "kts" => Language::Kotlin,
            "php" => Language::Php,
            "rb" => Language::Ruby,
            "c" | "h" => Language::C,
            "cpp" | "cc" | "cxx" | "hpp" => Language::Cpp,
            "dart" => Language::Dart,
            "swift" => Language::Swift,
            _ => Language::Unknown,
        }
    }
}

// Secret Match

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SecretMatch {
    pub file_path: PathBuf,
    pub line_number: usize,
    pub column: usize,
    pub matched_value: String,
    pub raw_line: String,
    pub suggested_var_name: String,
    pub secret_type: SecretType,
    pub confidence: Confidence,
    pub language: Language,
}

// Proposed File Change

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FileChange {
    pub file_path: PathBuf,
    pub original_content: String,
    pub new_content: String,
    pub replacements: Vec<LineReplacement>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LineReplacement {
    pub line_number: usize,
    pub original_line: String,
    pub new_line: String,
    pub env_var_name: String,
    pub secret_value: String,
}

// Fix Plan

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FixPlan {
    pub file_changes: Vec<FileChange>,
    pub env_entries: Vec<(String, String)>,
    pub env_example_entries: Vec<String>,
    pub gitignore_updated: bool,
}
