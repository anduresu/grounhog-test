# MCP Directory Listing Tool Specification

## Overview

The MCP Directory Listing Tool (`ls`) is a secure, sandboxed utility that provides directory contents listing functionality within the Groundhog AI assistant ecosystem. This tool follows the Model Context Protocol (MCP) standard and implements comprehensive security measures to prevent unauthorized file system access.

## Tool Metadata

```json
{
  "name": "directory_list",
  "version": "1.0.0",
  "description": "Securely list directory contents with filtering and security controls",
  "author": "Groundhog Team",
  "categories": ["filesystem", "utility", "navigation"],
  "mcp_version": "1.0.0",
  "tool_type": "filesystem",
  "trust_level": "verified"
}
```

## Capabilities

### Core Functionality
- **Directory Listing**: List files and directories in specified paths
- **Recursive Traversal**: Optional recursive directory exploration
- **Filtering**: Pattern-based filtering of results
- **Metadata Retrieval**: File size, permissions, modification dates
- **Symbolic Link Handling**: Safe resolution of symbolic links

### Security Features
- **Path Validation**: Strict validation of input paths
- **Access Control**: Permission-based access restrictions
- **Sandboxing**: Isolated execution environment
- **Resource Limits**: CPU, memory, and I/O constraints
- **Audit Logging**: Comprehensive activity logging

## Tool Schema

### Input Schema

```json
{
  "$schema": "http://json-schema.org/draft-07/schema#",
  "type": "object",
  "properties": {
    "path": {
      "type": "string",
      "description": "Directory path to list",
      "pattern": "^[a-zA-Z0-9/_.-]+$",
      "maxLength": 1024
    },
    "recursive": {
      "type": "boolean",
      "description": "Enable recursive directory traversal",
      "default": false
    },
    "max_depth": {
      "type": "integer",
      "description": "Maximum recursion depth",
      "minimum": 1,
      "maximum": 10,
      "default": 3
    },
    "include_hidden": {
      "type": "boolean",
      "description": "Include hidden files and directories",
      "default": false
    },
    "filter": {
      "type": "object",
      "properties": {
        "pattern": {
          "type": "string",
          "description": "Glob pattern for filtering",
          "maxLength": 256
        },
        "file_types": {
          "type": "array",
          "items": {
            "type": "string",
            "enum": ["file", "directory", "symlink", "socket", "pipe", "device"]
          },
          "description": "Filter by file types"
        },
        "size_range": {
          "type": "object",
          "properties": {
            "min": { "type": "integer", "minimum": 0 },
            "max": { "type": "integer", "minimum": 0 }
          },
          "description": "Filter by file size range (bytes)"
        }
      }
    },
    "sort": {
      "type": "object",
      "properties": {
        "by": {
          "type": "string",
          "enum": ["name", "size", "modified", "type"],
          "default": "name"
        },
        "order": {
          "type": "string",
          "enum": ["asc", "desc"],
          "default": "asc"
        }
      }
    },
    "limit": {
      "type": "integer",
      "description": "Maximum number of entries to return",
      "minimum": 1,
      "maximum": 1000,
      "default": 100
    }
  },
  "required": ["path"],
  "additionalProperties": false
}
```

### Output Schema

```json
{
  "$schema": "http://json-schema.org/draft-07/schema#",
  "type": "object",
  "properties": {
    "path": {
      "type": "string",
      "description": "Requested directory path"
    },
    "entries": {
      "type": "array",
      "items": {
        "type": "object",
        "properties": {
          "name": {
            "type": "string",
            "description": "File or directory name"
          },
          "path": {
            "type": "string",
            "description": "Full path to the entry"
          },
          "type": {
            "type": "string",
            "enum": ["file", "directory", "symlink", "socket", "pipe", "device"],
            "description": "Entry type"
          },
          "size": {
            "type": "integer",
            "description": "Size in bytes (files only)"
          },
          "permissions": {
            "type": "string",
            "description": "Unix-style permissions (e.g., rwxr-xr-x)"
          },
          "owner": {
            "type": "string",
            "description": "File owner"
          },
          "group": {
            "type": "string",
            "description": "File group"
          },
          "modified": {
            "type": "string",
            "format": "date-time",
            "description": "Last modification time"
          },
          "accessed": {
            "type": "string",
            "format": "date-time",
            "description": "Last access time"
          },
          "symlink_target": {
            "type": "string",
            "description": "Target path for symbolic links"
          }
        },
        "required": ["name", "path", "type", "permissions", "modified"]
      }
    },
    "metadata": {
      "type": "object",
      "properties": {
        "total_entries": {
          "type": "integer",
          "description": "Total number of entries found"
        },
        "filtered_entries": {
          "type": "integer",
          "description": "Number of entries after filtering"
        },
        "execution_time": {
          "type": "number",
          "description": "Execution time in seconds"
        },
        "truncated": {
          "type": "boolean",
          "description": "Whether results were truncated due to limits"
        }
      }
    }
  },
  "required": ["path", "entries", "metadata"]
}
```

