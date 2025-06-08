# MCP Protocol Specification

## Overview

This document defines the Model Context Protocol (MCP) implementation for the Groundhog AI assistant. MCP provides a standardized way for language models to securely interact with external tools and services while maintaining strict security boundaries and comprehensive audit trails.

## Protocol Architecture

### Core Components

```
┌─────────────────────────────────────────────────────────────┐
│                    MCP Protocol Layer                      │
├─────────────────────────────────────────────────────────────┤
│                                                             │
│  ┌─────────────────┐  ┌─────────────────┐  ┌─────────────┐ │
│  │   Message       │  │   Security      │  │   Session   │ │
│  │   Handler       │  │   Validator     │  │   Manager   │ │
│  └─────────────────┘  └─────────────────┘  └─────────────┘ │
│                                                             │
│  ┌─────────────────┐  ┌─────────────────┐  ┌─────────────┐ │
│  │   Tool          │  │   Serialization │  │   Error     │ │
│  │   Registry      │  │   Engine        │  │   Handler   │ │
│  └─────────────────┘  └─────────────────┘  └─────────────┘ │
│                                                             │
├─────────────────────────────────────────────────────────────┤
│                   Transport Layer                          │
│                 (JSON-RPC over WebSocket)                 │
└─────────────────────────────────────────────────────────────┘
```

## Protocol Messages

### Message Format

All MCP messages follow a standardized JSON-RPC 2.0 format:

```json
{
  "jsonrpc": "2.0",
  "id": "unique-request-id",
  "method": "method-name",
  "params": {
    "parameter": "value"
  }
}
```

### Request Types

#### Tool Discovery
```json
{
  "jsonrpc": "2.0",
  "id": "1",
  "method": "tools/list",
  "params": {
    "category": "filesystem",
    "trust_level": "verified"
  }
}
```

#### Tool Invocation
```json
{
  "jsonrpc": "2.0",
  "id": "2",
  "method": "tools/invoke",
  "params": {
    "tool_id": "directory_list",
    "tool_version": "1.0.0",
    "parameters": {
      "path": "./src",
      "recursive": true,
      "max_depth": 3
    },
    "security_context": {
      "user_id": "user123",
      "session_id": "session456",
      "permissions": ["read:filesystem"]
    }
  }
}
```

#### Tool Registration
```json
{
  "jsonrpc": "2.0",
  "id": "3",
  "method": "tools/register",
  "params": {
    "tool_metadata": {
      "name": "directory_list",
      "version": "1.0.0",
      "description": "Secure directory listing tool",
      "author": "Groundhog Team",
      "categories": ["filesystem", "utility"]
    },
    "security_profile": {
      "permissions": ["read:filesystem"],
      "sandbox_level": "container",
      "resource_limits": {
        "max_execution_time": 30,
        "max_memory": 268435456
      }
    },
    "schema": {
      "input": { /* JSON Schema */ },
      "output": { /* JSON Schema */ }
    }
  }
}
```

### Response Format

#### Success Response
```json
{
  "jsonrpc": "2.0",
  "id": "2",
  "result": {
    "tool_id": "directory_list",
    "execution_id": "exec-789",
    "status": "success",
    "data": {
      "path": "./src",
      "entries": [
        {
          "name": "main.rs",
          "path": "./src/main.rs",
          "type": "file",
          "size": 1024,
          "permissions": "rw-r--r--",
          "modified": "2024-01-01T12:00:00Z"
        }
      ],
      "metadata": {
        "total_entries": 1,
        "execution_time": 0.123,
        "truncated": false
      }
    },
    "security_context": {
      "access_granted": true,
      "permissions_used": ["read:filesystem"],
      "audit_id": "audit-abc123"
    }
  }
}
```

#### Error Response
```json
{
  "jsonrpc": "2.0",
  "id": "2",
  "error": {
    "code": -32601,
    "message": "Tool execution failed",
    "data": {
      "error_type": "security_error",
      "error_code": "path_not_allowed",
      "error_message": "Access to the specified path is not permitted",
      "details": {
        "requested_path": "/etc/passwd",
        "reason": "Path not in allowed list",
        "suggestion": "Request access to an allowed directory"
      },
      "security_context": {
        "violation_type": "unauthorized_access",
        "audit_id": "audit-xyz789"
      },
      "timestamp": "2024-01-01T12:00:00Z"
    }
  }
}
```

