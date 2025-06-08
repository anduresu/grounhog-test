# Dependencies Specification

## Overview

This document catalogs all third-party dependencies used in the Groundhog project, their purposes, licensing information, and usage guidelines.

## Dependency Categories

### Core Dependencies
Required for basic application functionality.

### Development Dependencies
Required only during development and testing.

### Optional Dependencies
Enable additional features when included.

---

## Core Dependencies

### Command Line Parsing

#### `clap` v4.0+
**Purpose**: Command-line argument parsing and help generation  
**License**: MIT OR Apache-2.0  
**Features Used**: `derive`  
**Justification**: Industry standard for Rust CLI applications with excellent derive macro support  

```toml
clap = { version = "4.0", features = ["derive"] }
```

**Usage Example**:
```rust
use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(name = "groundhog")]
#[command(about = "AI coding assistant")]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    Explain,
}
```

### Observability and Logging

#### `tracing` v0.1
**Purpose**: Structured logging, metrics, and distributed tracing  
**License**: MIT  
**Features Used**: Default  
**Justification**: Best-in-class observability for Rust applications  

```toml
tracing = "0.1"
```

**Usage Example**:
```rust
use tracing::{info, instrument};

#[instrument]
pub fn execute_command() -> Result<()> {
    info!("Starting command execution");
    // Command logic...
    Ok(())
}
```

#### `tracing-subscriber` v0.3
**Purpose**: Tracing output formatting and filtering  
**License**: MIT  
**Features Used**: `env-filter`  
**Justification**: Standard companion crate for tracing output management  

```toml
tracing-subscriber = { version = "0.3", features = ["env-filter"] }
```

**Usage Example**:
```rust
use tracing_subscriber::{fmt, EnvFilter};

fn init_tracing() {
    fmt()
        .with_env_filter(EnvFilter::from_default_env())
        .init();
}
```

### Async Runtime

#### `tokio` v1.0
**Purpose**: Async runtime for future async operations  
**License**: MIT  
**Features Used**: `full`  
**Justification**: De facto standard async runtime, needed for future AI service integration  

```toml
tokio = { version = "1.0", features = ["full"] }
```

**Usage Example**:
```rust
#[tokio::main]
async fn main() -> Result<()> {
    // Async application logic
    Ok(())
}
```

### Terminal User Interface

#### `ratatui` v0.28
**Purpose**: Terminal user interface framework  
**License**: MIT  
**Features Used**: Default  
**Justification**: Modern, feature-rich TUI library for building interactive terminal applications  

```toml
ratatui = "0.28"
```

**Usage Example**:
```rust
use ratatui::{
    backend::CrosstermBackend,
    widgets::{Block, Borders, Paragraph},
    Terminal,
};

let backend = CrosstermBackend::new(std::io::stdout());
let mut terminal = Terminal::new(backend)?;
```

#### `crossterm` v0.28
**Purpose**: Cross-platform terminal manipulation  
**License**: MIT  
**Features Used**: Default  
**Justification**: Required companion to ratatui for terminal control and event handling  

```toml
crossterm = "0.28"
```

**Usage Example**:
```rust
use crossterm::{
    event::{self, Event, KeyCode},
    terminal::{enable_raw_mode, disable_raw_mode},
};

enable_raw_mode()?;
if let Event::Key(key) = event::read()? {
    match key.code {
        KeyCode::Char('q') => break,
        _ => {}
    }
}
disable_raw_mode()?;
```

---

## Development Dependencies

### Testing Framework

#### `assert_cmd` v2.0
**Purpose**: Integration testing for CLI applications  
**License**: MIT OR Apache-2.0  
**Justification**: Specialized testing for command-line tools  

```toml
[dev-dependencies]
assert_cmd = "2.0"
```

**Usage Example**:
```rust
use assert_cmd::Command;

#[test]
fn test_explain_command() {
    let mut cmd = Command::cargo_bin("groundhog").unwrap();
    cmd.arg("explain")
       .assert()
       .success()
       .stdout("hello world\n");
}
```

#### `predicates` v3.0
**Purpose**: Assertion predicates for flexible testing  
**License**: MIT OR Apache-2.0  
**Justification**: Companion to assert_cmd for complex assertions  

