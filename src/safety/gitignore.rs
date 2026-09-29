use std::fs::{self, OpenOptions};
use std::io::Write;
use std::path::Path;

// Gitignore Manager

pub fn ensure_env_in_gitignore(root_path: &Path) -> Result<bool, std::io::Error> {
    let gitignore_path = root_path.join(".gitignore");

    if gitignore_path.exists() {
        let content = fs::read_to_string(&gitignore_path)?;
        let has_env = content
            .lines()
            .any(|l| l.trim() == ".env" || l.trim() == "/.env" || l.trim() == "*.env");

        if has_env {
            return Ok(false);
        }

        let mut file = OpenOptions::new().append(true).open(&gitignore_path)?;
        let prefix = if content.ends_with('\n') || content.is_empty() {
            ""
        } else {
            "\n"
        };
        writeln!(file, "{}.env", prefix)?;
        Ok(true)
    } else {
        fs::write(&gitignore_path, ".env\n")?;
        Ok(true)
    }
}
