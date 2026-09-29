use super::LanguageAdapter;
use crate::models::{Language, SecretMatch};

// C# Adapter

pub struct CSharpAdapter;

impl CSharpAdapter {
    pub fn new() -> Self {
        Self
    }
}

impl Default for CSharpAdapter {
    fn default() -> Self {
        Self::new()
    }
}

impl LanguageAdapter for CSharpAdapter {
    fn language(&self) -> Language {
        Language::CSharp
    }

    fn rewrite_line(&self, match_info: &SecretMatch, line: &str) -> Option<String> {
        let secret = &match_info.matched_value;
        let var_name = &match_info.suggested_var_name;

        let quoted_double = format!("\"{}\"", secret);
        let replacement = format!(
            "Environment.GetEnvironmentVariable(\"{}\") ?? \"\"",
            var_name
        );

        if line.contains(&quoted_double) {
            Some(line.replace(&quoted_double, &replacement))
        } else if line.contains(secret) {
            Some(line.replace(secret, &replacement))
        } else {
            None
        }
    }

    fn post_process_file_content(&self, content: &str) -> String {
        let has_using_system = content.lines().any(|l| {
            let trimmed = l.trim();
            trimmed == "using System;" || trimmed.starts_with("using System;")
        });

        if has_using_system || !content.contains("Environment.GetEnvironmentVariable(") {
            return content.to_string();
        }

        let lines: Vec<&str> = content.lines().collect();
        let mut new_lines = vec!["using System;"];
        new_lines.extend_from_slice(&lines);

        let mut result = new_lines.join("\n");
        if content.ends_with('\n') {
            result.push('\n');
        }
        result
    }
}
