# Development Workflow Specification

## Overview

This document outlines the development setup, testing procedures, build processes, and deployment workflows for the Groundhog project.

## Development Environment Setup

### Prerequisites

- **Rust**: Latest stable version (1.87.0+)
- **Git**: Version control
- **IDE/Editor**: VS Code with rust-analyzer or any Rust-compatible editor

### Initial Setup

```bash
# Clone the repository
git clone <repository-url>
cd groundhog

# Install Rust (if not already installed)
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
source ~/.cargo/env

# Install additional tools
cargo install cargo-watch    # For auto-recompilation
cargo install cargo-nextest  # For improved testing
cargo install cargo-audit    # For security auditing
cargo install cargo-deny     # For license and dependency checking

# Build the project
cargo build

# Run tests
cargo test

# Run the application
cargo run -- explain
```

### IDE Configuration

#### VS Code Settings

```json
{
  "rust-analyzer.cargo.features": "all",
  "rust-analyzer.checkOnSave.command": "clippy",
  "rust-analyzer.rustfmt.extraArgs": [
    "+nightly"
  ],
  "files.associations": {
    "*.rs": "rust"
  }
}
```

#### Recommended Extensions

- rust-analyzer
- Better TOML
- GitLens
- Error Lens

## Project Structure

```
groundhog/
├── Cargo.toml              # Project configuration
├── Cargo.lock              # Dependency lock file
├── README.md               # Project overview
├── SPECS.md                # Specifications overview
├── CHANGELOG.md            # Version history
├── LICENSE                 # License file
├── .gitignore              # Git ignore rules
├── .ci/                    # CI/CD workflows and scripts
│   ├── workflows/          # Workflow definitions
│   └── scripts/            # Build and deployment scripts
├── specs/                  # Specification documents
├── src/                    # Source code
│   ├── main.rs             # Application entry point
│   ├── lib.rs              # Library exports
│   ├── cli/                # CLI module
│   ├── core/               # Core business logic
│   └── infrastructure/     # Cross-cutting concerns
├── tests/                  # Integration tests
│   ├── integration/        # Integration test suites
│   └── fixtures/           # Test data
├── benches/                # Benchmarks
├── examples/               # Usage examples
└── docs/                   # Additional documentation
```

## Development Workflow

### Branch Strategy

#### Main Branches
- `main`: Production-ready code
- `develop`: Integration branch for features

#### Feature Branches
- `feature/<feature-name>`: New features
- `bugfix/<issue-number>`: Bug fixes
- `hotfix/<issue-number>`: Critical production fixes

#### Example Workflow
```bash
# Start a new feature
git checkout develop
git pull origin develop
git checkout -b feature/new-command

# Work on feature...
git add .
git commit -m "feat: add new command implementation"

# Push and create PR
git push origin feature/new-command
# Create pull request via your repository platform
```

### Commit Convention

