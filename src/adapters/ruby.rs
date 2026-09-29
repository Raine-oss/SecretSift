use super::LanguageAdapter;
use crate::models::{Language, SecretMatch};

// Ruby Adapter

pub struct RubyAdapter;

impl RubyAdapter {
    pub fn new() -> Self {
        Self
    }
}

impl Default for RubyAdapter {
    fn default() -> Self {
        Self::new()
    }
}

impl LanguageAdapter for RubyAdapter {
    fn language(&self) -> Language {
        Language::Ruby
    }

    fn rewrite_line(&self, match_info: &SecretMatch, line: &str) -> Option<String> {
        let secret = &match_info.matched_value;
        let var_name = &match_info.suggested_var_name;

        let quoted_double = format!("\"{}\"", secret);
        let quoted_single = format!("'{}'", secret);

        let replacement = format!("ENV['{}'] || ''", var_name);

        if line.contains(&quoted_double) {
            Some(line.replace(&quoted_double, &replacement))
        } else if line.contains(&quoted_single) {
            Some(line.replace(&quoted_single, &replacement))
        } else if line.contains(secret) {
            Some(line.replace(secret, &replacement))
        } else {
            None
        }
    }
}
