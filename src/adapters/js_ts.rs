use super::LanguageAdapter;
use crate::models::{Language, SecretMatch};

// JavaScript / TypeScript Adapter

pub struct JsTsAdapter {
    #[allow(dead_code)]
    language: Language,
}

impl JsTsAdapter {
    pub fn new(language: Language) -> Self {
        Self { language }
    }
}

impl LanguageAdapter for JsTsAdapter {
    fn language(&self) -> Language {
        self.language
    }

    fn rewrite_line(&self, match_info: &SecretMatch, line: &str) -> Option<String> {
        let secret = &match_info.matched_value;
        let var_name = &match_info.suggested_var_name;

        let quoted_double = format!("\"{}\"", secret);
        let quoted_single = format!("'{}'", secret);
        let quoted_backtick = format!("`{}`", secret);

        let replacement = format!("process.env.{} || \"\"", var_name);

        if line.contains(&quoted_double) {
            Some(line.replace(&quoted_double, &replacement))
        } else if line.contains(&quoted_single) {
            Some(line.replace(&quoted_single, &replacement))
        } else if line.contains(&quoted_backtick) {
            Some(line.replace(&quoted_backtick, &replacement))
        } else if line.contains(secret) {
            Some(line.replace(secret, &replacement))
        } else {
            None
        }
    }
}
