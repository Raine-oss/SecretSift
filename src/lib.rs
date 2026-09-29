pub mod adapters;
pub mod cli;
pub mod detector;
pub mod fix;
pub mod models;
pub mod reporter;
pub mod safety;
pub mod scanner;

// Library Re-exports

pub use adapters::get_adapter_for_language;
pub use detector::detect_secrets_in_content;
pub use fix::build_fix_plan;
pub use models::{Confidence, Language, SecretMatch, SecretType};
pub use safety::apply_fix_plan_atomically;
pub use scanner::ProjectScanner;
