use super::LanguageAdapter;
use crate::models::{Language, SecretMatch};

// C and C++ Adapter

pub struct CppAdapter {
    language: Language,
}

impl CppAdapter {
    pub fn new(language: Language) -> Self {
        Self { language }
    }
}

impl LanguageAdapter for CppAdapter {
    fn language(&self) -> Language {
        self.language
    }

    fn rewrite_line(&self, match_info: &SecretMatch, line: &str) -> Option<String> {
        let secret = &match_info.matched_value;
        let var_name = &match_info.suggested_var_name;

        let quoted_double = format!("\"{}\"", secret);
        let replacement = format!("getenv(\"{}\")", var_name);

        if line.contains(&quoted_double) {
            Some(line.replace(&quoted_double, &replacement))
        } else if line.contains(secret) {
            Some(line.replace(secret, &replacement))
        } else {
            None
        }
    }

    fn post_process_file_content(&self, content: &str) -> String {
        let has_stdlib = content.lines().any(|l| {
            let trimmed = l.trim();
            trimmed.contains("<stdlib.h>") || trimmed.contains("<cstdlib>")
        });

        if has_stdlib || !content.contains("getenv(") {
            return content.to_string();
        }

        let include_stmt = if self.language == Language::Cpp {
            "#include <cstdlib>"
        } else {
            "#include <stdlib.h>"
        };

        let lines: Vec<&str> = content.lines().collect();
        let mut new_lines = vec![include_stmt];
        new_lines.extend_from_slice(&lines);

        let mut result = new_lines.join("\n");
        if content.ends_with('\n') {
            result.push('\n');
        }
        result
    }
}