## Security Implementation

### 1. Path Validation and Sanitization

```rust
#[derive(Debug, Clone)]
pub struct PathValidator {
    /// Allowed base paths
    allowed_paths: Vec<PathBuf>,
    
    /// Blocked paths (blacklist)
    blocked_paths: Vec<PathBuf>,
    
    /// Maximum path length
    max_path_length: usize,
    
    /// Allow symbolic links
    allow_symlinks: bool,
}

impl PathValidator {
    pub fn validate_path(&self, path: &str) -> Result<PathBuf, SecurityError> {
        // 1. Basic format validation
        if path.len() > self.max_path_length {
            return Err(SecurityError::PathTooLong);
        }
        
        // 2. Sanitize path - remove dangerous sequences
        let sanitized = self.sanitize_path(path)?;
        let canonical_path = self.canonicalize_path(&sanitized)?;
        
        // 3. Check against allowed paths
        if !self.is_path_allowed(&canonical_path) {
            return Err(SecurityError::PathNotAllowed);
        }
        
        // 4. Check against blocked paths
        if self.is_path_blocked(&canonical_path) {
            return Err(SecurityError::PathBlocked);
        }
        
        // 5. Validate symbolic links
        if canonical_path.is_symlink() && !self.allow_symlinks {
            return Err(SecurityError::SymlinksNotAllowed);
        }
        
        Ok(canonical_path)
    }
    
    fn sanitize_path(&self, path: &str) -> Result<String, SecurityError> {
        // Remove dangerous sequences
        let dangerous_patterns = [
            "../", "./", "//", "~", "$", "`", ";", "|", "&", "*", "?", 
            "[", "]", "{", "}", "(", ")", "<", ">", "\"", "'", "\\",
            "\0", "\n", "\r", "\t"
        ];
        
        let mut sanitized = path.to_string();
        for pattern in &dangerous_patterns {
            if sanitized.contains(pattern) {
                return Err(SecurityError::DangerousPathSequence(pattern.to_string()));
            }
        }
        
        // Ensure path is absolute or relative to allowed base
        if !sanitized.starts_with('/') && !sanitized.starts_with("./") {
            sanitized = format!("./{}", sanitized);
        }
        
        Ok(sanitized)
    }
}
```

### 2. Permission-Based Access Control

```rust
#[derive(Debug, Clone)]
pub struct AccessController {
    /// User permissions
    user_permissions: UserPermissions,
    
    /// System-wide restrictions
    system_restrictions: SystemRestrictions,
}

#[derive(Debug, Clone)]
pub struct UserPermissions {
    /// Allowed read paths
    read_paths: Vec<PathPattern>,
    
    /// Maximum directory depth
    max_depth: u32,
    
    /// Maximum entries per operation
    max_entries: u32,
    
    /// Allow hidden files access
    allow_hidden: bool,
    
    /// Allow system directories
    allow_system_dirs: bool,
}

#[derive(Debug, Clone)]
pub struct SystemRestrictions {
    /// Blocked system paths
    blocked_system_paths: Vec<PathBuf>,
    
    /// Sensitive directories
    sensitive_dirs: Vec<PathBuf>,
    
    /// Maximum file size to examine
    max_file_size: u64,
}

impl AccessController {
    pub fn check_access(&self, path: &Path, operation: &Operation) -> Result<(), SecurityError> {
        // Check user permissions
        if !self.user_permissions.can_read_path(path) {
            return Err(SecurityError::AccessDenied);
        }
        
        // Check system restrictions
        if self.system_restrictions.is_sensitive_path(path) {
            return Err(SecurityError::SensitivePathAccess);
        }
        
        // Check depth limits
        if let Operation::RecursiveList { depth } = operation {
            if *depth > self.user_permissions.max_depth {
                return Err(SecurityError::DepthLimitExceeded);
            }
        }
        
        Ok(())
    }
}
```

### 3. Resource Limits and Rate Limiting

```rust
#[derive(Debug, Clone)]
pub struct ResourceLimiter {
    /// Maximum execution time
    max_execution_time: Duration,
    
