# Release Notes

## v1.0.0

### Summary
Initial production release of SecretSift: a Rust-powered developer security utility to turn private codebases into public-safe repositories with secret scanning, confidence classification, preview diffs, automated `.env` extraction, `.gitignore` protection, and atomic transactional writes.

### Key Features
- **Multi-Confidence Secret Scanner**: Detects OpenAI API keys, AWS access keys, GitHub tokens, Slack tokens, Stripe keys, Database URLs, and Private Keys with `HIGH` confidence. Uses Shannon entropy scoring for generic secret heuristics.
- **Language Adapters**:
  - Rust (`std::env::var("...").unwrap_or_default()`, `&std::env::var(...)` in function calls, and `format!` interpolations).
  - Java (`System.getenv("...")` across fields, assignments, arguments, and return statements).
  - JavaScript & TypeScript (`process.env.... || ""` with string fallback to prevent `undefined` runtime exceptions).
  - Python (`os.getenv("...")` with clean, top-level `import os` insertion).
- **Interactive Safe Fix Engine**: Renders colored terminal diffs for code, `.env`, `.env.example`, and `.gitignore` prior to user confirmation.
- **Timestamped Snapshots and Atomic Rollbacks**: Backs up modified files to `.secretsift-backup/<timestamp>/` and automatically restores files on I/O write failure.
- **CI/CD Integration (`secretsift check`)**: Lightweight check utility exiting with code `0` on clean repos and code `1` on detected high-confidence secrets.
- **Multi-Language Compiler Validation**: Tested against real multi-file projects using native compilers (`cargo check`, `javac`, `python3 -m py_compile`, `node -c`).

### Verification
- 21 automated tests across 7 test suites passing (100% pass rate).
- Zero compiler and Clippy warnings under `-D warnings`.
- Performance benchmarks established via Criterion.
