//! Groundhog AI Coding Assistant
//!
//! A command-line AI coding assistant built in Rust.

pub mod cli;
pub mod core;
pub mod infrastructure;
pub mod tui;

// Re-export main types for convenience
pub use cli::{Cli, Commands};
pub use infrastructure::error::GroundhogError;

pub type Result<T> = std::result::Result<T, GroundhogError>; 