    /// Maximum memory usage
    max_memory: u64,
    
    /// Maximum number of file system operations
    max_fs_operations: u32,
    
    /// Rate limiter for requests
    rate_limiter: RateLimiter,
}

#[derive(Debug, Clone)]
pub struct RateLimiter {
    /// Requests per minute
    requests_per_minute: u32,
    
    /// Burst capacity
    burst_capacity: u32,
    
    /// Current request count
    current_requests: Arc<AtomicU32>,
    
    /// Last reset time
    last_reset: Arc<Mutex<Instant>>,
}

impl ResourceLimiter {
    pub async fn execute_with_limits<F, T>(&self, operation: F) -> Result<T, SecurityError>
    where
        F: Future<Output = Result<T, SecurityError>>,
    {
        // Check rate limits
        self.rate_limiter.check_rate_limit()?;
        
        // Execute with timeout
        let result = tokio::time::timeout(
            self.max_execution_time,
            operation
        ).await;
        
        match result {
            Ok(Ok(value)) => Ok(value),
            Ok(Err(e)) => Err(e),
            Err(_) => Err(SecurityError::ExecutionTimeout),
        }
    }
}
```

### 4. Sandboxing Configuration

```rust
#[derive(Debug, Clone)]
pub struct SandboxConfig {
    /// Enable process isolation
    enable_process_isolation: bool,
    
    /// Filesystem restrictions
    filesystem: FilesystemSandbox,
    
    /// Network restrictions
    network: NetworkSandbox,
    
    /// System call restrictions
    syscalls: SyscallRestrictions,
}

#[derive(Debug, Clone)]
pub struct FilesystemSandbox {
    /// Read-only paths
    readonly_paths: Vec<PathBuf>,
    
    /// Completely blocked paths
    blocked_paths: Vec<PathBuf>,
    
    /// Temporary directory for tool
    temp_dir: PathBuf,
    
    /// Maximum file operations
    max_operations: u32,
}

#[derive(Debug, Clone)]
pub struct NetworkSandbox {
    /// Block all network access
    block_all: bool,
    
    /// Allowed outbound connections (if any)
    allowed_hosts: Vec<String>,
    
    /// Allowed ports
    allowed_ports: Vec<u16>,
}

#[derive(Debug, Clone)]
pub struct SyscallRestrictions {
    /// Allowed system calls
    allowed_syscalls: Vec<String>,
    
    /// Blocked system calls
    blocked_syscalls: Vec<String>,
    
    /// Enable seccomp filtering
    enable_seccomp: bool,
}
```

## Security Best Practices

### 1. Input Validation
- **Path Sanitization**: Remove dangerous path sequences (../, ~, etc.)
- **Length Limits**: Enforce maximum path and parameter lengths
- **Character Filtering**: Only allow safe characters in paths
- **Canonical Path Resolution**: Resolve all symbolic links and relative paths

### 2. Access Control
- **Whitelist Approach**: Only allow access to explicitly permitted paths
- **Depth Limits**: Restrict recursive traversal depth
- **Size Limits**: Limit the number of entries returned
- **Hidden File Control**: Restrict access to hidden files by default

### 3. Resource Protection
- **Execution Timeouts**: Prevent long-running operations
- **Memory Limits**: Restrict memory usage
- **File Operation Limits**: Limit number of filesystem operations
- **Rate Limiting**: Prevent abuse through excessive requests

### 4. Audit and Monitoring

```rust
#[derive(Debug, Clone)]
pub struct AuditLogger {
    /// Audit event sink
    event_sink: Arc<dyn AuditEventSink>,
    
    /// Security context
    security_context: SecurityContext,
}

impl AuditLogger {
    pub fn log_directory_access(&self, event: DirectoryAccessEvent) {
        let audit_event = AuditEvent {
            timestamp: Utc::now(),
            tool_id: "directory_list".to_string(),
            event_type: AuditEventType::FileSystemAccess,
            details: serde_json::to_value(event).unwrap(),
            security_context: self.security_context.clone(),
            risk_level: self.assess_risk_level(&event),
        };
        
        self.event_sink.send(audit_event);
    }
    
