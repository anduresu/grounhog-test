use std::path::PathBuf;
use thiserror::Error;

/// Main error type for the Groundhog application
#[derive(Debug, Error)]
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
    
    #[error("TUI error: {0}")]
    TUIError(String),
}

#[derive(Debug, Error)]
pub enum CommandError {
    #[error("Command '{command}' not found")]
    NotFound { command: String },
    
    #[error("Invalid arguments for command '{command}': {message}")]
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

#[derive(Debug, Error)]
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

#[derive(Debug, Error)]
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
    
    #[error("I/O error")]
    Io(#[from] std::io::Error),
}

#[derive(Debug, Error)]
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

#[derive(Debug, Error)]
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
        source: Box<dyn std::error::Error + Send + Sync>,
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

#[derive(Debug, Error)]
pub enum InternalError {
    #[error("Unexpected application state: {message}")]
    UnexpectedState { message: String },
    
    #[error("Resource exhausted: {resource}")]
    ResourceExhausted { resource: String },
    
    #[error("Initialization failed: {component}")]
    InitializationFailed { 
        component: String,
        #[source]
        source: Box<dyn std::error::Error + Send + Sync>,
    },
    
    #[error("Configuration validation failed: {message}")]
    ConfigValidation { message: String },
}

impl GroundhogError {
    /// Returns a user-friendly error message with suggestions
    pub fn user_message(&self) -> String {
        match self {
            GroundhogError::Command(CommandError::NotFound { command }) => {
                format!("Command '{}' not found. Run 'groundhog --help' to see available commands.", command)
            }
            GroundhogError::Config(ConfigError::NotFound { path }) => {
                format!(
                    "Configuration file not found at '{}'.\nTry running 'groundhog config init' to create a default configuration.",
                    path.display()
                )
            }
            GroundhogError::FileSystem(FileSystemError::NotFound { path }) => {
                format!(
                    "File not found: '{}'\nPlease check the path and try again.",
                    path.display()
                )
            }
            GroundhogError::FileSystem(FileSystemError::PermissionDenied { path }) => {
                format!(
                    "Permission denied accessing '{}'\nPlease check file permissions or run with appropriate privileges.",
                    path.display()
                )
            }
            _ => self.to_string(),
        }
    }
    
