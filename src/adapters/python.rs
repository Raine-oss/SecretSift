use super::LanguageAdapter;
use crate::models::{Language, SecretMatch};
use regex::Regex;

// Python Adapter

pub struct PythonAdapter;

impl PythonAdapter {
    pub fn new() -> Self {
        Self
    }
}

impl Default for PythonAdapter {
    fn default() -> Self {
        Self::new()
    }
}

impl LanguageAdapter for PythonAdapter {
    fn language(&self) -> Language {
        Language::Python
    }

    fn rewrite_line(&self, match_info: &SecretMatch, line: &str) -> Option<String> {
        let secret = &match_info.matched_value;
        let var_name = &match_info.suggested_var_name;

        let quoted_double = format!("\"{}\"", secret);
        let quoted_single = format!("'{}'", secret);

        let replacement = format!("os.getenv(\"{}\")", var_name);

        if line.contains(&quoted_double) {
            Some(line.replace(&quoted_double, &replacement))
        } else if line.contains(&quoted_single) {
            Some(line.replace(&quoted_single, &replacement))
        } else {
            let escaped_secret = regex::escape(secret);
            let re_pattern = format!(r#"["'](?:[^"']*{}[^"']*)["']"#, escaped_secret);
            if let Ok(re) = Regex::new(&re_pattern) {
                if re.is_match(line) {
                    return Some(re.replace(line, replacement.as_str()).to_string());
                }
            }
            Some(line.replace(secret, &replacement))
        }
    }

    fn post_process_file_content(&self, content: &str) -> String {
        let has_os_import = content.lines().any(|l| {
            let trimmed = l.trim();
            trimmed == "import os"
                || trimmed.starts_with("import os,")
                || trimmed.starts_with("import os ")
                || trimmed.starts_with("from os import")
        });

        if has_os_import || !content.contains("os.getenv(") {
            return content.to_string();
        }

        let lines: Vec<&str> = content.lines().collect();
        let mut first_import_idx = None;
        let mut insert_idx = 0;

        for (i, line) in lines.iter().enumerate() {
            let trimmed = line.trim();
            if trimmed.starts_with("#!")
                || trimmed.starts_with("# -*-")
                || trimmed.starts_with("# coding")
            {
                insert_idx = i + 1;
                continue;
            }
            if trimmed.starts_with("import ") || trimmed.starts_with("from ") {
                first_import_idx = Some(i);
                break;
            }
        }

        if let Some(idx) = first_import_idx {
            let mut new_lines = Vec::new();
            for (i, line) in lines.iter().enumerate() {
                if i == idx {
                    new_lines.push("import os");
                }
                new_lines.push(*line);
            }
            let mut result = new_lines.join("\n");
            if content.ends_with('\n') {
                result.push('\n');
            }
            result
        } else {
            let mut new_lines = Vec::new();
            let mut inserted = false;
            for (i, line) in lines.iter().enumerate() {
                if i == insert_idx && !inserted {
                    new_lines.push("import os");
                    if !line.is_empty() {
                        new_lines.push("");
                    }
                    inserted = true;
                }
                new_lines.push(*line);
            }
            if !inserted {
                new_lines.insert(0, "import os\n");
            }
            let mut result = new_lines.join("\n");
            if content.ends_with('\n') {
                result.push('\n');
            }
            result
        }
    }
}