```toml
[dev-dependencies]
predicates = "3.0"
```

#### `tempfile` v3.0
**Purpose**: Temporary file and directory creation for tests  
**License**: MIT OR Apache-2.0  
**Justification**: Safe temporary resource management in tests  

```toml
[dev-dependencies]
tempfile = "3.0"
```

#### `proptest` v1.0
**Purpose**: Property-based testing  
**License**: MIT OR Apache-2.0  
**Justification**: Comprehensive testing of edge cases  

```toml
[dev-dependencies]
proptest = "1.0"
```

### Benchmarking

#### `criterion` v0.5
**Purpose**: Micro-benchmarking framework  
**License**: MIT OR Apache-2.0  
**Justification**: Statistical benchmarking with detailed analysis  

```toml
[dev-dependencies]
criterion = { version = "0.5", features = ["html_reports"] }
```

### Code Quality

#### `tracing-test` v0.2
**Purpose**: Testing tracing output in unit tests  
**License**: MIT  
**Justification**: Verifying logging behavior in tests  

```toml
[dev-dependencies]
tracing-test = "0.2"
```

---

## Future Dependencies

### Error Handling

#### `thiserror` v1.0
**Purpose**: Procedural macro for error type generation  
**License**: MIT OR Apache-2.0  
**Status**: Planned for v0.2.0  

```toml
thiserror = "1.0"
```

#### `anyhow` v1.0
**Purpose**: Flexible error handling for applications  
**License**: MIT OR Apache-2.0  
**Status**: Considering for error context  

```toml
anyhow = "1.0"
```

### Configuration

#### `serde` v1.0
**Purpose**: Serialization/deserialization framework  
**License**: MIT OR Apache-2.0  
**Features**: `derive`  
**Status**: Planned for configuration management  

```toml
serde = { version = "1.0", features = ["derive"] }
```

#### `toml` v0.8
**Purpose**: TOML configuration file parsing  
**License**: MIT OR Apache-2.0  
**Status**: Planned for configuration files  

```toml
toml = "0.8"
```

#### `dirs` v5.0
**Purpose**: Platform-specific directory paths  
**License**: MIT OR Apache-2.0  
**Status**: Planned for config file location  

```toml
dirs = "5.0"
```

### AI Integration

#### `reqwest` v0.11
**Purpose**: HTTP client for AI service APIs  
**License**: MIT OR Apache-2.0  
**Features**: `json`, `rustls-tls`  
**Status**: Planned for AI service integration  

```toml
reqwest = { version = "0.11", features = ["json", "rustls-tls"] }
```

#### `serde_json` v1.0
**Purpose**: JSON serialization for API communication  
**License**: MIT OR Apache-2.0  
**Status**: Planned for AI API integration  

```toml
serde_json = "1.0"
```

### File Processing

#### `walkdir` v2.0
**Purpose**: Recursive directory traversal  
**License**: MIT OR Apache-2.0  
**Status**: Planned for file analysis commands  

```toml
walkdir = "2.0"
```

#### `ignore` v0.4
**Purpose**: Gitignore-style file filtering  
**License**: MIT OR Apache-2.0  
**Status**: Planned for respecting .gitignore files  

```toml
ignore = "0.4"
```

---

## Dependency Management Guidelines

### Version Pinning Strategy

#### Semantic Versioning Approach
- **Major versions**: Pin to avoid breaking changes
- **Minor versions**: Allow updates for new features
- **Patch versions**: Allow automatic updates for bug fixes

```toml
# Pin major version, allow minor/patch updates
clap = "4.0"

# Pin major.minor for stability-critical dependencies
tracing = "0.1"

# Pin exact version for security-sensitive dependencies
# (when necessary)
```

### License Compatibility

#### Approved Licenses
- **MIT**: Compatible
- **Apache-2.0**: Compatible  
- **MIT OR Apache-2.0**: Preferred (dual-licensed)
- **BSD-3-Clause**: Compatible

#### Restricted Licenses
- **GPL/LGPL**: Avoid due to copyleft requirements
- **AGPL**: Prohibited
- **Commercial**: Case-by-case evaluation

### Security Considerations

