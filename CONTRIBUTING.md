# Contributing to SecretSift

Thank you for your interest in contributing to SecretSift. This document outlines the process for proposing changes, reporting issues, and submitting pull requests.

---

## Project Philosophy and Scope

SecretSift is designed as a developer-first tool to convert private repositories into public-safe codebases through pattern scanning, entropy classification, and safe language-aware refactoring.

To preserve stability and reliability:
- Transformations must be deterministic and syntax-preserving.
- High-confidence rules must be backed by explicit pattern signatures.
- All code modifications must remain safe, backed up, and user-confirmed.

---

## Development Setup

### Prerequisites
- Stable Rust toolchain (1.80.0+)
- Cargo package manager
- Git

### Building the Project
```bash
git clone https://github.com/Raine-oss/SecretSift.git
cd SecretSift
cargo build
```

### Running Tests
All contributions must pass the full test suite:
```bash
cargo test
```

### Running Benchmarks
```bash
cargo bench
```

### Code Formatting and Lints
The codebase enforces standard formatting and zero Clippy warnings:
```bash
cargo fmt --all -- --check
cargo clippy --all-targets --all-features -- -D warnings
```

---

## Contribution Workflow

1. Fork the repository on GitHub.
2. Create a feature branch (`git checkout -b feature/your-feature`).
3. Implement your changes adhering to existing code conventions.
4. Ensure all unit and integration tests pass.
5. Format the code with `cargo fmt`.
6. Commit your changes using descriptive commit messages.
7. Push the branch to your fork and open a Pull Request.

---

## Pull Request Guidelines

- Keep PRs focused on a single change, bug fix, or language adapter.
- Include corresponding unit or integration tests for any logic modifications.
- Update documentation in `README.md` if command-line options or behaviors change.
