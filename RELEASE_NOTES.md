# Release Notes

## v1.1.0

### Summary
Major language expansion and validation upgrade. SecretSift now supports 13 programming languages across individual language adapters, 13 real fixture projects, expanded test suites (38 tests), multi-tiered syntax/compiler validation, and comprehensive multi-language idempotency guarantees.

### Key Additions & Improvements
- **13 Supported Programming Languages**:
  - **Rust**: `std::env::var("VAR").unwrap_or_default()`, `&std::env::var(...)`, and `format!` interpolations.
  - **Java**: `System.getenv("VAR")` for field declarations, variable assignments, method arguments, and returns.
  - **JavaScript**: `process.env.VAR || ""` with dedicated adapter.
  - **TypeScript**: `process.env.VAR || ""` compatible with strict type-checking.
  - **Python**: `os.getenv("VAR")` with non-destructive `import os` insertion.
  - **Go**: `os.Getenv("VAR")` with automated AST-friendly `import "os"` block injection.
  - **C#**: `Environment.GetEnvironmentVariable("VAR") ?? ""` with automated `using System;` injection.
  - **Kotlin**: `System.getenv("VAR") ?: ""` for JVM null-safety.
  - **PHP**: `getenv('VAR') ?: ''` for standard environment extraction.
  - **Ruby**: `ENV['VAR'] || ''` for clean hash-based environment access.
  - **C & C++**: `getenv("VAR")` with automated `#include <cstdlib>` / `#include <stdlib.h>` management (C and C++ share the unified `cpp.rs` adapter implementation).
  - **Dart**: `Platform.environment['VAR'] ?? ''` with automated `import 'dart:io';` insertion.
  - **Swift**: `ProcessInfo.processInfo.environment["VAR"] ?? ""` with automated `import Foundation` insertion.
- **Validation Tiers**:
  - **Tier 1 (Native Compiler Verification)**: `cargo check`, `javac`, `python3 -m py_compile`, `node -c`, `g++ -fsyntax-only`.
  - **Tier 2 (Toolchain-Conditional Compiler & AST Structural Verification)**: `go vet` / `go build`, `dotnet build`, `kotlinc`, `php -l`, `ruby -c`, `dart analyze`, `swiftc -parse` executed when respective toolchains are available, accompanied by strict AST structural correctness assertions.
- **Real Fixture Projects**: Expanded `tests/fixtures/` to 13 real multi-file projects across all supported languages.
- **Full Test Suite**: 38 automated tests across 7 suites passing with 100% success rate.
- **Multi-Language Idempotency**: Verified across 12 source files in a single pass.

---

## v1.0.0

### Summary
Initial production release of SecretSift: a Rust-powered developer security utility to turn private codebases into public-safe repositories with secret scanning, confidence classification, preview diffs, automated `.env` extraction, `.gitignore` protection, and atomic transactional writes.

### Key Features
- **Multi-Confidence Secret Scanner**: Detects OpenAI API keys, AWS access keys, GitHub tokens, Slack tokens, Stripe keys, Database URLs, and Private Keys with `HIGH` confidence. Uses Shannon entropy scoring for generic secret heuristics.
- **Language Adapters**: Rust, Java, JavaScript, TypeScript, and Python.
- **Interactive Safe Fix Engine**: Renders colored terminal diffs for code, `.env`, `.env.example`, and `.gitignore` prior to user confirmation.
- **Timestamped Snapshots and Atomic Rollbacks**: Backs up modified files to `.secretsift-backup/<timestamp>/` and automatically restores files on I/O write failure.
- **CI/CD Integration (`secretsift check`)**: Lightweight check utility exiting with code `0` on clean repos and code `1` on detected high-confidence secrets.
