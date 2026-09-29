use std::collections::{HashMap, HashSet};
use std::fs;
use std::path::Path;

// Environment Manager

pub struct EnvFileResult {
    pub env_content: String,
    pub example_content: String,
}

pub fn generate_env_files(
    root_path: &Path,
    extracted_entries: &[(String, String)],
) -> EnvFileResult {
    let env_path = root_path.join(".env");
    let example_path = root_path.join(".env.example");

    let existing_env = fs::read_to_string(&env_path).unwrap_or_default();
    let existing_example = fs::read_to_string(&example_path).unwrap_or_default();

    let mut env_map: HashMap<String, String> = HashMap::new();
    let mut env_order: Vec<String> = Vec::new();

    for line in existing_env.lines() {
        let trimmed = line.trim();
        if trimmed.is_empty() || trimmed.starts_with('#') {
            continue;
        }
        if let Some((k, v)) = trimmed.split_once('=') {
            let key = k.trim().to_string();
            let val = v.trim().to_string();
            if !env_map.contains_key(&key) {
                env_order.push(key.clone());
            }
            env_map.insert(key, val);
        }
    }

    let mut example_keys: HashSet<String> = HashSet::new();
    let mut example_order: Vec<String> = Vec::new();

    for line in existing_example.lines() {
        let trimmed = line.trim();
        if trimmed.is_empty() || trimmed.starts_with('#') {
            continue;
        }
        let key = if let Some((k, _)) = trimmed.split_once('=') {
            k.trim().to_string()
        } else {
            trimmed.to_string()
        };
        if !example_keys.contains(&key) {
            example_order.push(key.clone());
            example_keys.insert(key);
        }
    }

    for (k, v) in extracted_entries {
        if !env_map.contains_key(k) {
            env_order.push(k.clone());
            env_map.insert(k.clone(), v.clone());
        }
        if !example_keys.contains(k) {
            example_order.push(k.clone());
            example_keys.insert(k.clone());
        }
    }

    let mut final_env = String::new();
    for key in &env_order {
        if let Some(val) = env_map.get(key) {
            final_env.push_str(&format!("{}={}\n", key, val));
        }
    }

    let mut final_example = String::new();
    for key in &example_order {
        final_example.push_str(&format!("{}=\n", key));
    }

    EnvFileResult {
        env_content: final_env,
        example_content: final_example,
    }
}
