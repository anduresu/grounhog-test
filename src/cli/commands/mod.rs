pub mod explain;
pub mod tui;

use crate::cli::Commands;
use crate::infrastructure::error::GroundhogError;

/// Execute a command based on the provided command enum
#[tracing::instrument(name = "command.execute", fields(command = %get_command_name(&command)))]
pub async fn execute_command(command: Commands) -> Result<(), GroundhogError> {
    match command {
        Commands::Explain { topic } => explain::execute(topic),
        Commands::Tui { debug } => tui::handle_tui(debug).await,
    }
}

fn get_command_name(command: &Commands) -> &'static str {
    match command {
        Commands::Explain { .. } => "explain",
        Commands::Tui { .. } => "tui",
    }
} 