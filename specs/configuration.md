# Configuration Specification

## Overview

Groundhog uses a hierarchical configuration system that supports multiple sources with clear precedence rules. Configuration can be provided through files, environment variables, and command-line arguments.

## Configuration Sources (by precedence)

1. **Command-line arguments** (highest priority)
2. **Environment variables**
3. **Configuration files**
4. **Default values** (lowest priority)

## Configuration File Locations

Groundhog searches for configuration files in the following order:

1. Path specified by `--config` flag
2. `GROUNDHOG_CONFIG` environment variable
3. `./groundhog.toml` (current directory)
4. `~/.groundhog/config.toml` (user config directory)
5. `/etc/groundhog/config.toml` (system-wide config)

## Configuration Format

### TOML Format (Default)

```toml
# Groundhog Configuration File
# Version: 0.1.0

[logging]
# Log level: trace, debug, info, warn, error
level = "info"
# Output format: pretty, json
format = "pretty"
# Log file path (optional, defaults to stdout)
file = "/var/log/groundhog.log"

[commands]
# Default command if none specified
default = "help"

[commands.explain]
# Command-specific configuration
enabled = true

[ai]
# AI service configuration (future)
provider = "openai"
model = "gpt-4"
# API key (prefer environment variable)
api_key = "${OPENAI_API_KEY}"

[output]
# Default output format
format = "text"
# Enable colored output
color = true
# Pagination settings
pager = "auto"

[performance]
# Maximum file size to process (in MB)
max_file_size = 100
# Timeout for operations (in seconds)
timeout = 30
# Number of worker threads
threads = 4
```

### YAML Format (Alternative)

```yaml
# Groundhog Configuration File
logging:
  level: info
  format: pretty
  file: /var/log/groundhog.log

commands:
  default: help
  explain:
    enabled: true

ai:
  provider: openai
  model: gpt-4
  api_key: ${OPENAI_API_KEY}

output:
  format: text
  color: true
  pager: auto

performance:
  max_file_size: 100
  timeout: 30
  threads: 4
```

## Configuration Schema

### Root Configuration

```rust
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Config {
    pub logging: LoggingConfig,
    pub commands: CommandsConfig,
    pub ai: Option<AiConfig>,
    pub output: OutputConfig,
    pub performance: PerformanceConfig,
}
```

### Logging Configuration

```rust
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LoggingConfig {
    /// Log level filter
    #[serde(default = "default_log_level")]
    pub level: LogLevel,
    
    /// Output format
    #[serde(default = "default_log_format")]
    pub format: LogFormat,
    
    /// Optional log file path
    pub file: Option<PathBuf>,
    
    /// Include timestamps
    #[serde(default = "default_true")]
    pub timestamps: bool,
    
    /// Include thread IDs
    #[serde(default = "default_false")]
    pub thread_ids: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum LogLevel {
    Trace,
    Debug,
    Info,
    Warn,
    Error,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum LogFormat {
    Pretty,
    Json,
    Compact,
}
```

### Commands Configuration

```rust
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CommandsConfig {
    /// Default command when none specified
    pub default: Option<String>,
    
    /// Command-specific configurations
    pub explain: Option<ExplainConfig>,
    pub analyze: Option<AnalyzeConfig>,
    pub generate: Option<GenerateConfig>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExplainConfig {
    /// Whether the command is enabled
    #[serde(default = "default_true")]
    pub enabled: bool,
    
    /// Default output format for this command
    pub format: Option<String>,
}
```

### AI Configuration

```rust
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AiConfig {
    /// AI service provider
    pub provider: AiProvider,
    
    /// Model name
    pub model: String,
    
    /// API key (supports environment variable expansion)
    pub api_key: Option<String>,
    
    /// API endpoint URL
    pub endpoint: Option<String>,
    
    /// Request timeout in seconds
    #[serde(default = "default_ai_timeout")]
    pub timeout: u64,
    
    /// Maximum retries for failed requests
    #[serde(default = "default_retries")]
    pub max_retries: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum AiProvider {
    OpenAI,
    Anthropic,
    Local,
}
```

### Output Configuration

```rust
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OutputConfig {
    /// Default output format
    #[serde(default = "default_output_format")]
    pub format: OutputFormat,
    
    /// Enable colored output
    #[serde(default = "default_true")]
    pub color: bool,
    
    /// Pager configuration
    #[serde(default = "default_pager")]
    pub pager: PagerConfig,
    
    /// Maximum width for formatted output
    pub max_width: Option<usize>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum OutputFormat {
    Text,
    Json,
    Yaml,
    Markdown,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum PagerConfig {
    Auto,
    Always,
    Never,
}
```

### Performance Configuration

```rust
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PerformanceConfig {
    /// Maximum file size to process (MB)
    #[serde(default = "default_max_file_size")]
    pub max_file_size: u64,
    
    /// Default timeout for operations (seconds)
    #[serde(default = "default_timeout")]
    pub timeout: u64,
    
    /// Number of worker threads
    pub threads: Option<usize>,
    
    /// Memory limit (MB)
    pub memory_limit: Option<u64>,
}
```

## Environment Variables

### Configuration Override

| Variable | Description | Example |
|----------|-------------|---------|
| `GROUNDHOG_CONFIG` | Path to configuration file | `/path/to/config.toml` |
| `GROUNDHOG_LOG` | Log level override | `debug` |
| `GROUNDHOG_LOG_FORMAT` | Log format override | `json` |
| `GROUNDHOG_LOG_FILE` | Log file path | `/var/log/groundhog.log` |
| `GROUNDHOG_NO_COLOR` | Disable colored output | `1` |
| `GROUNDHOG_TIMEOUT` | Default timeout (seconds) | `60` |