    /// Returns the exit code that should be used for this error
    pub fn exit_code(&self) -> i32 {
        match self {
            GroundhogError::Command(CommandError::NotFound { .. }) => 64, // EX_USAGE
            GroundhogError::Command(CommandError::InvalidArguments { .. }) => 64, // EX_USAGE
            GroundhogError::Config(ConfigError::InvalidFormat { .. }) => 65, // EX_DATAERR
            GroundhogError::FileSystem(FileSystemError::NotFound { .. }) => 66, // EX_NOINPUT
            GroundhogError::FileSystem(FileSystemError::PermissionDenied { .. }) => 77, // EX_NOPERM
            GroundhogError::FileSystem(FileSystemError::Io(_)) => 74, // EX_IOERR
            GroundhogError::Network(_) => 69, // EX_UNAVAILABLE
            _ => 1, // General error
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::PathBuf;

    #[test]
    fn test_command_error_not_found() {
        let error = CommandError::NotFound {
            command: "invalid".to_string(),
        };
        assert!(error.to_string().contains("invalid"));
        assert!(error.to_string().contains("not found"));
    }

    #[test]
    fn test_command_error_invalid_arguments() {
        let error = CommandError::InvalidArguments {
            command: "explain".to_string(),
            message: "missing required argument".to_string(),
        };
        assert!(error.to_string().contains("explain"));
        assert!(error.to_string().contains("Invalid arguments"));
    }

    #[test]
    fn test_config_error_not_found() {
        let error = ConfigError::NotFound {
            path: PathBuf::from("/nonexistent/config.toml"),
        };
        assert!(error.to_string().contains("not found"));
        assert!(error.to_string().contains("config.toml"));
    }

    #[test]
    fn test_config_error_invalid_format() {
        let error = ConfigError::InvalidFormat {
            path: PathBuf::from("config.toml"),
            line: Some(5),
            source: Box::new(std::io::Error::new(std::io::ErrorKind::InvalidData, "test error")),
        };
        assert!(error.to_string().contains("invalid format"));
    }

    #[test]
    fn test_filesystem_error_not_found() {
        let error = FileSystemError::NotFound {
            path: PathBuf::from("/nonexistent/file.txt"),
        };
        assert!(error.to_string().contains("File not found"));
        assert!(error.to_string().contains("file.txt"));
    }

    #[test]
    fn test_filesystem_error_permission_denied() {
        let error = FileSystemError::PermissionDenied {
            path: PathBuf::from("/root/secret.txt"),
        };
        assert!(error.to_string().contains("Permission denied"));
        assert!(error.to_string().contains("secret.txt"));
    }

    #[test]
    fn test_network_error_connection_failed() {
        let error = NetworkError::ConnectionFailed {
            url: "https://api.example.com".to_string(),
            source: Box::new(std::io::Error::new(std::io::ErrorKind::ConnectionRefused, "connection refused")),
        };
        assert!(error.to_string().contains("Failed to connect"));
        assert!(error.to_string().contains("api.example.com"));
    }

    #[test]
    fn test_network_error_timeout() {
        let error = NetworkError::Timeout { timeout_ms: 5000 };
        assert!(error.to_string().contains("timeout"));
        assert!(error.to_string().contains("5000"));
    }

    #[test]
    fn test_parse_error_json() {
        let json_error = serde_json::from_str::<serde_json::Value>("invalid json").unwrap_err();
        let error = ParseError::Json {
            input: "invalid json".to_string(),
            line: Some(1),
            column: Some(1),
            source: json_error,
        };
        assert!(error.to_string().contains("JSON parsing failed"));
    }

    #[test]
    fn test_parse_error_toml() {
        let toml_error = toml::from_str::<toml::Value>("invalid = toml = syntax").unwrap_err();
        let error = ParseError::Toml {
            input: "invalid = toml = syntax".to_string(),
            source: toml_error,
        };
        assert!(error.to_string().contains("TOML parsing failed"));
    }

    #[test]
    fn test_internal_error_unexpected_state() {
        let error = InternalError::UnexpectedState {
            message: "Invalid state transition".to_string(),
        };
        assert!(error.to_string().contains("Unexpected application state"));
        assert!(error.to_string().contains("Invalid state transition"));
    }

    #[test]
    fn test_groundhog_error_user_message() {
        let error = GroundhogError::Command(CommandError::NotFound {
            command: "invalid".to_string(),
        });
        let user_msg = error.user_message();
        assert!(user_msg.contains("Command 'invalid' not found"));
        assert!(user_msg.contains("groundhog --help"));
    }

    #[test]
    fn test_groundhog_error_exit_codes() {
        let command_error = GroundhogError::Command(CommandError::NotFound {
            command: "invalid".to_string(),
        });
        assert_eq!(command_error.exit_code(), 64);

        let config_error = GroundhogError::Config(ConfigError::InvalidFormat {
            path: PathBuf::from("config.toml"),
            line: None,
            source: Box::new(std::io::Error::new(std::io::ErrorKind::InvalidData, "test")),
        });
        assert_eq!(config_error.exit_code(), 65);

        let fs_error = GroundhogError::FileSystem(FileSystemError::NotFound {
            path: PathBuf::from("file.txt"),
        });
        assert_eq!(fs_error.exit_code(), 66);

        let network_error = GroundhogError::Network(NetworkError::Timeout { timeout_ms: 1000 });
        assert_eq!(network_error.exit_code(), 69);
    }

    #[test]
    fn test_error_chain_conversion() {
        let io_error = std::io::Error::new(std::io::ErrorKind::NotFound, "file not found");
        let fs_error = FileSystemError::Io(io_error);
        let groundhog_error = GroundhogError::FileSystem(fs_error);
        
        assert!(groundhog_error.to_string().contains("File system operation failed"));
    }
} 