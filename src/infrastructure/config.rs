use serde::{Deserialize, Serialize};
use std::path::{Path, PathBuf};
use tracing::{debug, info, instrument, warn};

use crate::infrastructure::error::{ConfigError, GroundhogError};

/// Main configuration structure
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct Config {
    #[serde(default)]
    pub logging: LoggingConfig,
    
    #[serde(default)]
    pub commands: CommandsConfig,
    
    pub ai: Option<AiConfig>,
    
    #[serde(default)]
    pub output: OutputConfig,
    
    #[serde(default)]
    pub performance: PerformanceConfig,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LoggingConfig {
    #[serde(default = "default_log_level")]
    pub level: LogLevel,
    
    #[serde(default = "default_log_format")]
    pub format: LogFormat,
    
    pub file: Option<PathBuf>,
    
    #[serde(default = "default_true")]
    pub timestamps: bool,
    
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

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CommandsConfig {
    pub default: Option<String>,
    pub explain: Option<ExplainConfig>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExplainConfig {
    #[serde(default = "default_true")]
    pub enabled: bool,
    
    pub format: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AiConfig {
    pub provider: AiProvider,
    pub model: String,
    pub api_key: Option<String>,
    pub endpoint: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum AiProvider {
    OpenAI,
    Anthropic,
    Local,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OutputConfig {
    #[serde(default = "default_output_format")]
    pub format: String,
    
    #[serde(default = "default_true")]
    pub color: bool,
    
    #[serde(default = "default_pager")]
    pub pager: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PerformanceConfig {
    #[serde(default = "default_max_file_size")]
    pub max_file_size: u64,
    
    #[serde(default = "default_timeout")]
    pub timeout: u64,
    
    #[serde(default = "default_threads")]
    pub threads: usize,
}

// Default value functions
fn default_log_level() -> LogLevel { LogLevel::Info }
fn default_log_format() -> LogFormat { LogFormat::Pretty }
fn default_output_format() -> String { "text".to_string() }
fn default_pager() -> String { "auto".to_string() }
fn default_max_file_size() -> u64 { 100 }
fn default_timeout() -> u64 { 30 }
fn default_threads() -> usize { 4 }
fn default_true() -> bool { true }
fn default_false() -> bool { false }



impl Default for LoggingConfig {
    fn default() -> Self {
        Self {
            level: default_log_level(),
            format: default_log_format(),
            file: None,
            timestamps: default_true(),
            thread_ids: default_false(),
        }
    }
}

impl Default for CommandsConfig {
    fn default() -> Self {
        Self {
            default: None,
            explain: Some(ExplainConfig::default()),
        }
    }
}

impl Default for ExplainConfig {
    fn default() -> Self {
        Self {
            enabled: default_true(),
            format: None,
        }
    }
}

impl Default for OutputConfig {
    fn default() -> Self {
        Self {
            format: default_output_format(),
            color: default_true(),
            pager: default_pager(),
        }
    }
}

impl Default for PerformanceConfig {
    fn default() -> Self {
        Self {
            max_file_size: default_max_file_size(),
            timeout: default_timeout(),
            threads: default_threads(),
        }
    }
}

impl Config {
    /// Load configuration from file with fallback to defaults
    #[instrument(name = "config.load", fields(path = %path.as_ref().display()))]
    pub fn load_from_file<P: AsRef<Path>>(path: P) -> Result<Self, GroundhogError> {
        let path = path.as_ref();
        
        debug!("Loading configuration from file");
        
        if !path.exists() {
            warn!("Configuration file not found, using defaults");
            return Ok(Self::default());
        }
        
        let content = std::fs::read_to_string(path)
            .map_err(|_e| ConfigError::NotFound { path: path.to_path_buf() })?;
        
        let config: Config = toml::from_str(&content)
            .map_err(|e| ConfigError::InvalidFormat {
                path: path.to_path_buf(),
                line: None, // TOML errors don't provide line_col in this version
                source: Box::new(e),
            })?;
        
        info!("Configuration loaded successfully");
        Ok(config)
    }
    
    /// Load configuration with hierarchical search
    #[instrument(name = "config.load_hierarchical")]
    pub fn load_hierarchical(config_path: Option<PathBuf>) -> Result<Self, GroundhogError> {
        info!("Loading configuration with hierarchical search");
        
        let search_paths = Self::get_config_search_paths(config_path);
        
        for path in search_paths {
            debug!(path = %path.display(), "Checking configuration path");
            
            if path.exists() {
                info!(path = %path.display(), "Found configuration file");
                return Self::load_from_file(&path);
            }
        }
        
        info!("No configuration file found, using defaults");
        Ok(Self::default())
    }
    
    /// Get configuration file search paths in order of precedence
    fn get_config_search_paths(explicit_path: Option<PathBuf>) -> Vec<PathBuf> {
        let mut paths = Vec::new();
        
        // 1. Explicit path from command line
        if let Some(path) = explicit_path {
            paths.push(path);
        }
        
        // 2. Environment variable
        if let Ok(env_path) = std::env::var("GROUNDHOG_CONFIG") {
            paths.push(PathBuf::from(env_path));
        }
        
        // 3. Current directory
        paths.push(PathBuf::from("./groundhog.toml"));
        
        // 4. User config directory
        if let Some(home) = dirs::home_dir() {
            paths.push(home.join(".groundhog").join("config.toml"));
        }
        
        // 5. System-wide config
        paths.push(PathBuf::from("/etc/groundhog/config.toml"));
        
        paths
    }
    
    /// Validate configuration values
    #[instrument(name = "config.validate")]
    pub fn validate(&self) -> Result<(), GroundhogError> {
        debug!("Validating configuration");
        
        // Validate performance settings
        if self.performance.max_file_size == 0 {
            return Err(ConfigError::InvalidValue {
                key: "performance.max_file_size".to_string(),
                value: "0".to_string(),
                expected: "positive integer".to_string(),
            }.into());
        }
        
        if self.performance.timeout == 0 {
            return Err(ConfigError::InvalidValue {
                key: "performance.timeout".to_string(),
                value: "0".to_string(),
                expected: "positive integer".to_string(),
            }.into());
        }
        
        if self.performance.threads == 0 {
            return Err(ConfigError::InvalidValue {
                key: "performance.threads".to_string(),
                value: "0".to_string(),
                expected: "positive integer".to_string(),
            }.into());
        }
        
        info!("Configuration validation passed");
        Ok(())
    }
    
    /// Create a default configuration file
    #[instrument(name = "config.create_default", fields(path = %path.as_ref().display()))]
    pub fn create_default_file<P: AsRef<Path>>(path: P) -> Result<(), GroundhogError> {
        let path = path.as_ref();
        
        info!("Creating default configuration file");
        
        // Create parent directory if it doesn't exist
        if let Some(parent) = path.parent() {
            std::fs::create_dir_all(parent)
                .map_err(|e| ConfigError::InvalidFormat {
                    path: path.to_path_buf(),
                    line: None,
                    source: Box::new(e),
                })?;
        }
        
        let default_config = Self::default();
        let toml_content = toml::to_string_pretty(&default_config)
            .map_err(|e| ConfigError::InvalidFormat {
                path: path.to_path_buf(),
                line: None,
                source: Box::new(e),
            })?;
        
        let content = format!(
            "# Groundhog Configuration File\n# Version: 0.1.0\n\n{}",
            toml_content
        );
        
        std::fs::write(path, content)
            .map_err(|e| ConfigError::InvalidFormat {
                path: path.to_path_buf(),
                line: None,
                source: Box::new(e),
            })?;
        
        info!("Default configuration file created successfully");
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::TempDir;
    
    #[test]
    fn test_default_config() {
        let config = Config::default();
        assert!(config.validate().is_ok());
    }
    
    #[test]
    fn test_config_validation() {
        let mut config = Config::default();
        
        // Test invalid max_file_size
        config.performance.max_file_size = 0;
        assert!(config.validate().is_err());
        
        // Reset and test invalid timeout
        config = Config::default();
        config.performance.timeout = 0;
        assert!(config.validate().is_err());
        
        // Reset and test invalid threads
        config = Config::default();
        config.performance.threads = 0;
        assert!(config.validate().is_err());
    }
    
    #[test]
    fn test_load_nonexistent_file() {
        let result = Config::load_from_file("nonexistent.toml");
        assert!(result.is_ok()); // Should return default config
    }
    
    #[test]
    fn test_create_and_load_config_file() {
        let temp_dir = TempDir::new().unwrap();
        let config_path = temp_dir.path().join("test_config.toml");
        
        // Create default config file
        Config::create_default_file(&config_path).unwrap();
        
        // Load the created file
        let loaded_config = Config::load_from_file(&config_path).unwrap();
        
        // Validate it matches defaults
        let default_config = Config::default();
        assert_eq!(loaded_config.logging.level as u8, default_config.logging.level as u8);
        assert_eq!(loaded_config.performance.max_file_size, default_config.performance.max_file_size);
    }
    
    #[test]
    fn test_invalid_toml_format() {
        let temp_dir = TempDir::new().unwrap();
        let config_path = temp_dir.path().join("invalid.toml");
        
        // Write invalid TOML
        std::fs::write(&config_path, "invalid toml content [[[").unwrap();
        
        let result = Config::load_from_file(&config_path);
        assert!(result.is_err());
        
        if let Err(GroundhogError::Config(ConfigError::InvalidFormat { .. })) = result {
            // Expected error type
        } else {
            panic!("Expected ConfigError::InvalidFormat");
        }
    }
} 