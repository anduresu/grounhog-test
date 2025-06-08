use serde::{Deserialize, Serialize};

/// Represents a command execution context
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CommandContext {
    pub command_name: String,
    pub start_time: std::time::SystemTime,
    pub user_input: Option<String>,
}

impl CommandContext {
    pub fn new(command_name: impl Into<String>) -> Self {
        Self {
            command_name: command_name.into(),
            start_time: std::time::SystemTime::now(),
            user_input: None,
        }
    }
    
    pub fn with_input(mut self, input: impl Into<String>) -> Self {
        self.user_input = Some(input.into());
        self
    }
    
    /// Get the elapsed time since the command started
    pub fn elapsed(&self) -> std::time::Duration {
        self.start_time.elapsed().unwrap_or_default()
    }
}

/// Represents the result of a command execution
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CommandResult {
    pub success: bool,
    pub message: Option<String>,
    pub duration_ms: u64,
}

impl CommandResult {
    pub fn success() -> Self {
        Self {
            success: true,
            message: None,
            duration_ms: 0,
        }
    }
    
    pub fn success_with_message(message: impl Into<String>) -> Self {
        Self {
            success: true,
            message: Some(message.into()),
            duration_ms: 0,
        }
    }
    
    pub fn failure(message: impl Into<String>) -> Self {
        Self {
            success: false,
            message: Some(message.into()),
            duration_ms: 0,
        }
    }
    
    pub fn with_duration(mut self, duration: std::time::Duration) -> Self {
        self.duration_ms = duration.as_millis() as u64;
        self
    }
    
    /// Check if the command was successful
    pub fn is_success(&self) -> bool {
        self.success
    }
    
    /// Check if the command failed
    pub fn is_failure(&self) -> bool {
        !self.success
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::time::Duration;

    #[test]
    fn test_command_context_new() {
        let context = CommandContext::new("explain");
        assert_eq!(context.command_name, "explain");
        assert!(context.user_input.is_none());
        assert!(context.start_time <= std::time::SystemTime::now());
    }

    #[test]
    fn test_command_context_with_input() {
        let context = CommandContext::new("explain")
            .with_input("test input");
        
        assert_eq!(context.command_name, "explain");
        assert_eq!(context.user_input, Some("test input".to_string()));
    }

    #[test]
    fn test_command_context_elapsed() {
        let context = CommandContext::new("explain");
        std::thread::sleep(Duration::from_millis(1));
        let elapsed = context.elapsed();
        assert!(elapsed.as_millis() > 0);
    }

    #[test]
    fn test_command_result_success() {
        let result = CommandResult::success();
        assert!(result.is_success());
        assert!(!result.is_failure());
        assert!(result.message.is_none());
        assert_eq!(result.duration_ms, 0);
    }

    #[test]
    fn test_command_result_success_with_message() {
        let result = CommandResult::success_with_message("Operation completed");
        assert!(result.is_success());
        assert_eq!(result.message, Some("Operation completed".to_string()));
    }

    #[test]
    fn test_command_result_failure() {
        let result = CommandResult::failure("Something went wrong");
        assert!(result.is_failure());
        assert!(!result.is_success());
        assert_eq!(result.message, Some("Something went wrong".to_string()));
    }

    #[test]
    fn test_command_result_with_duration() {
        let duration = Duration::from_millis(150);
        let result = CommandResult::success().with_duration(duration);
        assert_eq!(result.duration_ms, 150);
    }

    #[test]
    fn test_command_context_serialization() {
        let context = CommandContext::new("explain")
            .with_input("test input");
        
        let serialized = serde_json::to_string(&context).unwrap();
        let deserialized: CommandContext = serde_json::from_str(&serialized).unwrap();
        
        assert_eq!(context.command_name, deserialized.command_name);
        assert_eq!(context.user_input, deserialized.user_input);
    }

    #[test]
    fn test_command_result_serialization() {
        let result = CommandResult::success_with_message("test")
            .with_duration(Duration::from_millis(100));
        
        let serialized = serde_json::to_string(&result).unwrap();
        let deserialized: CommandResult = serde_json::from_str(&serialized).unwrap();
        
        assert_eq!(result.success, deserialized.success);
        assert_eq!(result.message, deserialized.message);
        assert_eq!(result.duration_ms, deserialized.duration_ms);
    }
} 