use criterion::{black_box, criterion_group, criterion_main, Criterion};
use secretsift::detector::detect_secrets_in_content;
use secretsift::models::Language;
use std::path::Path;

// Benchmark Scenarios

fn benchmark_detector(c: &mut Criterion) {
    let dummy_path = Path::new("src/config.rs");
    let content = r#"
        // Configuration Module
        const API_KEY: &str = "sk-testabcdef1234567890abcdef1234567890";
        let database_url = "postgres://admin:supersecretpassword@localhost:5432/mydb";
        let stripe_key = "sk_sift_51ABCDEF1234567890abcdef123456";
        let github_token = "ghp_111111111122222222223333333333444444";
        let slack_token = "xoxp-1111111111-22222222222-3333333333333-mocktokenabcdef12345";
        let normal_msg = "Application initialized successfully on port 8080";
        let route = "/api/v1/users/profile/settings";
    "#;

    c.bench_function("detector_multi_pattern", |b| {
        b.iter(|| {
            detect_secrets_in_content(
                black_box(dummy_path),
                black_box(content),
                black_box(Language::Rust),
            )
        })
    });
}

// Benchmark Groups

criterion_group!(benches, benchmark_detector);
criterion_main!(benches);
