# Error Handling Specification

## Overview

Groundhog implements a comprehensive error handling strategy that provides clear, actionable error messages to users while maintaining detailed error context for debugging and observability.

## Error Design Principles

### 1. User-Centric Messages
- Clear, non-technical language for end users
- Actionable suggestions for resolution
- Contextual information about what failed

### 2. Developer-Friendly Context
- Detailed error chains for debugging
- Source location information
- Structured error data for tooling

### 3. Graceful Degradation
- Partial success when possible
- Recovery suggestions
- Safe failure modes

### 4. Observability Integration
- All errors are logged with structured data
- Error correlation across operations
- Performance impact tracking

## Error Types Hierarchy

```rust
#[derive(Debug, thiserror::Error)]
pub enum GroundhogError {
    #[error("Command execution failed")]
    Command(#[from] CommandError),
    
    #[error("Configuration error")]
    Config(#[from] ConfigError),
    
    #[error("File system operation failed")]
    FileSystem(#[from] FileSystemError),
    
    #[error("Network operation failed")]
    Network(#[from] NetworkError),
    
    #[error("Parsing error")]
    Parse(#[from] ParseError),
    
    #[error("Internal application error")]
    Internal(#[from] InternalError),
}
```

## Specific Error Categories

### Command Errors

```rust
#[derive(Debug, thiserror::Error)]
pub enum CommandError {
    #[error("Command '{command}' not found")]
    NotFound { command: String },
    
    #[error("Invalid arguments for command '{command}'")]
    InvalidArguments { 
        command: String, 
        message: String 
    },
    
    #[error("Command '{command}' execution failed")]
    ExecutionFailed { 
        command: String,
        #[source]
        source: Box<dyn std::error::Error + Send + Sync>,
    },
    
    #[error("Permission denied for command '{command}'")]
    PermissionDenied { command: String },
}
```

### Configuration Errors

```rust
#[derive(Debug, thiserror::Error)]
pub enum ConfigError {
    #[error("Configuration file not found at '{path}'")]
    NotFound { path: PathBuf },
    
    #[error("Configuration file has invalid format")]
    InvalidFormat {
        path: PathBuf,
        line: Option<usize>,
        #[source]
        source: Box<dyn std::error::Error + Send + Sync>,
    },
    
    #[error("Missing required configuration key '{key}'")]
    MissingKey { key: String },
    
    #[error("Invalid value for configuration key '{key}': {value}")]
    InvalidValue { 
        key: String, 
        value: String,
        expected: String,
    },
}
```

### File System Errors

```rust
#[derive(Debug, thiserror::Error)]
pub enum FileSystemError {
    #[error("File not found: '{path}'")]
    NotFound { path: PathBuf },
    
    #[error("Permission denied accessing '{path}'")]
    PermissionDenied { path: PathBuf },
    
    #[error("File '{path}' is not readable")]
    NotReadable { path: PathBuf },
    
    #[error("File '{path}' is not writable")]
    NotWritable { path: PathBuf },
    
    #[error("Directory '{path}' is not accessible")]
    DirectoryNotAccessible { path: PathBuf },
    
    #[error("Invalid file format for '{path}'")]
    InvalidFormat { 
        path: PathBuf,
        expected: String,
        #[source]
        source: Box<dyn std::error::Error + Send + Sync>,
    },
}
```

### Network Errors

```rust
#[derive(Debug, thiserror::Error)]
pub enum NetworkError {
    #[error("Failed to connect to '{url}'")]
    ConnectionFailed { 
        url: String,
        #[source]
        source: Box<dyn std::error::Error + Send + Sync>,
    },
    
    #[error("Request timeout after {timeout_ms}ms")]
    Timeout { timeout_ms: u64 },
    
    #[error("HTTP error {status}: {message}")]
    Http { 
        status: u16, 
        message: String 
    },
    
    #[error("Invalid URL: '{url}'")]
    InvalidUrl { url: String },
    
    #[error("Authentication failed")]
    AuthenticationFailed,
}
```

### Parse Errors

```rust
#[derive(Debug, thiserror::Error)]
pub enum ParseError {
    #[error("JSON parsing failed")]
    Json {
        input: String,
        line: Option<usize>,
        column: Option<usize>,
        #[source]
        source: serde_json::Error,
    },
    
    #[error("YAML parsing failed")]
    Yaml {
        input: String,
        #[source]
        source: serde_yaml::Error,
    },
    
    #[error("TOML parsing failed")]
    Toml {
        input: String,
        #[source]
        source: toml::de::Error,
    },
    
    #[error("Invalid syntax at line {line}, column {column}")]
    Syntax { 
        line: usize, 
        column: usize,
        message: String,
    },
}
```

## Error Context and Reporting

### Error Context Collection

```rust
impl GroundhogError {
    pub fn with_context<F>(self, f: F) -> Self 
    where 
        F: FnOnce() -> String 
    {
        // Add contextual information to error
    }
    
    pub fn operation_id(&self) -> Option<&str> {
        // Return operation identifier for correlation
    }
    
    pub fn user_message(&self) -> String {
        // Generate user-friendly error message
    }
    
    pub fn suggestions(&self) -> Vec<String> {
        // Provide actionable suggestions
    }
}
```

### Error Reporting Format

#### Console Output
```
error: File not found: 'config.toml'
  │
  ├─ cause: No such file or directory (os error 2)
  │
  └─ help: Try creating a configuration file with:
           groundhog config init
```

