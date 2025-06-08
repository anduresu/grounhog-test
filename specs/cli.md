# Command Line Interface Specification

## Overview

Groundhog provides a command-line interface for AI-powered coding assistance. The CLI is built using the `clap` crate with derive macros for clean, maintainable argument parsing.

## Global Structure

```
groundhog [GLOBAL_OPTIONS] <COMMAND> [COMMAND_OPTIONS] [ARGS...]
```

### Global Options

| Option | Short | Long | Description | Default |
|--------|-------|------|-------------|---------|
| Verbosity | `-v` | `--verbose` | Increase logging verbosity (can be repeated) | `warn` |
| Quiet | `-q` | `--quiet` | Suppress non-error output | `false` |
| Config | `-c` | `--config` | Path to configuration file | `~/.groundhog/config.toml` |
| Help | `-h` | `--help` | Show help information | - |
| Version | `-V` | `--version` | Show version information | - |

### Logging Levels (by verbosity)

- No flags: `WARN` level
- `-v`: `INFO` level  
- `-vv`: `DEBUG` level
- `-vvv`: `TRACE` level

## Commands

### `explain` Command

**Status**: ✅ Implemented (v0.1.0)

**Purpose**: Provides explanations and demonstrations.

**Usage**:
```bash
groundhog explain [OPTIONS]
```

**Options**:
Currently takes no additional options.

**Behavior**:
- Prints "hello world" to stdout
- Logs the operation with appropriate tracing
- Returns exit code 0 on success

**Examples**:
```bash
# Basic usage
$ groundhog explain
hello world

# With verbose logging
$ groundhog -v explain
[INFO] Starting explain command
hello world
[INFO] Explain command completed successfully
```

### Future Commands (Planned)

#### `analyze` Command
```bash
groundhog analyze [FILE_OR_DIRECTORY] [OPTIONS]
```
Analyze code for patterns, issues, or improvements.

#### `generate` Command
```bash
groundhog generate [TEMPLATE] [OPTIONS]
```
Generate code based on templates or descriptions.

#### `review` Command
```bash
groundhog review [FILE_OR_DIRECTORY] [OPTIONS]
```
Perform code review and suggest improvements.

## Exit Codes

| Code | Meaning |
|------|---------|
| `0` | Success |
| `1` | General error |
| `2` | Invalid arguments |
| `64` | Usage error (incorrect command syntax) |
| `65` | Data format error |
| `66` | Cannot open input file |
| `73` | Cannot create output file |
| `74` | I/O error |

## Error Output

Errors are written to stderr with structured format:
```
error: <brief description>
  cause: <detailed explanation>
  help: <suggested solution>
```

## Environment Variables

| Variable | Description | Default |
|----------|-------------|---------|
| `GROUNDHOG_LOG` | Set log level (error, warn, info, debug, trace) | `warn` |
| `GROUNDHOG_CONFIG` | Path to configuration file | `~/.groundhog/config.toml` |
| `GROUNDHOG_NO_COLOR` | Disable colored output | `false` |

## Configuration Integration

The CLI respects configuration files for default values. Command-line arguments override configuration file settings.

## Shell Completion

Future versions will support shell completion for:
- bash
- zsh  
- fish
- PowerShell

## Examples

### Basic Usage
```bash
# Show version
groundhog --version

# Get help
groundhog --help
groundhog explain --help

# Run with different log levels
groundhog -v explain          # INFO level
groundhog -vv explain         # DEBUG level
groundhog -q explain          # Quiet mode
```

### Advanced Usage
```bash
# Use custom config file
groundhog --config ./my-config.toml explain

# Combine global and command options
groundhog --verbose --config ./dev-config.toml explain
``` 