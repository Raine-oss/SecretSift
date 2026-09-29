use crate::detector::detect_secrets_in_content;
use crate::models::{Confidence, Language, SecretMatch};
use ignore::WalkBuilder;
use std::fs;
use std::path::Path;

// File Scanner

pub struct ProjectScanner {
    min_confidence: Confidence,
}

impl ProjectScanner {
    pub fn new(min_confidence: Confidence) -> Self {
        Self { min_confidence }
    }

    pub fn scan_path(&self, root_path: &Path) -> Result<Vec<SecretMatch>, std::io::Error> {
        let mut all_matches = Vec::new();

        let walker = WalkBuilder::new(root_path)
            .hidden(true)
            .parents(true)
            .git_ignore(true)
            .git_global(true)
            .git_exclude(true)
            .filter_entry(|entry| {
                let file_name = entry.file_name().to_string_lossy();
                if file_name == ".git"
                    || file_name == "node_modules"
                    || file_name == "target"
                    || file_name == ".secretsift-backup"
                    || file_name == ".venv"
                    || file_name == "venv"
                    || file_name == "__pycache__"
                    || file_name == "dist"
                    || file_name == "build"
                {
                    return false;
                }
                true
            })
            .build();

        for result in walker {
            let entry = match result {
                Ok(e) => e,
                Err(_) => continue,
            };

            let path = entry.path();
            if !path.is_file() {
                continue;
            }

            let ext = path
                .extension()
                .and_then(|s| s.to_str())
                .unwrap_or("")
                .to_lowercase();

            let language = Language::from_extension(&ext);

            let content = match fs::read_to_string(path) {
                Ok(c) => c,
                Err(_) => continue,
            };

            let matches = detect_secrets_in_content(path, &content, language);
            for m in matches {
                if m.confidence >= self.min_confidence {
                    all_matches.push(m);
                }
            }
        }

        all_matches.sort_by(|a, b| {
            b.confidence
                .cmp(&a.confidence)
                .then_with(|| a.file_path.cmp(&b.file_path))
                .then_with(|| a.line_number.cmp(&b.line_number))
        });

        Ok(all_matches)
    }
}