    fn assess_risk_level(&self, event: &DirectoryAccessEvent) -> RiskLevel {
        if event.path.starts_with("/etc") || event.path.starts_with("/root") {
            RiskLevel::High
        } else if event.recursive && event.depth > 5 {
            RiskLevel::Medium
        } else {
            RiskLevel::Low
        }
    }
}

#[derive(Debug, Clone, Serialize)]
pub struct DirectoryAccessEvent {
    pub path: String,
    pub recursive: bool,
    pub depth: u32,
    pub entries_returned: u32,
    pub filters_applied: Vec<String>,
    pub execution_time: Duration,
    pub user_id: String,
    pub session_id: String,
}
```

## Configuration

### Tool Configuration

```toml
[mcp.tools.directory_list]
# Enable the directory listing tool
enabled = true

# Security settings
[mcp.tools.directory_list.security]
# Maximum path length
max_path_length = 1024

# Allow symbolic links
allow_symlinks = false

# Maximum recursion depth
max_depth = 10

# Maximum entries per request
max_entries = 1000

# Allowed base paths (whitelist)
allowed_paths = [
    "/home/user/projects",
    "/tmp/workspace",
    "./",
]

# Blocked paths (blacklist)
blocked_paths = [
    "/etc",
    "/root",
    "/proc",
    "/sys",
    "/dev",
    "~/.ssh",
    "~/.gnupg",
]

# Resource limits
[mcp.tools.directory_list.limits]
# Maximum execution time (seconds)
max_execution_time = 30

# Maximum memory usage (MB)
max_memory_mb = 256

# Maximum file system operations
max_fs_operations = 10000

# Rate limiting
[mcp.tools.directory_list.rate_limits]
# Requests per minute
requests_per_minute = 120

# Burst capacity
burst_capacity = 10

# Sandboxing
[mcp.tools.directory_list.sandbox]
# Enable process isolation
enable_process_isolation = true

# Block network access
block_network = true

# Enable seccomp filtering
enable_seccomp = true

# Audit settings
[mcp.tools.directory_list.audit]
# Enable audit logging
enabled = true

# Log successful operations
log_success = true

# Log security violations
log_violations = true

# Audit retention (days)
retention_days = 30
```

## Error Handling

### Error Types

```rust
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum DirectoryListError {
    /// Security-related errors
    Security(SecurityError),
    
    /// Path validation errors
    InvalidPath(String),
    
    /// Permission denied
    AccessDenied(String),
    
    /// Path does not exist
    PathNotFound(String),
    
    /// Resource limit exceeded
    ResourceLimitExceeded(String),
    
    /// I/O error
    IoError(String),
    
    /// Configuration error
    ConfigError(String),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum SecurityError {
    PathTooLong,
    PathNotAllowed,
    PathBlocked,
    SymlinksNotAllowed,
    DangerousPathSequence(String),
    AccessDenied,
    SensitivePathAccess,
    DepthLimitExceeded,
    ExecutionTimeout,
    RateLimitExceeded,
    ResourceLimitExceeded,
}
```

### Error Response Format

```json
{
  "error": {
    "type": "security_error",
    "code": "path_not_allowed",
    "message": "Access to the specified path is not permitted",
    "details": {
      "requested_path": "/etc/passwd",
      "reason": "Path not in allowed list",
      "suggestion": "Request access to an allowed directory"
    },
    "timestamp": "2024-01-01T12:00:00Z"
  }
}
```

## Testing Strategy

### Security Testing
- **Path Traversal Tests**: Verify protection against directory traversal attacks
- **Permission Bypass Tests**: Ensure access controls cannot be circumvented
- **Resource Exhaustion Tests**: Verify resource limits are enforced
- **Injection Tests**: Test for command and path injection vulnerabilities

### Performance Testing
- **Large Directory Tests**: Test with directories containing many files
- **Deep Recursion Tests**: Verify performance with deep directory structures
- **Concurrent Access Tests**: Test tool under concurrent usage
- **Memory Usage Tests**: Verify memory limits are respected

### Integration Testing
- **MCP Protocol Tests**: Verify compliance with MCP standard
- **Registry Integration Tests**: Test tool registration and discovery
- **Audit System Tests**: Verify audit events are properly generated 