use crate::models::FixPlan;
use colored::Colorize;
use similar::{ChangeTag, TextDiff};
use std::path::Path;

// Diff Preview

pub fn display_fix_plan_diff(root_path: &Path, plan: &FixPlan) {
    println!("\n{}", "Proposed changes".bold().underline());
    println!();

    for file_change in &plan.file_changes {
        let relative_path = file_change
            .file_path
            .strip_prefix(root_path)
            .unwrap_or(&file_change.file_path)
            .display();

        println!("{}", relative_path.to_string().white().bold());
        println!(
            "{}",
            "────────────────────────────────────────────────".dimmed()
        );

        let diff = TextDiff::from_lines(&file_change.original_content, &file_change.new_content);

        for change in diff.iter_all_changes() {
            let sign = match change.tag() {
                ChangeTag::Delete => "- ".red(),
                ChangeTag::Insert => "+ ".green(),
                ChangeTag::Equal => "  ".dimmed(),
            };
            print!("{}{}", sign, change);
        }
        println!();
    }

    if !plan.env_entries.is_empty() {
        println!("{}", ".env".white().bold());
        println!(
            "{}",
            "────────────────────────────────────────────────".dimmed()
        );
        for (k, v) in &plan.env_entries {
            println!("{}", format!("+ {}={}", k, v).green());
        }
        println!();
    }

    if !plan.env_example_entries.is_empty() {
        println!("{}", ".env.example".white().bold());
        println!(
            "{}",
            "────────────────────────────────────────────────".dimmed()
        );
        for k in &plan.env_example_entries {
            println!("{}", format!("+ {}=", k).green());
        }
        println!();
    }

    if plan.gitignore_updated {
        println!("{}", ".gitignore".white().bold());
        println!(
            "{}",
            "────────────────────────────────────────────────".dimmed()
        );
        println!("{}", "+ .env".green());
        println!();
    }
}
