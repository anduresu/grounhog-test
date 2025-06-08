pub mod app;
pub mod ui;
pub mod event;

pub use app::App;
pub use ui::render;
pub use event::{Event, EventHandler};

use crate::infrastructure::error::GroundhogError;

/// Initialize and run the TUI application
pub async fn run() -> Result<(), GroundhogError> {
    let mut app = App::new();
    app.run().await
} 