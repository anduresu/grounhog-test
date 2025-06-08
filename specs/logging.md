# Logging & Observability Specification

## Overview

Groundhog implements comprehensive observability using the `tracing` ecosystem to provide logging, metrics, and distributed tracing capabilities. All operations are instrumented to enable effective troubleshooting and performance monitoring.

## Tracing Strategy

### Instrumentation Principles

1. **Comprehensive Coverage**: All public functions and critical code paths are instrumented
2. **Structured Logging**: Use structured fields rather than formatted strings
3. **Contextual Information**: Include relevant context in spans and events
4. **Performance Awareness**: Minimize overhead in hot paths
5. **User Privacy**: Never log sensitive user data

### Tracing Hierarchy

```
┌─────────────────────────────────────────┐
│ Application Span (groundhog)            │
├─────────────────────────────────────────┤
│ Command Span (explain, analyze, etc.)   │
├─────────────────────────────────────────┤
│ Operation Spans (file_read, ai_call)    │
├─────────────────────────────────────────┤
│ Function Spans (parse, validate, etc.)  │
└─────────────────────────────────────────┘
```

## Log Levels

### ERROR
- Application crashes
- Unrecoverable errors
- Data corruption
- External service failures

**Example**:
```rust
tracing::error!(
    error = %e,
    command = "explain",
    "Failed to execute command"
);
```

### WARN
- Recoverable errors
- Configuration issues
- Performance degradation
- Deprecated feature usage

**Example**:
```rust
tracing::warn!(
    path = %config_path,
    "Configuration file not found, using defaults"
);
```

### INFO
- Command execution start/end
- Major operation milestones
- Configuration loading
- Feature usage statistics

**Example**:
```rust
tracing::info!(
    command = "explain",
    duration_ms = duration.as_millis(),
    "Command completed successfully"
);
```

### DEBUG
- Function entry/exit
- Intermediate processing steps
- Configuration values
- Algorithm decisions

**Example**:
```rust
tracing::debug!(
    input_size = input.len(),
    "Processing user input"
);
```

### TRACE
- Detailed execution flow
- Variable values
- Loop iterations
- External API calls

**Example**:
```rust
tracing::trace!(
    iteration = i,
    current_value = %value,
    "Processing item in loop"
);
```

## Span Structure

### Command Spans
Every command execution is wrapped in a span:

```rust
#[tracing::instrument(
    name = "command.explain",
    fields(
        command = "explain",
        user_id = tracing::field::Empty,
        duration_ms = tracing::field::Empty,
    )
)]
async fn execute_explain() -> Result<()> {
    // Command implementation
}
```

### Operation Spans
Individual operations within commands:

```rust
#[tracing::instrument(
    name = "file.read",
    fields(
        path = %file_path,
        size_bytes = tracing::field::Empty,
    ),
    skip(content)
)]
fn read_file(file_path: &Path) -> Result<String> {
    // File reading implementation
}
```

## Structured Fields

### Standard Fields

| Field | Type | Description | Example |
|-------|------|-------------|---------|
| `command` | String | Command being executed | `"explain"` |
| `operation` | String | Operation type | `"file_read"` |
| `duration_ms` | u64 | Operation duration | `42` |
| `error` | String | Error message | `"File not found"` |
| `user_id` | String | User identifier (if applicable) | `"user123"` |
| `file_path` | String | File path being processed | `"/path/to/file.rs"` |
| `file_size` | u64 | File size in bytes | `1024` |
| `line_count` | u32 | Number of lines processed | `150` |

### Dynamic Fields
Fields that can be set during span execution:

```rust
let span = tracing::info_span!("process_file", path = %file_path);
let _guard = span.enter();

// Set field after calculating value
span.record("size_bytes", file.len());
```

## Subscriber Configuration

### Development Environment
```rust
tracing_subscriber::fmt()
    .with_env_filter("groundhog=debug,warn")
    .with_target(false)
    .with_thread_ids(true)
    .with_file(true)
    .with_line_number(true)
    .pretty()
    .init();
```

### Production Environment
```rust
tracing_subscriber::fmt()
    .with_env_filter("groundhog=info,warn")
    .with_target(false)
    .json()
    .init();
```

## Performance Monitoring

### Metrics Collection
Key performance indicators tracked:

- Command execution duration
- File processing throughput
- Memory usage patterns
- Error rates by command
- User adoption metrics

### Custom Metrics
```rust
use tracing::{info, Instrument};

async fn timed_operation() {
    let start = std::time::Instant::now();
    
    // Perform operation
    let result = perform_work().await;
    
    let duration = start.elapsed();
    info!(
        operation = "work",
        duration_ms = duration.as_millis(),
        success = result.is_ok(),
        "Operation completed"
    );
}
```

## Error Correlation

### Error Context
All errors include correlation IDs and context:

```rust
#[derive(Debug, thiserror::Error)]
pub enum GroundhogError {
    #[error("File operation failed")]
    FileError {
        path: PathBuf,
        operation: String,
        #[source]
        source: std::io::Error,
    },
}
```

### Error Logging
```rust
match file_operation() {
    Ok(result) => {
        tracing::info!("File operation succeeded");
        result
    }
    Err(e) => {
        tracing::error!(
            error = %e,
            error.source = ?e.source(),
            "File operation failed"
        );
        return Err(e);
    }
}
```

## Environment Configuration

### Environment Variables

| Variable | Description | Default |
|----------|-------------|---------|
| `GROUNDHOG_LOG` | Log level filter | `warn` |
| `GROUNDHOG_LOG_FORMAT` | Output format (pretty/json) | `pretty` |
| `GROUNDHOG_LOG_FILE` | Log file path | None (stdout) |
| `RUST_LOG` | Rust ecosystem log filter | Falls back to `GROUNDHOG_LOG` |

### Configuration Examples

```bash
# Debug level for groundhog, info for dependencies
export GROUNDHOG_LOG="groundhog=debug,info"

# JSON format for production
export GROUNDHOG_LOG_FORMAT=json

# Log to file
export GROUNDHOG_LOG_FILE="/var/log/groundhog.log"
```

## Integration with External Systems

### Future Integrations
- OpenTelemetry for distributed tracing
- Prometheus metrics export
- Jaeger for trace visualization
- Log aggregation systems (ELK stack)

### Trace Export Format
When enabled, traces will be exported in OpenTelemetry format for external analysis tools. 