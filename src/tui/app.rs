use std::io;
use ratatui::{
    backend::CrosstermBackend,
    crossterm::{
        event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode},
        execute,
        terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
    },
    Terminal,
};
use tracing::{info, instrument};

use crate::infrastructure::error::GroundhogError;
use super::ui;

/// Main TUI application state
pub struct App {
    /// Should the application quit?
    pub should_quit: bool,
    /// Current message to display
    pub message: String,
    /// Counter for demo purposes
    pub counter: u32,
}

impl App {
    /// Create a new App instance
    pub fn new() -> Self {
        Self {
            should_quit: false,
            message: "Hello, Groundhog! 🐹".to_string(),
            counter: 0,
        }
    }

    /// Run the TUI application
    #[instrument(skip(self))]
    pub async fn run(&mut self) -> Result<(), GroundhogError> {
        info!("Starting TUI application");

        // Setup terminal
        enable_raw_mode().map_err(|e| GroundhogError::TUIError(e.to_string()))?;
        let mut stdout = io::stdout();
        execute!(stdout, EnterAlternateScreen, EnableMouseCapture)
            .map_err(|e| GroundhogError::TUIError(e.to_string()))?;
        let backend = CrosstermBackend::new(stdout);
        let mut terminal = Terminal::new(backend)
            .map_err(|e| GroundhogError::TUIError(e.to_string()))?;

        // Main application loop
        let result = self.run_loop(&mut terminal).await;

        // Restore terminal
        disable_raw_mode().map_err(|e| GroundhogError::TUIError(e.to_string()))?;
        execute!(
            terminal.backend_mut(),
            LeaveAlternateScreen,
            DisableMouseCapture
        )
        .map_err(|e| GroundhogError::TUIError(e.to_string()))?;
        terminal.show_cursor().map_err(|e| GroundhogError::TUIError(e.to_string()))?;

        info!("TUI application stopped");
        result
    }

    /// Main application event loop
    #[instrument(skip(self, terminal))]
    async fn run_loop(&mut self, terminal: &mut Terminal<CrosstermBackend<io::Stdout>>) -> Result<(), GroundhogError> {
        loop {
            // Draw the UI
            terminal
                .draw(|f| ui::render(f, self))
                .map_err(|e| GroundhogError::TUIError(e.to_string()))?;

            // Handle events
            if event::poll(std::time::Duration::from_millis(100))
                .map_err(|e| GroundhogError::TUIError(e.to_string()))?
            {
                if let Event::Key(key) = event::read().map_err(|e| GroundhogError::TUIError(e.to_string()))? {
                    match key.code {
                        KeyCode::Char('q') => {
                            self.should_quit = true;
                        }
                        KeyCode::Char(' ') => {
                            self.counter += 1;
                            self.message = format!("Counter: {} (Press 'q' to quit, Space to increment)", self.counter);
                        }
                        KeyCode::Char('r') => {
                            self.counter = 0;
                            self.message = "Counter reset! 🐹".to_string();
                        }
                        _ => {}
                    }
                }
            }

            if self.should_quit {
                break;
            }
        }

        Ok(())
    }
}

impl Default for App {
    fn default() -> Self {
        Self::new()
    }
} 