Following [Conventional Commits](https://www.conventionalcommits.org/):

```
<type>[optional scope]: <description>

[optional body]

[optional footer(s)]
```

#### Types
- `feat`: New features
- `fix`: Bug fixes
- `docs`: Documentation changes
- `style`: Code style changes
- `refactor`: Code refactoring
- `perf`: Performance improvements
- `test`: Test additions/changes
- `chore`: Build process or auxiliary tool changes

#### Examples
```
feat(cli): add explain command
fix(logging): resolve tracing subscriber initialization
docs(specs): update architecture documentation
test(commands): add integration tests for explain command
```

## Testing Strategy

### Test Categories

#### Unit Tests
```bash
# Run all unit tests
cargo test

# Run tests with output
cargo test -- --nocapture

# Test specific module
cargo test cli::commands::explain
```

#### Integration Tests
```bash
# Run integration tests
cargo test --test integration

# Run specific integration test
cargo test --test cli_integration
```

#### Property-Based Tests
```rust
use proptest::prelude::*;

proptest! {
    #[test]
    fn test_config_roundtrip(config in config_strategy()) {
        let serialized = toml::to_string(&config)?;
        let deserialized: Config = toml::from_str(&serialized)?;
        prop_assert_eq!(config, deserialized);
    }
}
```

#### Performance Tests
```bash
# Run benchmarks
cargo bench

# Profile specific benchmark
cargo bench --bench command_bench
```

### Test Organization

```rust
// Unit test structure
#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_function_behavior() {
        // Test implementation
    }
    
    #[test]
    fn test_error_conditions() {
        // Error case testing
    }
}

// Integration test structure
// tests/integration/cli_tests.rs
use assert_cmd::Command;
use predicates::prelude::*;

#[test]
fn test_explain_command() {
    let mut cmd = Command::cargo_bin("groundhog").unwrap();
    cmd.arg("explain")
       .assert()
       .success()
       .stdout(predicate::str::contains("hello world"));
}
```

### Test Data Management

```rust
// Test fixtures
pub fn create_test_config() -> Config {
    Config {
        logging: LoggingConfig {
            level: LogLevel::Debug,
            format: LogFormat::Pretty,
            file: None,
            timestamps: false,
            thread_ids: false,
        },
        // ... other fields
    }
}

// Temporary directories for tests
use tempfile::TempDir;

#[test]
fn test_config_file_loading() {
    let temp_dir = TempDir::new().unwrap();
    let config_path = temp_dir.path().join("config.toml");
    
    // Test implementation...
}
```

## Code Quality

### Linting and Formatting

```bash
# Format code
cargo fmt

# Check formatting without applying
cargo fmt -- --check

# Run clippy lints
cargo clippy

# Run clippy with all features
cargo clippy --all-features -- -D warnings
```

### Clippy Configuration

```toml
# Cargo.toml
[lints.clippy]
# Deny clippy lints
all = "deny"
pedantic = "deny"
nursery = "warn"

# Allow specific lints
module_name_repetitions = "allow"
missing_errors_doc = "allow"
```

### Pre-commit Hooks

```bash
# Install pre-commit
pip install pre-commit

# Setup hooks
pre-commit install
```

`.pre-commit-config.yaml`:
```yaml
repos:
  - repo: local
    hooks:
      - id: cargo-fmt
        name: cargo fmt
        entry: cargo fmt --all --
        language: system
        files: \.rs$
        
      - id: cargo-clippy
        name: cargo clippy
        entry: cargo clippy --all-features -- -D warnings
        language: system
        files: \.rs$
        pass_filenames: false
        
      - id: cargo-test
        name: cargo test
        entry: cargo test
        language: system
        files: \.rs$
        pass_filenames: false
```

## Build Process

### Local Builds

```bash
# Debug build
cargo build

# Release build
cargo build --release

# Build with all features
cargo build --all-features

# Cross-compilation
cargo build --target x86_64-pc-windows-gnu
```

### Build Configuration

```toml
# Cargo.toml
[profile.release]
opt-level = 3
lto = true
codegen-units = 1
panic = "abort"
strip = true

[profile.dev]
opt-level = 0
debug = true
```

### Build Scripts

```bash
#!/bin/bash
# scripts/build.sh

set -e

echo "Building Groundhog..."

# Clean previous builds
cargo clean

# Run tests
cargo test --all-features

# Build release version
cargo build --release

# Run security audit
cargo audit

echo "Build completed successfully!"
```

## Continuous Integration

### CI/CD Pipeline Structure

The CI/CD pipeline should include the following stages:

#### Testing Pipeline
```bash
# Format checking
cargo fmt -- --check

# Linting
cargo clippy --all-features -- -D warnings

# Unit tests
cargo test --all-features

# Integration tests
cargo test --test integration

# Security audit
cargo install cargo-audit
cargo audit
```

#### Coverage Pipeline
```bash
# Install coverage tool
cargo install cargo-tarpaulin

# Generate coverage report
cargo tarpaulin --all-features --out xml

# Upload to coverage service (if configured)
```

#### Build Matrix
Test across multiple Rust versions:
- stable
- beta  
- nightly

Test across multiple platforms:
- Linux (ubuntu-latest)
- Windows (windows-latest)
- macOS (macos-latest)

### Release Pipeline

#### Automated Release Process
```bash
# Build release binaries
cargo build --release

# Create platform-specific archives
tar -czf groundhog-linux-x86_64.tar.gz -C target/release groundhog
```

#### Cross-Platform Builds
Build for multiple targets:
- x86_64-unknown-linux-gnu
- x86_64-pc-windows-gnu
- x86_64-apple-darwin
- aarch64-apple-darwin

## Documentation

### Code Documentation

```rust
//! Module documentation
//! 
//! This module provides...

/// Function documentation
/// 
/// # Arguments
/// 
/// * `arg1` - Description of argument
/// 
/// # Returns
/// 
/// Returns a `Result` containing...
/// 
/// # Examples
/// 
/// ```
/// use groundhog::example_function;
/// 
/// let result = example_function("input")?;
/// assert_eq!(result, "expected");
/// ```
/// 
/// # Errors
/// 
/// This function returns an error if...
pub fn example_function(arg1: &str) -> Result<String> {
    // Implementation
}
```

### Documentation Generation

```bash
# Generate documentation
cargo doc

# Generate and open documentation
cargo doc --open

# Generate documentation with private items
cargo doc --document-private-items
```

## Debugging

### Debug Configuration

```toml
# Cargo.toml
[dependencies]
tracing = "0.1"
tracing-subscriber = { version = "0.3", features = ["env-filter"] }

[dev-dependencies]
tracing-test = "0.2"
```

### Debug Logging

```rust
use tracing::{debug, info, warn, error};

#[tracing::instrument]
fn debug_function(input: &str) -> Result<String> {
    debug!("Processing input: {}", input);
    
    // Function logic...
    
    info!("Function completed successfully");
    Ok(result)
}
```

### Debugging Tools

```bash
# Run with debug logging
RUST_LOG=debug cargo run -- explain

# Run with specific module logging
RUST_LOG=groundhog::cli=trace cargo run -- explain

# Use debugger
rust-gdb target/debug/groundhog
```

## Performance Monitoring

### Benchmarking

```rust
// benches/command_bench.rs
use criterion::{black_box, criterion_group, criterion_main, Criterion};
use groundhog::cli::commands::explain;

fn bench_explain_command(c: &mut Criterion) {
    c.bench_function("explain command", |b| {
        b.iter(|| explain::execute(black_box(&ExplainArgs::default())))
    });
}

criterion_group!(benches, bench_explain_command);
criterion_main!(benches);
```

### Profiling

```bash
# Install profiling tools
cargo install flamegraph
cargo install cargo-profdata

# Generate flame graph
cargo flamegraph --bin groundhog -- explain

# Profile with perf
perf record --call-graph=dwarf cargo run --release -- explain
perf report
```

## Deployment

### Binary Distribution

```bash
# Build optimized binaries
cargo build --release

# Strip debug symbols
strip target/release/groundhog

# Create distribution package
tar -czf groundhog-v0.1.0-linux-x86_64.tar.gz -C target/release groundhog
```

### Package Managers

#### Homebrew Formula
```ruby
class Groundhog < Formula
  desc "AI coding assistant CLI tool"
  homepage "<project-homepage>"
  url "<source-archive-url>"
  sha256 "..."
  
  depends_on "rust" => :build
  
  def install
    system "cargo", "install", *std_cargo_args
  end
  
  test do
    assert_match "hello world", shell_output("#{bin}/groundhog explain")
  end
end
```

#### Cargo Publication
```bash
# Publish to crates.io
cargo publish --dry-run
cargo publish
```

## Maintenance

### Dependency Updates

```bash
# Check for outdated dependencies
cargo outdated

# Update dependencies
cargo update

# Audit for security vulnerabilities
cargo audit
```

### Version Management

```bash
# Update version in Cargo.toml
# Create git tag
git tag v0.1.0
git push origin v0.1.0

# Generate changelog
git cliff --output CHANGELOG.md
``` 