## Security Framework

### Authentication and Authorization

#### Session Management
```rust
#[derive(Debug, Clone)]
pub struct McpSession {
    /// Unique session identifier
    pub session_id: String,
    
    /// User context
    pub user_context: UserContext,
    
    /// Security permissions
    pub permissions: Vec<Permission>,
    
    /// Session creation time
    pub created_at: DateTime<Utc>,
    
    /// Session expiration time
    pub expires_at: DateTime<Utc>,
    
    /// Rate limiting state
    pub rate_limit_state: RateLimitState,
}

#[derive(Debug, Clone)]
pub struct UserContext {
    /// User identifier
    pub user_id: String,
    
    /// User role
    pub role: UserRole,
    
    /// Trust level
    pub trust_level: TrustLevel,
    
    /// Allowed resources
    pub allowed_resources: Vec<ResourcePattern>,
}
```

#### Permission Validation
```rust
#[derive(Debug, Clone)]
pub struct PermissionValidator {
    /// Permission policies
    policies: Vec<PermissionPolicy>,
    
    /// Security context
    security_context: SecurityContext,
}

impl PermissionValidator {
    pub fn validate_tool_access(
        &self,
        tool_id: &str,
        user_context: &UserContext,
        requested_permissions: &[Permission],
    ) -> Result<ValidationResult, SecurityError> {
        // 1. Check user permissions
        for permission in requested_permissions {
            if !self.user_has_permission(user_context, permission) {
                return Err(SecurityError::PermissionDenied {
                    permission: permission.clone(),
                    reason: "User lacks required permission".to_string(),
                });
            }
        }
        
        // 2. Check tool-specific policies
        if let Some(policy) = self.get_tool_policy(tool_id) {
            policy.validate_access(user_context, requested_permissions)?;
        }
        
        // 3. Apply security constraints
        let constraints = self.generate_security_constraints(user_context, tool_id);
        
        Ok(ValidationResult {
            granted: true,
            constraints,
            audit_context: self.create_audit_context(user_context, tool_id),
        })
    }
}
```

### Input Validation

#### Schema Validation
```rust
#[derive(Debug, Clone)]
pub struct InputValidator {
    /// JSON Schema validator
    schema_validator: SchemaValidator,
    
    /// Content sanitizers
    sanitizers: Vec<Box<dyn ContentSanitizer>>,
    
    /// Security filters
    security_filters: Vec<Box<dyn SecurityFilter>>,
}

impl InputValidator {
    pub fn validate_tool_input(
        &self,
        tool_id: &str,
        input: &serde_json::Value,
    ) -> Result<ValidatedInput, ValidationError> {
        // 1. Schema validation
        let schema = self.get_tool_schema(tool_id)?;
        self.schema_validator.validate(input, &schema)?;
        
        // 2. Security filtering
        let filtered_input = self.apply_security_filters(input)?;
        
        // 3. Content sanitization
        let sanitized_input = self.apply_sanitizers(&filtered_input)?;
        
        Ok(ValidatedInput {
            original: input.clone(),
            validated: sanitized_input,
            validation_metadata: self.create_validation_metadata(),
        })
    }
}
```

### Output Sanitization

#### Response Filtering
```rust
#[derive(Debug, Clone)]
pub struct OutputSanitizer {
    /// Content filters
    filters: Vec<Box<dyn OutputFilter>>,
    
    /// Privacy protections
    privacy_filters: Vec<Box<dyn PrivacyFilter>>,
    
    /// Security redaction rules
    redaction_rules: Vec<RedactionRule>,
}

impl OutputSanitizer {
    pub fn sanitize_output(
        &self,
        tool_id: &str,
        output: &serde_json::Value,
        security_context: &SecurityContext,
    ) -> Result<SanitizedOutput, SanitizationError> {
        let mut sanitized = output.clone();
        
        // 1. Apply security redaction
        for rule in &self.redaction_rules {
            if rule.applies_to_tool(tool_id) {
                sanitized = rule.apply_redaction(sanitized, security_context)?;
            }
        }
        
        // 2. Apply privacy filters
        for filter in &self.privacy_filters {
            sanitized = filter.filter_output(sanitized, security_context)?;
        }
        
        // 3. Apply content filters
        for filter in &self.filters {
            sanitized = filter.filter_content(sanitized)?;
        }
        
        Ok(SanitizedOutput {
            original_size: self.calculate_size(output),
            sanitized_size: self.calculate_size(&sanitized),
            sanitized_content: sanitized,
            redactions_applied: self.count_redactions(),
        })
    }
}
```

