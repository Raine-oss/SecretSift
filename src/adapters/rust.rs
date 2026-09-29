use super::LanguageAdapter;
use crate::models::{Language, SecretMatch};

// Rust Adapter

pub struct RustAdapter;

impl RustAdapter {
    pub fn new() -> Self {
        Self
    }
}

impl Default for RustAdapter {
    fn default() -> Self {
        Self::new()
    }
}

impl LanguageAdapter for RustAdapter {
    fn language(&self) -> Language {
        Language::Rust
    }

    fn rewrite_line(&self, match_info: &SecretMatch, line: &str) -> Option<String> {
        let secret = &match_info.matched_value;
        let var_name = &match_info.suggested_var_name;

        let quoted_double = format!("\"{}\"", secret);
        let quoted_single = format!("'{}'", secret);

        if line.contains(&quoted_double) {
            let is_fn_arg = line.contains(&format!("(\"{}\")", secret))
                || line.contains(&format!("(\"{}\",", secret))
                || line.contains(&format!(", \"{}\"", secret))
                || line.contains(&format!(", \"{}\",", secret));

            let replacement = if is_fn_arg {
                format!("&std::env::var(\"{}\").unwrap_or_default()", var_name)
            } else {
                format!("std::env::var(\"{}\").unwrap_or_default()", var_name)
            };

            Some(line.replace(&quoted_double, &replacement))
        } else if line.contains(&quoted_single) {
            Some(line.replace(
                &quoted_single,
                &format!("std::env::var(\"{}\").unwrap_or_default()", var_name),
            ))
        } else if line.contains(secret) {
            if line.contains("format!(\"") {
                let pattern_to_replace = format!("{}\");", secret);
                if line.contains(&pattern_to_replace) {
                    let target = format!(
                        "{}\", std::env::var(\"{}\").unwrap_or_default());",
                        "{}", var_name
                    );
                    Some(line.replace(&pattern_to_replace, &target))
                } else {
                    let pattern_inside = format!("{}\"", secret);
                    let target = format!(
                        "{}\", std::env::var(\"{}\").unwrap_or_default()",
                        "{}", var_name
                    );
                    Some(line.replace(&pattern_inside, &target))
                }
            } else {
                Some(line.replace(
                    secret,
                    &format!("std::env::var(\"{}\").unwrap_or_default()", var_name),
                ))
            }
        } else {
            None
        }
    }
}
