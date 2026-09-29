use super::LanguageAdapter;
use crate::models::{Language, SecretMatch};

// Kotlin Adapter

pub struct KotlinAdapter;

impl KotlinAdapter {
    pub fn new() -> Self {
        Self
    }
}

impl Default for KotlinAdapter {
    fn default() -> Self {
        Self::new()
    }
}

impl LanguageAdapter for KotlinAdapter {
    fn language(&self) -> Language {
        Language::Kotlin
    }

    fn rewrite_line(&self, match_info: &SecretMatch, line: &str) -> Option<String> {
        let secret = &match_info.matched_value;
        let var_name = &match_info.suggested_var_name;

        let quoted_double = format!("\"{}\"", secret);
        let replacement = format!("System.getenv(\"{}\") ?: \"\"", var_name);

        if line.contains(&quoted_double) {
            Some(line.replace(&quoted_double, &replacement))
        } else if line.contains(secret) {
            Some(line.replace(secret, &replacement))
        } else {
            None
        }
    }
}
