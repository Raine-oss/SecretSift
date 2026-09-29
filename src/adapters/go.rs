use super::LanguageAdapter;
use crate::models::{Language, SecretMatch};

// Go Adapter

pub struct GoAdapter;

impl GoAdapter {
    pub fn new() -> Self {
        Self
    }
}

impl Default for GoAdapter {
    fn default() -> Self {
        Self::new()
    }
}

impl LanguageAdapter for GoAdapter {
    fn language(&self) -> Language {
        Language::Go
    }

    fn rewrite_line(&self, match_info: &SecretMatch, line: &str) -> Option<String> {
        let secret = &match_info.matched_value;
        let var_name = &match_info.suggested_var_name;

        let quoted_double = format!("\"{}\"", secret);
        let quoted_backtick = format!("`{}`", secret);
        let replacement = format!("os.Getenv(\"{}\")", var_name);

        let rewritten = if line.contains(&quoted_double) {
            line.replace(&quoted_double, &replacement)
        } else if line.contains(&quoted_backtick) {
            line.replace(&quoted_backtick, &replacement)
        } else if line.contains(secret) {
            line.replace(secret, &replacement)
        } else {
            return None;
        };

        let trimmed = rewritten.trim_start();
        if trimmed.starts_with("const ") {
            let indent = &rewritten[..rewritten.len() - trimmed.len()];
            let without_const = trimmed.trim_start_matches("const ");
            Some(format!("{}var {}", indent, without_const))
        } else {
            Some(rewritten)
        }
    }

    fn post_process_file_content(&self, content: &str) -> String {
        let has_os_import = content.lines().any(|l| {
            let trimmed = l.trim();
            trimmed == "\"os\""
                || trimmed == "import \"os\""
                || trimmed.starts_with("import \"os\"")
        });

        if has_os_import || !content.contains("os.Getenv(") {
            return content.to_string();
        }

        let lines: Vec<&str> = content.lines().collect();

        let mut in_import_block = false;
        let mut import_block_start = None;
        let mut single_import_idx = None;
        let mut package_line_idx = None;

        for (i, line) in lines.iter().enumerate() {
            let trimmed = line.trim();
            if trimmed.starts_with("package ") {
                package_line_idx = Some(i);
            }
            if trimmed.starts_with("import (") {
                in_import_block = true;
                import_block_start = Some(i);
                break;
            }
            if trimmed.starts_with("import ") && !trimmed.starts_with("import (") {
                single_import_idx = Some(i);
                break;
            }
        }

        if in_import_block {
            if let Some(idx) = import_block_start {
                let mut new_lines = Vec::new();
                for (i, line) in lines.iter().enumerate() {
                    new_lines.push(*line);
                    if i == idx {
                        new_lines.push("\t\"os\"");
                    }
                }
                let mut result = new_lines.join("\n");
                if content.ends_with('\n') {
                    result.push('\n');
                }
                return result;
            }
        }

        if let Some(idx) = single_import_idx {
            let mut new_lines = Vec::new();
            for (i, line) in lines.iter().enumerate() {
                if i == idx {
                    new_lines.push("import (");
                    new_lines.push("\t\"os\"");
                    let existing_import = line.trim_start_matches("import ").trim();
                    new_lines.push(existing_import);
                    new_lines.push(")");
                } else {
                    new_lines.push(*line);
                }
            }
            let mut result = new_lines.join("\n");
            if content.ends_with('\n') {
                result.push('\n');
            }
            return result;
        }

        if let Some(pkg_idx) = package_line_idx {
            let mut new_lines = Vec::new();
            for (i, line) in lines.iter().enumerate() {
                new_lines.push(*line);
                if i == pkg_idx {
                    new_lines.push("");
                    new_lines.push("import \"os\"");
                }
            }
            let mut result = new_lines.join("\n");
            if content.ends_with('\n') {
                result.push('\n');
            }
            return result;
        }

        let mut new_lines = vec!["import \"os\"", ""];
        new_lines.extend_from_slice(&lines);
        let mut result = new_lines.join("\n");
        if content.ends_with('\n') {
            result.push('\n');
        }
        result
    }
}
