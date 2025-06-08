# Groundhog AI Coding Assistant

A command-line AI coding assistant built in Rust, designed to help developers with coding tasks, analysis, and assistance.

Following the steps done in https://ghuntley.com/specs/
Overall it does work but it is really slow. Rust carries a lot of weight, requires tests for python and speed improvement.

Most issues are regarding context-window management from cursor per task done. And overall speed could be improved by doing a task dependency graph with BFS completion.

## Features

- **Command-line Interface**: Clean, intuitive CLI built with clap
- **Structured Logging**: Comprehensive observability with tracing
- **Configuration Management**: Hierarchical configuration system
- **Error Handling**: User-friendly error messages with actionable suggestions
- **Extensible Architecture**: Modular design for easy feature additions

## Installation

### From Source

```bash
git clone <repository-url>
cd groundhog
cargo build --release
cargo install --path .
```

## Usage

### Basic Commands

```bash
# Show help
groundhog --help

# Run explain command
groundhog explain

# Run with verbose logging
groundhog -v explain

# Run with custom configuration
groundhog --config ./my-config.toml explain
```

### Logging Levels

- No flags: `WARN` level
- `-v`: `INFO` level  
- `-vv`: `DEBUG` level
- `-vvv`: `TRACE` level
- `-q`: Quiet mode (errors only)

## Configuration

Groundhog uses a hierarchical configuration system. Configuration files are searched in this order:

1. Path specified by `--config` flag
2. `GROUNDHOG_CONFIG` environment variable
3. `./groundhog.toml` (current directory)
4. `~/.groundhog/config.toml` (user config directory)
5. `/etc/groundhog/config.toml` (system-wide config)

### Example Configuration

See `examples/groundhog.toml` for a complete example configuration file.

```toml
[logging]
level = "Info"
format = "Pretty"

[commands.explain]
enabled = true

[output]
format = "text"
color = true

[performance]
max_file_size = 100
timeout = 30
threads = 4
```

## Development

### Prerequisites

- Rust 1.87.0 or later
- Git

### Setup

```bash
# Clone and build
git clone <repository-url>
cd groundhog
cargo build

# Run tests
cargo test

# Run benchmarks
cargo bench

# Run with development logging
RUST_LOG=debug cargo run -- -v explain
```

### Project Structure

```
src/
├── main.rs              # Application entry point
├── lib.rs               # Library exports
├── cli/                 # CLI parsing and commands
│   ├── args.rs          # Argument definitions
│   └── commands/        # Command implementations
├── core/                # Core business logic
│   ├── models.rs        # Data models
│   └── services.rs      # Application services
└── infrastructure/      # Cross-cutting concerns
    ├── config.rs        # Configuration management
    ├── error.rs         # Error handling
    └── logging.rs       # Tracing setup
```

### Testing

```bash
# Run all tests
cargo test

# Run with output
cargo test -- --nocapture

# Run integration tests only
cargo test --test integration_tests

# Run benchmarks
cargo bench
```

## Architecture

Groundhog follows clean architecture principles with clear separation of concerns:

- **CLI Layer**: Handles user interface and argument parsing
- **Core Layer**: Contains business logic and domain models
- **Infrastructure Layer**: Manages cross-cutting concerns like logging, configuration, and error handling

## Contributing

1. Fork the repository
2. Create a feature branch (`git checkout -b feature/amazing-feature`)
3. Commit your changes (`git commit -m 'Add amazing feature'`)
4. Push to the branch (`git push origin feature/amazing-feature`)
5. Open a Pull Request

### Commit Convention

We follow [Conventional Commits](https://www.conventionalcommits.org/):

- `feat:` New features
- `fix:` Bug fixes
- `docs:` Documentation changes
- `test:` Test additions/changes
- `refactor:` Code refactoring

## License

This project is licensed under the MIT License - see the LICENSE file for details.

## Roadmap

- [ ] AI service integration
- [ ] Code analysis commands
- [ ] Code generation features
- [ ] Plugin system
- [ ] Web interface
- [ ] IDE integrations

## Version

Current version: 0.1.0 