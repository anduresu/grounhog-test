use clap::{Parser, Subcommand};
use std::path::PathBuf;

#[derive(Debug, Parser)]
#[command(name = "groundhog")]
#[command(about = "An AI coding assistant command line application")]
#[command(version = "0.1.0")]
#[command(author = "Groundhog Team")]
#[command(subcommand_required = true)]
#[command(arg_required_else_help = false)]
pub struct Cli {
    /// Increase logging verbosity (can be repeated)
    #[arg(short, long, action = clap::ArgAction::Count)]
    pub verbose: u8,

    /// Suppress non-error output
    #[arg(short, long)]
    pub quiet: bool,

    /// Path to configuration file
    #[arg(short, long, value_name = "FILE")]
    pub config: Option<PathBuf>,

    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Debug, Subcommand)]
pub enum Commands {
    /// Provides explanations and demonstrations
    Explain {
        /// Future: example topics
        #[arg(long)]
        topic: Option<String>,
    },
    /// Launch the TUI (Terminal User Interface)
    Tui {
        /// Enable TUI debug mode
        #[arg(long)]
        debug: bool,
    },
} 