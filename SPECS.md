# Groundhog AI Coding Assistant - Specifications Overview

This document provides an overview of all specifications for the Groundhog AI coding assistant CLI application.

## Project Overview

Groundhog is an AI-powered coding assistant implemented as a command-line interface (CLI) application in Rust. The application provides various commands to help developers with coding tasks, analysis, and assistance.

## Specification Documents

| Topic | Document | Description |
|-------|----------|-------------|
| **Project Architecture** | [specs/architecture.md](specs/architecture.md) | High-level system architecture and design patterns |
| **Command Line Interface** | [specs/cli.md](specs/cli.md) | CLI structure, commands, and argument parsing |
| **Logging & Observability** | [specs/logging.md](specs/logging.md) | Tracing, metrics, telemetry, and monitoring requirements |
| **Commands Specification** | [specs/commands.md](specs/commands.md) | Detailed specification of all CLI commands |
| **Error Handling** | [specs/error-handling.md](specs/error-handling.md) | Error types, handling strategies, and user messaging |
| **Configuration** | [specs/configuration.md](specs/configuration.md) | Configuration file format and runtime settings |
| **Development Workflow** | [specs/development.md](specs/development.md) | Development setup, testing, and deployment procedures |
| **Dependencies** | [specs/dependencies.md](specs/dependencies.md) | Third-party libraries and their usage |

### MCP (Model Context Protocol) Specifications

| Topic | Document | Description |
|-------|----------|-------------|
| **MCP Protocol** | [specs/mcp/protocol.md](specs/mcp/protocol.md) | Core MCP protocol implementation with security framework |
| **MCP Registry** | [specs/mcp/registry.md](specs/mcp/registry.md) | Tool registry, discovery, and security management |
| **Directory List Tool** | [specs/mcp/ls-tool.md](specs/mcp/ls-tool.md) | Secure filesystem directory listing tool specification |
| **LLM System Prompt** | [specs/mcp/system-prompt.md](specs/mcp/system-prompt.md) | System prompt for safe MCP tool usage by LLMs |

## Quick Start

To get started with development or usage, see:
- [Development Workflow](specs/development.md) for setting up the development environment
- [CLI Specification](specs/cli.md) for command usage examples
- [Architecture](specs/architecture.md) for understanding the system design

## Version

Current specification version: 0.1.0
Last updated: 2024 