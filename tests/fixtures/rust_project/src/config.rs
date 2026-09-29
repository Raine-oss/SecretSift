// Rust Fixture Config

pub struct AppConfig {
    pub app_name: String,
    pub port: String,
    pub db_url: String,
}

pub fn load_config() -> AppConfig {
    let app_name = "SecureApp".to_string();
    let port = std::env::var("PORT").unwrap_or_else(|_| "8080".to_string());
    let database_url = "postgres://admin:supersecret987654321@postgres.internal:5432/production_db";

    AppConfig {
        app_name,
        port,
        db_url: database_url.to_string(),
    }
}
