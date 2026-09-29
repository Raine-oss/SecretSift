mod config;

// Rust Fixture Main

fn main() {
    println!("Starting Rust Fixture Application");
    let cfg = config::load_config();
    println!("Connected to database at host: {}", cfg.app_name);
}