## Tool Lifecycle Management

### Tool Registration Process

```rust
#[derive(Debug, Clone)]
pub struct ToolRegistrationManager {
    /// Registry storage
    registry: Arc<dyn ToolRegistry>,
    
    /// Security validator
    security_validator: SecurityValidator,
    
    /// Schema validator
    schema_validator: SchemaValidator,
}

impl ToolRegistrationManager {
    pub async fn register_tool(
        &self,
        registration: ToolRegistration,
    ) -> Result<RegistrationResult, RegistrationError> {
        // 1. Validate tool metadata
        self.validate_metadata(&registration.metadata)?;
        
        // 2. Validate security profile
        self.security_validator.validate_security_profile(&registration.security)?;
        
        // 3. Validate tool schema
        self.schema_validator.validate_tool_schema(&registration.schema)?;
        
        // 4. Check for conflicts
        self.check_registration_conflicts(&registration)?;
        
        // 5. Store registration
        let tool_id = self.registry.store_registration(registration).await?;
        
        // 6. Initialize tool instance
        let tool_instance = self.initialize_tool_instance(&tool_id).await?;
        
        Ok(RegistrationResult {
            tool_id,
            tool_instance,
            registration_time: Utc::now(),
        })
    }
}
```

### Tool Discovery

```rust
#[derive(Debug, Clone)]
pub struct ToolDiscoveryService {
    /// Tool registry
    registry: Arc<dyn ToolRegistry>,
    
    /// Search engine
    search_engine: SearchEngine,
    
    /// Permission checker
    permission_checker: PermissionChecker,
}

impl ToolDiscoveryService {
    pub async fn discover_tools(
        &self,
        query: DiscoveryQuery,
        user_context: &UserContext,
    ) -> Result<DiscoveryResult, DiscoveryError> {
        // 1. Apply user permission filters
        let accessible_tools = self.filter_by_permissions(&query, user_context).await?;
        
        // 2. Apply search criteria
        let matching_tools = self.search_engine.search(&accessible_tools, &query).await?;
        
        // 3. Sort and rank results
        let ranked_tools = self.rank_search_results(matching_tools, &query).await?;
        
        // 4. Apply result limits
        let limited_results = self.apply_result_limits(ranked_tools, &query);
        
        Ok(DiscoveryResult {
            tools: limited_results,
            total_matches: matching_tools.len(),
            search_time: self.measure_search_time(),
        })
    }
}
```

## Error Handling

### Error Categories

```rust
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum McpError {
    /// Protocol-level errors
    Protocol(ProtocolError),
    
    /// Authentication/authorization errors
    Security(SecurityError),
    
    /// Tool execution errors
    Tool(ToolError),
    
    /// Validation errors
    Validation(ValidationError),
    
    /// System errors
    System(SystemError),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ProtocolError {
    /// Invalid JSON-RPC message
    InvalidMessage(String),
    
    /// Unsupported protocol version
    UnsupportedVersion(String),
    
    /// Missing required fields
    MissingFields(Vec<String>),
    
    /// Invalid method name
    InvalidMethod(String),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum SecurityError {
    /// Authentication failed
    AuthenticationFailed(String),
    
    /// Permission denied
    PermissionDenied(String),
    
    /// Rate limit exceeded
    RateLimitExceeded(String),
    
    /// Security policy violation
    PolicyViolation(String),
    
    /// Invalid security context
    InvalidSecurityContext(String),
}
```

### Error Response Generation