### AI Service Configuration

| Variable | Description | Example |
|----------|-------------|---------|
| `OPENAI_API_KEY` | OpenAI API key | `sk-...` |
| `ANTHROPIC_API_KEY` | Anthropic API key | `ant-...` |
| `GROUNDHOG_AI_PROVIDER` | AI provider override | `openai` |
| `GROUNDHOG_AI_MODEL` | AI model override | `gpt-4` |

## Configuration Loading

### Loading Process

```rust
impl Config {
    pub fn load() -> Result<Self> {
        let mut config = Config::default();
        
        // 1. Load from configuration files
        if let Some(file_config) = Self::load_from_files()? {
            config = config.merge(file_config);
        }
        
        // 2. Apply environment variable overrides
        config = config.apply_env_overrides()?;
        
        // 3. Validate configuration
        config.validate()?;
        
        Ok(config)
    }
    
    fn load_from_files() -> Result<Option<Self>> {
        for path in Config::search_paths() {
            if path.exists() {
                tracing::debug!(path = %path.display(), "Loading configuration file");
                return Ok(Some(Self::load_from_file(&path)?));
            }
        }
        Ok(None)
    }
    
    fn search_paths() -> Vec<PathBuf> {
        let mut paths = Vec::new();
        
        // Command line specified config
        if let Some(path) = env::var("GROUNDHOG_CONFIG").ok() {
            paths.push(PathBuf::from(path));
        }
        
        // Current directory
        paths.push(PathBuf::from("./groundhog.toml"));
        
        // User config directory
        if let Some(config_dir) = dirs::config_dir() {
            paths.push(config_dir.join("groundhog").join("config.toml"));
        }
        
        // System config
        paths.push(PathBuf::from("/etc/groundhog/config.toml"));
        
        paths
    }
}
```

### Configuration Merging

```rust
impl Config {
    fn merge(mut self, other: Config) -> Self {
        // Merge non-None values from other into self
        if other.logging.level != LogLevel::Info {
            self.logging.level = other.logging.level;
        }
        // ... merge other fields
        self
    }
    
    fn apply_env_overrides(mut self) -> Result<Self> {
        // Apply environment variable overrides
        if let Ok(level) = env::var("GROUNDHOG_LOG") {
            self.logging.level = level.parse()?;
        }
        
        if env::var("GROUNDHOG_NO_COLOR").is_ok() {
            self.output.color = false;
        }
        
        // ... apply other overrides
        Ok(self)
    }
}
```

## Configuration Validation

### Validation Rules

```rust
impl Config {
    fn validate(&self) -> Result<()> {
        // Validate logging configuration
        if let Some(ref file) = self.logging.file {
            if let Some(parent) = file.parent() {
                if !parent.exists() {
                    return Err(ConfigError::InvalidValue {
                        key: "logging.file".to_string(),
                        value: file.display().to_string(),
                        expected: "Parent directory must exist".to_string(),
                    });
                }
            }
        }
        
        // Validate performance limits
        if self.performance.max_file_size == 0 {
            return Err(ConfigError::InvalidValue {
                key: "performance.max_file_size".to_string(),
                value: "0".to_string(),
                expected: "Value must be greater than 0".to_string(),
            });
        }
        
        // Validate AI configuration
        if let Some(ref ai) = self.ai {
            if ai.timeout == 0 {
                return Err(ConfigError::InvalidValue {
                    key: "ai.timeout".to_string(),
                    value: "0".to_string(),
                    expected: "Timeout must be greater than 0".to_string(),
                });
            }
        }
        
        Ok(())
    }
}
```

## Configuration Commands

### Future `config` Command

#### Show Configuration
```bash
groundhog config show
groundhog config show --format json
groundhog config show logging.level
```

#### Set Configuration Values
```bash
groundhog config set logging.level debug
groundhog config set ai.provider openai
groundhog config set output.color false
```

#### Initialize Configuration
```bash
groundhog config init                    # Interactive setup
groundhog config init --defaults         # Use default values
groundhog config init --global           # Create system-wide config
```

#### Validate Configuration
```bash
groundhog config validate
groundhog config validate --file custom-config.toml
```

## Default Values

```rust
impl Default for Config {
    fn default() -> Self {
        Config {
            logging: LoggingConfig {
                level: LogLevel::Info,
                format: LogFormat::Pretty,
                file: None,
                timestamps: true,
                thread_ids: false,
            },
            commands: CommandsConfig {
                default: None,
                explain: Some(ExplainConfig {
                    enabled: true,
                    format: None,
                }),
                analyze: None,
                generate: None,
            },
            ai: None,
            output: OutputConfig {
                format: OutputFormat::Text,
                color: true,
                pager: PagerConfig::Auto,
                max_width: None,
            },
            performance: PerformanceConfig {
                max_file_size: 100,
                timeout: 30,
                threads: None,
                memory_limit: None,
            },
        }
    }
}
```

## Security Considerations

### Sensitive Data Handling

- API keys should be stored in environment variables, not config files
- Config files should have appropriate permissions (600)
- No sensitive data should be logged or traced
- Support for encrypted configuration sections (future feature)

### File Permissions

```rust
fn create_config_file(path: &Path, config: &Config) -> Result<()> {
    let content = toml::to_string_pretty(config)?;
    
    // Create file with restricted permissions
    let mut file = OpenOptions::new()
        .create(true)
        .write(true)
        .truncate(true)
        .mode(0o600)  // User read/write only
        .open(path)?;
    
    file.write_all(content.as_bytes())?;
    Ok(())
}
``` 