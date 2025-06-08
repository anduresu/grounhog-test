use tracing::{info, instrument};

use crate::infrastructure::error::GroundhogError;
use crate::tui;

/// Handle the TUI command
#[instrument]
pub async fn handle_tui(debug_mode: bool) -> Result<(), GroundhogError> {
    info!("Starting TUI mode (debug: {})", debug_mode);

    if debug_mode {
        info!("TUI debug mode enabled");
    }

    // Launch the TUI application
    tui::run().await?;

    info!("TUI mode ended");
    Ok(())
} 