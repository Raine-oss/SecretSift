mod adapters;
mod cli;
mod detector;
mod fix;
mod models;
mod reporter;
mod safety;
mod scanner;

use clap::Parser;
use cli::{CheckArgs, Cli, Commands, FixArgs, ScanArgs};
use colored::Colorize;
use fix::build_fix_plan;
use fix::diff::display_fix_plan_diff;
use models::Confidence;
use reporter::{print_json_report, print_terminal_report};
use safety::apply_fix_plan_atomically;
use scanner::ProjectScanner;
use std::io::{self, Write};
use std::path::PathBuf;

// Command Handlers

fn handle_scan(args: ScanArgs) {
    let target_path = args.path.canonicalize().unwrap_or(args.path);
    let min_conf: Confidence = args.min_confidence.into();
    let scanner = ProjectScanner::new(min_conf);

    match scanner.scan_path(&target_path) {
        Ok(matches) => {
            if args.json {
                print_json_report(&matches);
            } else {
                print_terminal_report(&target_path, &matches);
            }
        }
        Err(err) => {
            eprintln!("Error scanning directory: {}", err);
            std::process::exit(1);
        }
    }
}

fn handle_fix(args: FixArgs) {
    let target_path = args.path.canonicalize().unwrap_or(args.path);
    let min_conf: Confidence = args.min_confidence.into();
    let scanner = ProjectScanner::new(min_conf);

    let matches = match scanner.scan_path(&target_path) {
        Ok(m) => m,
        Err(err) => {
            eprintln!("Error scanning directory: {}", err);
            std::process::exit(1);
        }
    };

    if matches.is_empty() {
        println!(
            "{}",
            "No potential secrets found matching the confidence threshold to fix."
                .green()
                .bold()
        );
        return;
    }

    let plan = match build_fix_plan(&target_path, &matches) {
        Ok(p) => p,
        Err(err) => {
            eprintln!("Failed to construct fix plan: {}", err);
            std::process::exit(1);
        }
    };

    if plan.file_changes.is_empty() && plan.env_entries.is_empty() {
        println!("{}", "No actionable code transformations found.".yellow());
        return;
    }

    display_fix_plan_diff(&target_path, &plan);

    if args.dry_run {
        println!(
            "{}",
            "(Dry-run mode enabled: no changes were written to disk)"
                .yellow()
                .italic()
        );
        return;
    }

    if !args.yes {
        print!("{} ", "Apply changes? [y/N]".bold());
        let _ = io::stdout().flush();

        let mut input = String::new();
        if io::stdin().read_line(&mut input).is_err() {
            println!("{}", "Failed to read user input. Aborting.".red());
            std::process::exit(1);
        }

        let trimmed = input.trim().to_lowercase();
        if trimmed != "y" && trimmed != "yes" {
            println!("{}", "Aborted. No changes were made.".yellow());
            return;
        }
    }

    match apply_fix_plan_atomically(&target_path, &plan) {
        Ok(backup_dir) => {
            let relative_backup = backup_dir
                .strip_prefix(&target_path)
                .unwrap_or(&backup_dir)
                .display();

            println!();
            println!("{}", "✔ Changes successfully applied!".green().bold());
            println!("  Backup stored at: {}", relative_backup.to_string().cyan());
            println!("  Updated/Created : .env, .env.example, .gitignore");
            println!("  Refactored files: {}", plan.file_changes.len());
        }
        Err(err) => {
            eprintln!(
                "\n{}",
                format!("Failed to apply changes: {}", err).red().bold()
            );
            std::process::exit(1);
        }
    }
}

fn handle_check(args: CheckArgs) {
    let target_path = args.path.canonicalize().unwrap_or(args.path);
    let min_conf: Confidence = args.min_confidence.into();
    let scanner = ProjectScanner::new(min_conf);

    println!("\n{}", "SecretSift Check".bold().underline());
    println!();

    match scanner.scan_path(&target_path) {
        Ok(matches) => {
            if matches.is_empty() {
                println!(
                    "{}",
                    "✔ No secrets detected matching threshold. Repository is clean."
                        .green()
                        .bold()
                );
                std::process::exit(0);
            } else {
                let count_str = if matches.len() == 1 {
                    format!("✗ 1 {} secret detected", min_conf)
                } else {
                    format!("✗ {} {} secrets detected", matches.len(), min_conf)
                };
                println!("{}", count_str.red().bold());
                println!();

                for m in &matches {
                    let relative_path = m
                        .file_path
                        .strip_prefix(&target_path)
                        .unwrap_or(&m.file_path)
                        .display();

                    let loc = format!("{}:{}", relative_path, m.line_number);
                    println!("{:<28} {}", loc.white().bold(), m.suggested_var_name.cyan());
                }

                println!();
                std::process::exit(1);
            }
        }
        Err(err) => {
            eprintln!("Error running check: {}", err);
            std::process::exit(1);
        }
    }
}

// Main Entrypoint

fn main() {
    let args = Cli::parse();

    match args.command {
        Some(Commands::Scan(scan_args)) => handle_scan(scan_args),
        Some(Commands::Fix(fix_args)) => handle_fix(fix_args),
        Some(Commands::Check(check_args)) => handle_check(check_args),
        None => {
            let target_path = args.default_path.unwrap_or_else(|| PathBuf::from("."));
            let scan_args = ScanArgs {
                path: target_path,
                json: false,
                min_confidence: cli::CliConfidence::Low,
            };
            handle_scan(scan_args);
        }
    }
}
