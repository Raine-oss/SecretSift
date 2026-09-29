# SecretSift

[![Rust](https://img.shields.io/badge/rust-stable-brightgreen.svg)](https://www.rust-lang.org/)
[![Release](https://img.shields.io/github/v/release/Raine-oss/SecretSift?color=blue)](https://github.com/Raine-oss/SecretSift/releases)
[![License: MIT](https://img.shields.io/badge/License-MIT-blue.svg)](LICENSE)
[![CI](https://github.com/Raine-oss/SecretSift/actions/workflows/ci.yml/badge.svg)](https://github.com/Raine-oss/SecretSift/actions/workflows/ci.yml)

Turn a private codebase into a public-safe repository.

SecretSift inspects repositories for hardcoded credentials, classifies findings with confidence scoring, previews language-aware refactoring into environment variable accessors, extracts secrets into `.env` and `.env.example`, ensures `.env` is git-ignored, and applies modifications atomically with timestamped backups.

[![View Releases](https://img.shields.io/badge/GitHub-Releases-181717?style=for-the-badge&logo=github&logoColor=white)](https://github.com/Raine-oss/SecretSift/releases)
[![Report Issue](https://img.shields.io/badge/GitHub-Issues-blue?style=for-the-badge&logo=github&logoColor=white)](https://github.com/Raine-oss/SecretSift/issues)
[![Read Documentation](https://img.shields.io/badge/Docs-Release_Notes-2ea44f?style=for-the-badge&logo=readme&logoColor=white)](RELEASE_NOTES.md)

---

## What It Does

When preparing a private repository for public open-sourcing or team sharing, developers often have hardcoded secrets, connection strings, and API tokens scattered across their code.

SecretSift provides an automated refactoring pipeline:

1. **Scan**: Traverses the codebase respecting `.gitignore` rules and skipping build directories.
2. **Analyze**: Categorizes candidates into `HIGH`, `MEDIUM`, and `LOW` confidence using token signatures and Shannon entropy heuristics.
3. **Preview**: Renders unified terminal diffs showing exactly how source code, `.env`, `.env.example`, and `.gitignore` will be modified.
4. **Fix**: Safely replaces literals with idiomatic environment variable calls across supported programming languages.
5. **Protect**: Automatically adds `.env` to `.gitignore` during fix operations and creates a restorable snapshot in `.secretsift-backup/`.

---

## Installation

### Method 1: Cargo Install (From Source Repository)

Ensure a stable Rust toolchain is installed:

```bash
cargo install --git https://github.com/Raine-oss/SecretSift.git
```

### Method 2: Download Prebuilt Release Binaries

Download precompiled standalone binaries from the [GitHub Releases page](https://github.com/Raine-oss/SecretSift/releases):

```bash
# Example for Linux x86_64
curl -LO https://github.com/Raine-oss/SecretSift/releases/download/v1.1.0/secretsift-linux-x86_64.tar.gz
tar -xzf secretsift-linux-x86_64.tar.gz
chmod +x secretsift
sudo mv secretsift /usr/local/bin/
```

### Method 3: Build From Source

```bash
git clone https://github.com/Raine-oss/SecretSift.git
cd SecretSift
cargo build --release
sudo cp target/release/secretsift /usr/local/bin/
```

---

## Quick Start

Navigate to any project directory:

```bash
cd my-project

# 1. Scan for hardcoded credentials
secretsift scan

# 2. Preview the proposed modifications without touching disk
secretsift fix --dry-run

# 3. Interactively apply refactoring and extract .env
secretsift fix
```

---

## Commands

### Syntax

```text
secretsift [PATH] [COMMAND]
```

### Subcommands

| Command | Description |
| :--- | :--- |
| `scan [PATH]` | Scans the target directory and prints categorized findings without making modifications. |
| `fix [PATH]` | Constructs a FixPlan, shows a unified diff preview, creates a backup, and refactors source files. |
| `check [PATH]` | CI/CD verification utility. Exits with code `0` if clean, or code `1` if secrets match the threshold. |

### Global and Command Options

| Option | Command | Default | Description |
| :--- | :--- | :--- | :--- |
| `--json` | `scan` | `false` | Emits structured JSON findings for external scripts and tooling. |
| `--min-confidence <LEVEL>` | `scan`, `fix`, `check` | `low` (scan), `high` (fix/check) | Minimum confidence threshold: `high`, `medium`, `low`. |
| `--dry-run` | `fix` | `false` | Generates and prints diff preview without writing any files to disk. |
| `-y`, `--yes` | `fix` | `false` | Automatically confirms and applies changes non-interactively. |
| `-h`, `--help` | All | | Prints help and option descriptions. |
| `-V`, `--version` | All | | Prints application version. |

---

## Supported Languages

SecretSift utilizes language-specific refactoring adapters to generate syntactically valid code:

| Language | Extensions | Refactoring Strategy |
| :--- | :--- | :--- |
| **Rust** | `.rs` | `std::env::var("VAR").unwrap_or_default()`, `&std::env::var(...)` for function arguments, and `format!` interpolations. |
| **Java** | `.java` | `System.getenv("VAR")` for field declarations, variable assignments, method arguments, and returns. |
| **JavaScript** | `.js`, `.jsx`, `.mjs`, `.cjs` | `process.env.VAR \|\| ""` to ensure string type safety and prevent `undefined` runtime exceptions. |
| **TypeScript** | `.ts`, `.tsx`, `.mts`, `.cts` | `process.env.VAR \|\| ""` compatible with strict TypeScript type systems. |
| **Python** | `.py` | `os.getenv("VAR")` with automated, non-destructive `import os` insertion. |
| **Go** | `.go` | `os.Getenv("VAR")` with automated AST-friendly `import "os"` management. |
| **C#** | `.cs` | `Environment.GetEnvironmentVariable("VAR") ?? ""` with `using System;` management. |
| **Kotlin** | `.kt`, `.kts` | `System.getenv("VAR") ?: ""` for null-safe JVM execution. |
| **PHP** | `.php` | `getenv('VAR') ?: ''` for standard runtime environment resolution. |
| **Ruby** | `.rb` | `ENV['VAR'] \|\| ''` for clean hash-based environment access. |
| **C / C++** | `.c`, `.h`, `.cpp`, `.cc`, `.cxx`, `.hpp` | `getenv("VAR")` with automated `#include <stdlib.h>` / `#include <cstdlib>` management. |
| **Dart** | `.dart` | `Platform.environment['VAR'] ?? ''` with automated `import 'dart:io';` insertion. |
| **Swift** | `.swift` | `ProcessInfo.processInfo.environment["VAR"] ?? ""` with automated `import Foundation` insertion. |

> **Note:** C and C++ share the same C/C++ adapter implementation (`src/adapters/cpp.rs`).

---

## Detection

SecretSift combines high-precision regular expressions with heuristic analysis:

### High Confidence Patterns
- **OpenAI API Keys**: `sk-[a-zA-Z0-9_\-]{20,}`
- **AWS Access Key IDs**: `AKIA[0-9A-Z]{16}`
- **GitHub Personal Access Tokens**: `ghp_...`, `github_pat_...`
- **Slack Tokens**: `xoxb-...`, `xoxp-...`, `xoxa-...`
- **Stripe Secret API Keys**: `sk_live_[0-9a-zA-Z]{24,}`
- **Database Connection Strings**: `postgres://`, `mysql://`, `mongodb://`, `redis://` containing credentials.
- **Private Cryptographic Keys**: PEM-encoded RSA, EC, DSA, and OpenSSH private keys.

### Medium and Low Confidence Heuristics
- Variable assignment matching for identifiers containing `key`, `secret`, `token`, `password`, `auth`, `database_url`.
- Shannon entropy evaluation to separate pseudo-random cryptographic tokens (entropy >= 3.2) from readable text.
- Comprehensive false-positive filters excluding common test placeholders (`your-api-key`, `dummy`, `placeholder`, `123456`, `changeme`).
- Protection for existing environment variable calls (`std::env::var`, `System.getenv`, `process.env`, `os.getenv`).

---

## Safe Fixing

SecretSift does not perform blind string replacements. Fix operations execute through a multi-stage validation engine:

```text
Scan Findings
     |
     v
Build FixPlan (Group by file, route to language adapter)
     |
     v
Validate FixPlan (Verify non-empty output and syntax structure)
     |
     v
Display Unified Diff Preview
     |
     v
User Confirmation [y/N]
     |
     v
Create Snapshot in .secretsift-backup/<timestamp>/
     |
     v
Atomic Write (.env + .env.example + .gitignore + Code Refactoring)
     |
 (On I/O failure: Automatic rollback from snapshot)
```

---

## CI Usage

Integrate SecretSift into continuous integration workflows and pre-commit checks using `secretsift check`:

### GitHub Actions Workflow

```yaml
name: Security Audit

on: [push, pull_request]

jobs:
  secret-scan:
    name: Check for Hardcoded Secrets
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v4
      - uses: dtolnay/rust-toolchain@stable
      - name: Install SecretSift
        run: cargo install --git https://github.com/Raine-oss/SecretSift.git
      - name: Run SecretSift Check
        run: secretsift check .
```

### Pre-commit Hook

Add to `.git/hooks/pre-commit`:

```bash
#!/bin/sh
secretsift check .
```

---

## Safety Model

- **Zero Unprompted Modifications**: Running `secretsift scan` is strictly read-only.
- **Preservation of Relative Layout**: Snapshots in `.secretsift-backup/` mirror the exact relative file tree of modified files.
- **Idempotency**: Running `secretsift fix` repeatedly on an already-refactored codebase produces zero changes and avoids duplicate `.env` entries.
- **Gitignore Enforcement**: Ensures `.env` is present in `.gitignore` during fix operations so newly extracted credentials are not committed.

---

## Limitations

- **Complex AST Re-writes**: Compile-time constant expressions in languages like Rust (`const X: &str = ...`) require manual runtime architecture restructuring.
- **Obfuscated Credentials**: Encrypted, split, or dynamically constructed strings may not trigger pattern detectors.
- **Template Engines**: Custom template files outside standard language parsers require manual verification.

---

## Development / Testing

### Validation Tiers

SecretSift enforces rigorous verification across all supported languages:

1. **Native Compiler & Syntax Verification** (Run during standard test suite when toolchains are installed):
   - **Rust**: `cargo check`
   - **Java**: `javac`
   - **Python**: `python3 -m py_compile`
   - **JavaScript**: `node -c`
   - **C / C++**: `g++ -fsyntax-only`

2. **Toolchain-Conditional Compiler Validation & Structural AST Verification**:
   - **Go**: `go vet` / `go build` (conditional) + automated `import "os"` AST block injection verification.
   - **C#**: `dotnet build` (conditional) + `using System;` namespace injection verification.
   - **Kotlin**: `kotlinc` (conditional) + JVM null-safe Elvis syntax verification.
   - **PHP**: `php -l` (conditional) + `getenv(...)` runtime syntax verification.
   - **Ruby**: `ruby -c` (conditional) + hash environment syntax verification.
   - **Dart**: `dart analyze` (conditional) + `import 'dart:io';` import injection verification.
   - **Swift**: `swiftc -parse` (conditional) + `import Foundation` import injection verification.

### Prerequisites
- Stable Rust toolchain (Rust 1.80.0+)
- Cargo package manager
- Node.js, Python 3, OpenJDK, GCC/G++ (standard verification tools)

### Building and Testing

```bash
# Clone the repository
git clone https://github.com/Raine-oss/SecretSift.git
cd SecretSift

# Check formatting
cargo fmt --all -- --check

# Run linter
cargo clippy --all-targets --all-features -- -D warnings

# Execute full test suite (38 tests across 7 suites)
cargo test

# Run Criterion benchmarks
cargo bench
```

---

## License

This project is licensed under the MIT License. See the [LICENSE](LICENSE) file for details.