```rust
#[derive(Debug, Clone)]
pub struct ErrorHandler {
    /// Error mapping rules
    error_mappings: HashMap<String, ErrorMapping>,
    
    /// Security context
    security_context: SecurityContext,
    
    /// Audit logger
    audit_logger: Arc<dyn AuditLogger>,
}

impl ErrorHandler {
    pub fn handle_error(
        &self,
        error: McpError,
        request_id: &str,
        security_context: &SecurityContext,
    ) -> JsonRpcResponse {
        // 1. Log the error
        self.audit_logger.log_error(&error, request_id, security_context);
        
        // 2. Map to JSON-RPC error
        let rpc_error = self.map_to_rpc_error(&error);
        
        // 3. Apply security filtering
        let filtered_error = self.filter_error_details(rpc_error, security_context);
        
        // 4. Generate response
        JsonRpcResponse::error(request_id, filtered_error)
    }
    
    fn filter_error_details(
        &self,
        error: JsonRpcError,
        security_context: &SecurityContext,
    ) -> JsonRpcError {
        // Remove sensitive information based on security context
        // Provide appropriate level of detail based on user permissions
        // Apply organizational error handling policies
        
        match security_context.trust_level {
            TrustLevel::System => error, // Full details
            TrustLevel::Verified => self.filter_sensitive_details(error),
            TrustLevel::Community => self.provide_basic_error_info(error),
            TrustLevel::Untrusted => self.provide_minimal_error_info(error),
        }
    }
}
```

## Audit and Monitoring

### Audit Event Generation

```rust
#[derive(Debug, Clone)]
pub struct McpAuditLogger {
    /// Event sinks
    event_sinks: Vec<Box<dyn AuditEventSink>>,
    
    /// Audit configuration
    config: AuditConfig,
}

impl McpAuditLogger {
    pub fn log_tool_invocation(&self, event: ToolInvocationEvent) {
        let audit_event = AuditEvent {
            timestamp: Utc::now(),
            event_type: AuditEventType::ToolInvocation,
            source: "mcp_protocol".to_string(),
            user_context: event.user_context.clone(),
            details: serde_json::to_value(event).unwrap(),
            risk_level: self.assess_risk_level(&event),
            correlation_id: event.correlation_id.clone(),
        };
        
        for sink in &self.event_sinks {
            sink.send_event(audit_event.clone());
        }
    }
    
    pub fn log_security_violation(&self, violation: SecurityViolation) {
        let audit_event = AuditEvent {
            timestamp: Utc::now(),
            event_type: AuditEventType::SecurityViolation,
            source: "mcp_security".to_string(),
            user_context: violation.user_context.clone(),
            details: serde_json::to_value(violation).unwrap(),
            risk_level: RiskLevel::High,
            correlation_id: violation.correlation_id.clone(),
        };
        
        // Security violations always logged to all sinks
        for sink in &self.event_sinks {
            sink.send_event(audit_event.clone());
        }
        
        // Additional alerting for high-risk violations
        if violation.severity == ViolationSeverity::Critical {
            self.send_security_alert(violation);
        }
    }
}
```

### Performance Monitoring

```rust
#[derive(Debug, Clone)]
pub struct PerformanceMonitor {
    /// Metrics collector
    metrics: Arc<dyn MetricsCollector>,
    
    /// Performance thresholds
    thresholds: PerformanceThresholds,
}

impl PerformanceMonitor {
    pub fn record_tool_execution(&self, execution: ToolExecution) {
        // Record execution metrics
        self.metrics.record_histogram(
            "mcp_tool_execution_duration",
            execution.duration.as_secs_f64(),
            &[
                ("tool_id", execution.tool_id.as_str()),
                ("status", execution.status.as_str()),
            ],
        );
        
        self.metrics.record_counter(
            "mcp_tool_invocations_total",
            1,
            &[
                ("tool_id", execution.tool_id.as_str()),
                ("status", execution.status.as_str()),
            ],
        );
        
        // Check performance thresholds
        if execution.duration > self.thresholds.execution_time_warning {
            self.metrics.record_counter(
                "mcp_performance_warnings_total",
                1,
                &[
                    ("tool_id", execution.tool_id.as_str()),
                    ("warning_type", "slow_execution"),
                ],
            );
        }
        
        if execution.memory_usage > self.thresholds.memory_usage_warning {
            self.metrics.record_counter(
                "mcp_performance_warnings_total",
                1,
                &[
                    ("tool_id", execution.tool_id.as_str()),
                    ("warning_type", "high_memory"),
                ],
            );
        }
    }
}
```

## Configuration

### Protocol Configuration

