// Future: AI service integration, file processing services, etc.
// This module will contain the core business logic services

use crate::infrastructure::error::GroundhogError;

/// Placeholder for future AI service integration
pub struct AIService {
    // Future: API client, model configuration, etc.
    pub enabled: bool,
}

impl AIService {
    pub fn new() -> Self {
        Self { enabled: false }
    }
    
    pub fn with_enabled(mut self, enabled: bool) -> Self {
        self.enabled = enabled;
        self
    }
    
    /// Future: Generate explanations using AI
    pub async fn generate_explanation(&self, topic: &str) -> Result<String, GroundhogError> {
        if !self.enabled {
            return Ok(format!("AI service is disabled. Topic: {}", topic));
        }
        
        // Placeholder implementation
        Ok(format!("AI-generated explanation for '{}' (not implemented yet)", topic))
    }
    
    /// Check if the AI service is available
    pub fn is_available(&self) -> bool {
        self.enabled
    }
}

impl Default for AIService {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ai_service_new() {
        let service = AIService::new();
        assert!(!service.enabled);
        assert!(!service.is_available());
    }

    #[test]
    fn test_ai_service_with_enabled() {
        let service = AIService::new().with_enabled(true);
        assert!(service.enabled);
        assert!(service.is_available());
    }

    #[test]
    fn test_ai_service_default() {
        let service = AIService::default();
        assert!(!service.enabled);
    }

    #[tokio::test]
    async fn test_generate_explanation_disabled() {
        let service = AIService::new();
        let result = service.generate_explanation("rust").await.unwrap();
        assert!(result.contains("AI service is disabled"));
        assert!(result.contains("rust"));
    }

    #[tokio::test]
    async fn test_generate_explanation_enabled() {
        let service = AIService::new().with_enabled(true);
        let result = service.generate_explanation("rust").await.unwrap();
        assert!(result.contains("AI-generated explanation"));
        assert!(result.contains("rust"));
        assert!(result.contains("not implemented yet"));
    }
} 