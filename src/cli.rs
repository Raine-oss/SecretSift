use clap::{Args, Parser, Subcommand, ValueEnum};
use std::path::PathBuf;

// CLI Arguments

#[derive(Parser, Debug)]
#[command(
    name = "secretsift",
    version,
    about = "Turn a private codebase into a public-safe repository."
)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Option<Commands>,

    #[arg(value_name = "PATH", global = false)]
    pub default_path: Option<PathBuf>,
}

// Commands

#[derive(Subcommand, Debug)]
pub enum Commands {
    Scan(ScanArgs),
    Fix(FixArgs),
    Check(CheckArgs),
}

// Confidence Filter

#[derive(ValueEnum, Clone, Copy, Debug, PartialEq, Eq)]
pub enum CliConfidence {
    Low,
    Medium,
    High,
}

impl From<CliConfidence> for crate::models::Confidence {
    fn from(c: CliConfidence) -> Self {
        match c {
            CliConfidence::Low => crate::models::Confidence::Low,
            CliConfidence::Medium => crate::models::Confidence::Medium,
            CliConfidence::High => crate::models::Confidence::High,
        }
    }
}

// Scan Arguments

#[derive(Args, Debug)]
pub struct ScanArgs {
    #[arg(value_name = "PATH", default_value = ".")]
    pub path: PathBuf,

    #[arg(long)]
    pub json: bool,

    #[arg(long, value_enum, default_value = "low")]
    pub min_confidence: CliConfidence,
}

// Fix Arguments

#[derive(Args, Debug)]
pub struct FixArgs {
    #[arg(value_name = "PATH", default_value = ".")]
    pub path: PathBuf,

    #[arg(long)]
    pub dry_run: bool,

    #[arg(long, short = 'y')]
    pub yes: bool,

    #[arg(long, value_enum, default_value = "high")]
    pub min_confidence: CliConfidence,
}

// Check Arguments

#[derive(Args, Debug)]
pub struct CheckArgs {
    #[arg(value_name = "PATH", default_value = ".")]
    pub path: PathBuf,

    #[arg(long, value_enum, default_value = "high")]
    pub min_confidence: CliConfidence,
}
