use std::time::Duration;
use ratatui::crossterm::event::{self, Event as CrosstermEvent, KeyEvent};
use tracing::{debug, instrument};

use crate::infrastructure::error::GroundhogError;

/// TUI events
#[derive(Debug)]
pub enum Event {
    /// Terminal key press event
    Key(KeyEvent),
    /// Application tick event
    Tick,
    /// Resize event
    Resize(u16, u16),
    /// Mouse event (future use)
    Mouse,
}

/// Event handler for TUI
pub struct EventHandler {
    /// Tick rate for app updates
    tick_rate: Duration,
}

impl EventHandler {
    /// Create a new event handler
    pub fn new(tick_rate: Duration) -> Self {
        Self { tick_rate }
    }

    /// Poll for the next event
    #[instrument(skip(self))]
    pub fn next(&self) -> Result<Event, GroundhogError> {
        if event::poll(self.tick_rate).map_err(|e| GroundhogError::TUIError(e.to_string()))? {
            match event::read().map_err(|e| GroundhogError::TUIError(e.to_string()))? {
                CrosstermEvent::Key(key_event) => {
                    debug!("Key event: {:?}", key_event);
                    Ok(Event::Key(key_event))
                }
                CrosstermEvent::Resize(width, height) => {
                    debug!("Resize event: {}x{}", width, height);
                    Ok(Event::Resize(width, height))
                }
                CrosstermEvent::Mouse(_) => {
                    debug!("Mouse event");
                    Ok(Event::Mouse)
                }
                _ => Ok(Event::Tick),
            }
        } else {
            Ok(Event::Tick)
        }
    }
}

impl Default for EventHandler {
    fn default() -> Self {
        Self::new(Duration::from_millis(100))
    }
} 