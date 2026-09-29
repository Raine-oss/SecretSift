use crate::models::{Confidence, SecretMatch};
use colored::Colorize;
use std::path::Path;

// Terminal Reporter

pub fn print_terminal_report(root_path: &Path, matches: &[SecretMatch]) {
    if matches.is_empty() {
        println!(
            "{}",
            "No potential secrets found. Your repository appears clean!"
                .green()
                .bold()
        );
        return;
    }

    println!("\n{}", "Potential secrets found".bold().underline());
    println!();

    for m in matches {
        let conf_badge = match m.confidence {
            Confidence::High => format!("{:<8}", "HIGH".red().bold()),
            Confidence::Medium => format!("{:<8}", "MEDIUM".yellow().bold()),
            Confidence::Low => format!("{:<8}", "LOW".cyan()),
        };

        let relative_path = m
            .file_path
            .strip_prefix(root_path)
            .unwrap_or(&m.file_path)
            .display();

        let location = format!("{}:{}", relative_path, m.line_number);

        println!("{} {}", conf_badge, location.white().bold());
        println!("         {}", m.suggested_var_name.cyan());
        println!("         {}", format!("{}", m.secret_type).dimmed());
        println!();
    }

    let count_text = if matches.len() == 1 {
        "1 potential secret found".yellow().bold()
    } else {
        format!("{} potential secrets found", matches.len())
            .yellow()
            .bold()
    };

    println!("{}", count_text);
    println!();
}