#### Dependency Auditing
```bash
# Regular security audits
cargo audit

# Update security advisories database
cargo audit fetch

# Generate audit reports
cargo audit --json > audit-report.json
```

#### Dependency Review Process
1. **License verification**: Ensure compatible licensing
2. **Maintenance status**: Verify active maintenance
3. **Security history**: Review past vulnerabilities
4. **Alternatives evaluation**: Consider alternatives
5. **Minimal feature inclusion**: Only enable needed features

### Monitoring and Updates

#### Automated Monitoring
Dependency monitoring can be configured through your repository platform's dependency management tools or external services like:

- **Dependabot** (for platforms that support it)
- **Renovate Bot** (cross-platform)
- **David DM** (for JavaScript/Node.js dependencies)

Example configuration principles:
- Weekly dependency scans
- Automatic security updates
- Limited number of open PRs (typically 10)
- Separate PRs for major vs minor updates

#### Manual Review Process
1. **Monthly dependency review**: Check for updates
2. **Security patch priority**: Apply security updates immediately  
3. **Breaking change assessment**: Evaluate impact of major updates
4. **Testing requirements**: Full test suite on dependency updates

---

## Build Dependencies

### Build Tools

#### `cargo` (bundled with Rust)
**Purpose**: Package manager and build tool  
**License**: MIT OR Apache-2.0  
**Justification**: Standard Rust toolchain component  

### Additional Build Tools

#### `cargo-audit`
**Purpose**: Security vulnerability scanning  
**Installation**: `cargo install cargo-audit`  
**License**: MIT OR Apache-2.0  

#### `cargo-deny`
**Purpose**: Dependency policy enforcement  
**Installation**: `cargo install cargo-deny`  
**License**: MIT OR Apache-2.0  

#### `cargo-outdated`
**Purpose**: Check for outdated dependencies  
**Installation**: `cargo install cargo-outdated`  
**License**: MIT  

---

## Platform-Specific Dependencies

### Windows
```toml
[target.'cfg(windows)'.dependencies]
# Windows-specific dependencies (if needed)
```

### macOS
```toml
[target.'cfg(target_os = "macos")'.dependencies]
# macOS-specific dependencies (if needed)
```

### Linux
```toml
[target.'cfg(target_os = "linux")'.dependencies]
# Linux-specific dependencies (if needed)
```

---

## Dependency Configuration

### Feature Flags

#### Conditional Compilation
```rust
#[cfg(feature = "ai-integration")]
mod ai {
    // AI-related functionality
}

#[cfg(not(feature = "ai-integration"))]
mod ai {
    // Stub implementation
}
```

#### Feature Definition
```toml
[features]
default = ["tracing"]
ai-integration = ["reqwest", "serde_json"]
config-files = ["serde", "toml"]
full = ["ai-integration", "config-files"]
```

### Dependency Groups
```toml
[dependencies]
# Core runtime dependencies
clap = { version = "4.0", features = ["derive"] }
tracing = "0.1"
tracing-subscriber = { version = "0.3", features = ["env-filter"] }
tokio = { version = "1.0", features = ["full"] }

# Optional feature dependencies
serde = { version = "1.0", features = ["derive"], optional = true }
toml = { version = "0.8", optional = true }
reqwest = { version = "0.11", features = ["json"], optional = true }

[dev-dependencies]
# Testing dependencies
assert_cmd = "2.0"
predicates = "3.0"
tempfile = "3.0"
proptest = "1.0"
criterion = { version = "0.5", features = ["html_reports"] }
tracing-test = "0.2"
```

---

## Maintenance Schedule

### Regular Tasks

#### Weekly
- Monitor security advisories
- Review Dependabot PRs

#### Monthly  
- Full dependency audit
- Update patch versions
- Review deprecated dependencies

#### Quarterly
- Evaluate new dependencies
- Major version update planning
- License compliance review

#### Annually
- Complete dependency landscape review
- Alternative evaluation
- Dependency reduction opportunities

### Emergency Procedures

#### Security Vulnerabilities
1. **Immediate assessment** of vulnerability impact
2. **Emergency update** if actively exploited
3. **Testing** of security patches
4. **Deployment** of fixed version
5. **Post-incident review** and documentation 