```toml
[mcp.protocol]
# Protocol version
version = "1.0.0"

# Transport settings
[mcp.protocol.transport]
# Transport type (websocket, http, tcp)
type = "websocket"

# Connection settings
host = "localhost"
port = 8080
path = "/mcp"

# TLS settings
[mcp.protocol.transport.tls]
enabled = true
cert_file = "/etc/groundhog/certs/server.crt"
key_file = "/etc/groundhog/certs/server.key"
ca_file = "/etc/groundhog/certs/ca.crt"

# Security settings
[mcp.protocol.security]
# Authentication requirements
require_authentication = true
authentication_timeout = 30

# Session management
session_timeout = 3600
max_concurrent_sessions = 100

# Rate limiting
[mcp.protocol.rate_limits]
# Global rate limits
global_requests_per_second = 1000
global_requests_per_minute = 60000

# Per-user rate limits
user_requests_per_second = 10
user_requests_per_minute = 600

# Per-tool rate limits
tool_requests_per_second = 5
tool_requests_per_minute = 300

# Audit settings
[mcp.protocol.audit]
# Enable audit logging
enabled = true

# Audit event types to log
log_tool_invocations = true
log_security_events = true
log_performance_events = true
log_protocol_errors = true

# Audit retention
retention_days = 90

# Audit sinks
[mcp.protocol.audit.sinks]
# File sink
file_sink = true
file_path = "/var/log/groundhog/mcp-audit.log"

# Database sink
database_sink = true
database_url = "postgresql://user:pass@localhost/groundhog"

# Remote sink
remote_sink = false
remote_endpoint = "https://audit.example.com/api/events"
```

## Integration Points

### LLM Integration

```rust
#[derive(Debug, Clone)]
pub struct LlmIntegration {
    /// MCP client
    mcp_client: Arc<McpClient>,
    
    /// Tool registry
    tool_registry: Arc<dyn ToolRegistry>,
    
    /// Security context
    security_context: SecurityContext,
}

impl LlmIntegration {
    pub async fn execute_tool_call(
        &self,
        tool_call: LlmToolCall,
    ) -> Result<ToolCallResult, IntegrationError> {
        // 1. Validate tool call
        let validated_call = self.validate_tool_call(tool_call)?;
        
        // 2. Check permissions
        self.check_tool_permissions(&validated_call)?;
        
        // 3. Execute via MCP
        let mcp_result = self.mcp_client.invoke_tool(
            validated_call.tool_id,
            validated_call.parameters,
            &self.security_context,
        ).await?;
        
        // 4. Process result
        let processed_result = self.process_tool_result(mcp_result)?;
        
        Ok(processed_result)
    }
}
```

### External System Integration

```rust
#[derive(Debug, Clone)]
pub struct ExternalSystemIntegration {
    /// MCP server
    mcp_server: Arc<McpServer>,
    
    /// External system client
    external_client: Arc<dyn ExternalSystemClient>,
    
    /// Message translator
    message_translator: MessageTranslator,
}

impl ExternalSystemIntegration {
    pub async fn handle_external_request(
        &self,
        request: ExternalRequest,
    ) -> Result<ExternalResponse, IntegrationError> {
        // 1. Translate to MCP format
        let mcp_request = self.message_translator.translate_to_mcp(request)?;
        
        // 2. Process via MCP
        let mcp_response = self.mcp_server.handle_request(mcp_request).await?;
        
        // 3. Translate back to external format
        let external_response = self.message_translator.translate_from_mcp(mcp_response)?;
        
        Ok(external_response)
    }
}
```

## Testing Strategy

### Protocol Testing
- **Message Format Tests**: Validate JSON-RPC 2.0 compliance
- **Security Tests**: Verify authentication and authorization
- **Error Handling Tests**: Ensure proper error responses
- **Performance Tests**: Validate rate limiting and resource usage

### Integration Testing
- **Tool Integration Tests**: Verify tool registration and invocation
- **LLM Integration Tests**: Test interaction with language models
- **External System Tests**: Validate third-party integrations
- **End-to-End Tests**: Complete workflow validation

### Security Testing
- **Authentication Bypass Tests**: Attempt to bypass authentication
- **Authorization Escalation Tests**: Test privilege escalation attempts
- **Input Validation Tests**: Verify input sanitization
- **Audit Trail Tests**: Ensure complete audit logging 