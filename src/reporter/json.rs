use crate::models::SecretMatch;
use serde_json::to_string_pretty;

// JSON Reporter

pub fn print_json_report(matches: &[SecretMatch]) {
    match to_string_pretty(matches) {
        Ok(json) => println!("{}", json),
        Err(err) => eprintln!("Failed to serialize JSON report: {}", err),
    }
}