#### JSON Format (for tooling)
```json
{
  "error": {
    "type": "FileSystemError",
    "variant": "NotFound",
    "message": "File not found: 'config.toml'",
    "context": {
      "path": "/home/user/.groundhog/config.toml",
      "operation": "config_load",
      "operation_id": "op_12345"
    },
    "source": {
      "type": "IoError",
      "message": "No such file or directory (os error 2)",
      "code": 2
    },
    "suggestions": [
      "Try creating a configuration file with: groundhog config init",
      "Check that the path is correct and accessible"
    ],
    "timestamp": "2024-01-01T12:00:00Z"
  }
}
```

## Error Handling Patterns

### Result Type Usage

```rust
pub type Result<T> = std::result::Result<T, GroundhogError>;

// Function signature pattern
pub fn process_file(path: &Path) -> Result<String> {
    let content = std::fs::read_to_string(path)
        .map_err(|e| FileSystemError::NotReadable { 
            path: path.to_path_buf() 
        })?;
    
    // Process content...
    Ok(content)
}
```

### Error Conversion

```rust
impl From<std::io::Error> for FileSystemError {
    fn from(err: std::io::Error) -> Self {
        match err.kind() {
            std::io::ErrorKind::NotFound => FileSystemError::NotFound { 
                path: PathBuf::new() 
            },
            std::io::ErrorKind::PermissionDenied => FileSystemError::PermissionDenied { 
                path: PathBuf::new() 
            },
            _ => FileSystemError::NotReadable { 
                path: PathBuf::new() 
            },
        }
    }
}
```

### Error Logging Integration

```rust
fn handle_error(error: &GroundhogError) {
    tracing::error!(
        error = %error,
        error.type = ?error.type_name(),
        operation_id = ?error.operation_id(),
        "Operation failed"
    );
}
```

## Exit Codes

### Standard Exit Codes

| Code | Constant | Meaning | Usage |
|------|----------|---------|-------|
| 0 | `EXIT_SUCCESS` | Success | Command completed successfully |
| 1 | `EXIT_FAILURE` | General error | Unexpected errors |
| 2 | `EXIT_USAGE` | Invalid arguments | CLI usage errors |
| 64 | `EXIT_USAGE_ERROR` | Command syntax error | Command-specific usage errors |
| 65 | `EXIT_DATA_ERROR` | Data format error | Invalid input data |
| 66 | `EXIT_NO_INPUT` | Cannot open input | Input file problems |
| 73 | `EXIT_CANT_CREATE` | Cannot create output | Output file problems |
| 74 | `EXIT_IO_ERROR` | I/O error | General I/O problems |

### Exit Code Mapping

```rust
impl From<&GroundhogError> for i32 {
    fn from(error: &GroundhogError) -> Self {
        match error {
            GroundhogError::Command(CommandError::InvalidArguments { .. }) => 64,
            GroundhogError::FileSystem(FileSystemError::NotFound { .. }) => 66,
            GroundhogError::FileSystem(FileSystemError::NotWritable { .. }) => 73,
            GroundhogError::Parse(_) => 65,
            _ => 1,
        }
    }
}
```

## Testing Error Handling

### Unit Tests

```rust
#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_file_not_found_error() {
        let error = FileSystemError::NotFound { 
            path: PathBuf::from("missing.txt") 
        };
        
        assert_eq!(error.to_string(), "File not found: 'missing.txt'");
    }
    
    #[test]
    fn test_error_suggestions() {
        let error = GroundhogError::from(ConfigError::NotFound { 
            path: PathBuf::from("config.toml") 
        });
        
        let suggestions = error.suggestions();
        assert!(!suggestions.is_empty());
        assert!(suggestions[0].contains("config init"));
    }
}
```

### Integration Tests

```rust
#[test]
fn test_command_error_handling() {
    let output = Command::new("groundhog")
        .arg("nonexistent-command")
        .output()
        .expect("Failed to execute command");
    
    assert!(!output.status.success());
    assert_eq!(output.status.code().unwrap(), 64);
    
    let stderr = String::from_utf8(output.stderr).unwrap();
    assert!(stderr.contains("Command 'nonexistent-command' not found"));
}
```

## Recovery Strategies

### Graceful Degradation

```rust
pub fn load_config_with_fallback(path: &Path) -> Config {
    match Config::load(path) {
        Ok(config) => {
            tracing::info!("Configuration loaded successfully");
            config
        }
        Err(ConfigError::NotFound { .. }) => {
            tracing::warn!("Configuration file not found, using defaults");
            Config::default()
        }
        Err(e) => {
            tracing::error!(error = %e, "Failed to load configuration");
            Config::default()
        }
    }
}
```

### Retry Logic

```rust
pub async fn with_retry<F, T, E>(
    operation: F,
    max_attempts: usize,
) -> Result<T> 
where
    F: Fn() -> Result<T, E>,
    E: Into<GroundhogError>,
{
    let mut attempts = 0;
    loop {
        match operation() {
            Ok(result) => return Ok(result),
            Err(e) if attempts < max_attempts => {
                attempts += 1;
                tracing::warn!(
                    attempt = attempts,
                    max_attempts = max_attempts,
                    error = %e.into(),
                    "Operation failed, retrying"
                );
                tokio::time::sleep(Duration::from_secs(1)).await;
            }
            Err(e) => return Err(e.into()),
        }
    }
}
``` 