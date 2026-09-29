use super::LanguageAdapter;
use crate::models::{Language, SecretMatch};

// Dart Adapter

pub struct DartAdapter;

impl DartAdapter {
    pub fn new() -> Self {
        Self
    }
}

impl Default for DartAdapter {
    fn default() -> Self {
        Self::new()
    }
}

impl LanguageAdapter for DartAdapter {
    fn language(&self) -> Language {
        Language::Dart
    }

    fn rewrite_line(&self, match_info: &SecretMatch, line: &str) -> Option<String> {
        let secret = &match_info.matched_value;
        let var_name = &match_info.suggested_var_name;

        let quoted_double = format!("\"{}\"", secret);
        let quoted_single = format!("'{}'", secret);

        let replacement = format!("Platform.environment['{}'] ?? ''", var_name);

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

    fn post_process_file_content(&self, content: &str) -> String {
        let has_dart_io = content.lines().any(|l| {
            let trimmed = l.trim();
            trimmed == "import 'dart:io';" || trimmed == "import \"dart:io\";"
        });

        if has_dart_io || !content.contains("Platform.environment[") {
            return content.to_string();
        }

        let lines: Vec<&str> = content.lines().collect();
        let mut new_lines = vec!["import 'dart:io';"];
        new_lines.extend_from_slice(&lines);

        let mut result = new_lines.join("\n");
        if content.ends_with('\n') {
            result.push('\n');
        }
        result
    }
}
