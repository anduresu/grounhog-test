# Event Handling Specification

## Overview

The event handling system in the Groundhog TUI manages user input, terminal events, and application state updates. It provides a responsive interface that reacts to user actions in real-time.

## Event Types

### Core Event Enum
```rust
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
```

**Event Categories**:
- **Key Events**: User keyboard input
- **Tick Events**: Regular application updates
- **Resize Events**: Terminal size changes
- **Mouse Events**: Future mouse interaction support

## Event Handler

### EventHandler Structure
```rust
pub struct EventHandler {
    /// Tick rate for app updates
    tick_rate: Duration,
}

impl EventHandler {
    pub fn new(tick_rate: Duration) -> Self {
        Self { tick_rate }
    }
    
    pub fn next(&self) -> Result<Event, GroundhogError> {
        // Event polling implementation
    }
}
```

**Features**:
- Configurable tick rate for application updates
- Error handling integration with Groundhog error system
- Blocking and non-blocking event polling

### Event Polling Implementation
```rust
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
```

## Key Event Handling

### Current Implementation
```rust
// In app.rs event loop
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
```

### Key Mappings
| Key | Action | Description |
|-----|--------|-------------|
| `q` | Quit | Exit the application |
| `Space` | Increment | Increase counter by 1 |
| `r` | Reset | Reset counter to 0 |

## Application Event Loop

### Main Loop Structure
```rust
async fn run_loop(&mut self, terminal: &mut Terminal<CrosstermBackend<io::Stdout>>) -> Result<(), GroundhogError> {
    loop {
        // 1. Draw the UI
        terminal
            .draw(|f| ui::render(f, self))
            .map_err(|e| GroundhogError::TUIError(e.to_string()))?;

        // 2. Handle events
        if event::poll(std::time::Duration::from_millis(100))
            .map_err(|e| GroundhogError::TUIError(e.to_string()))?
        {
            // Process event
        }

        // 3. Check exit condition
        if self.should_quit {
            break;
        }
    }

    Ok(())
}
```

**Loop Phases**:
1. **Render**: Draw current UI state
2. **Poll**: Check for events with timeout
3. **Process**: Handle any received events
4. **Update**: Modify application state
5. **Check**: Evaluate exit conditions

## Event Processing Patterns

### State Updates
```rust
KeyCode::Char(' ') => {
    self.counter += 1;
    self.message = format!("Counter: {} (Press 'q' to quit, Space to increment)", self.counter);
}
```

**Pattern**:
1. Modify application state
2. Update related UI elements
3. Maintain consistency across components

### Immediate Response
```rust
KeyCode::Char('q') => {
    self.should_quit = true;
}
```

**Pattern**:
- Set flags for immediate action
- Exit loop on next iteration
- Clean shutdown process

### Complex State Changes
```rust
KeyCode::Char('r') => {
    self.counter = 0;
    self.message = "Counter reset! 🐹".to_string();
}
```

**Pattern**:
- Multiple state modifications
- Coordinated UI updates
- Consistent user feedback

## Timing and Performance

### Tick Rate Configuration
```rust
impl Default for EventHandler {
    fn default() -> Self {
        Self::new(Duration::from_millis(100))
    }
}
```

**Considerations**:
- **100ms default**: Good balance of responsiveness and CPU usage
- **Configurable**: Can be adjusted based on application needs
- **Non-blocking**: UI remains responsive during processing

### Event Polling
```rust
if event::poll(self.tick_rate).map_err(|e| GroundhogError::TUIError(e.to_string()))? {
    // Process available events
} else {
    // Return tick event for regular updates
    Ok(Event::Tick)
}
```

**Benefits**:
- **Responsive**: Immediate reaction to user input
- **Efficient**: CPU-friendly polling with timeout
- **Reliable**: Consistent update cycle

## Error Handling

### Event System Errors
```rust
.map_err(|e| GroundhogError::TUIError(e.to_string()))?
```

**Error Sources**:
- Terminal I/O errors
- Event reading failures
- System-level interruptions

### Graceful Degradation
```rust
// Continue operation on non-critical errors
match event::read() {
    Ok(event) => process_event(event),
    Err(e) => {
        warn!("Event read error: {}", e);
        // Continue with tick event
    }
}
```

## Future Enhancements

### Mouse Support
```rust
CrosstermEvent::Mouse(mouse_event) => {
    match mouse_event.kind {
        MouseEventKind::Down(MouseButton::Left) => {
            // Handle left click
        }
        MouseEventKind::Scroll(ScrollDirection::Up) => {
            // Handle scroll up
        }
        _ => {}
    }
    Ok(Event::Mouse)
}
```

### Advanced Key Combinations
```rust
KeyEvent {
    code: KeyCode::Char('c'),
    modifiers: KeyModifiers::CONTROL,
    ..
} => {
    // Handle Ctrl+C
}
```

### Custom Events
```rust
pub enum Event {
    Key(KeyEvent),
    Tick,
    Resize(u16, u16),
    Mouse,
    Custom(CustomEvent),  // Application-specific events
}

pub enum CustomEvent {
    FileLoaded(PathBuf),
    ProcessComplete(String),
    NetworkUpdate(NetworkStatus),
}
```

## Best Practices

### Event Handling
1. **Immediate Response**: Process UI events immediately
2. **State Consistency**: Ensure all related state is updated together
3. **Error Recovery**: Handle errors gracefully without crashing

### Performance
1. **Efficient Polling**: Use appropriate tick rates
2. **Minimal Processing**: Keep event handlers lightweight
3. **Async Operations**: Use background tasks for heavy work

### User Experience
1. **Visual Feedback**: Provide immediate visual response
2. **Consistent Behavior**: Maintain predictable key mappings
3. **Clear Actions**: Make event effects obvious to users

## Integration with Application State

### State Management
```rust
pub struct App {
    pub should_quit: bool,
    pub message: String,
    pub counter: u32,
}
```

**Principles**:
- Events modify application state
- UI renders based on current state
- State changes trigger UI updates

### Event-Driven Updates
```rust
// Event triggers state change
self.counter += 1;

// State change triggers UI update
self.message = format!("Counter: {}", self.counter);

// UI reflects new state on next render
```

This creates a clean separation between event handling, state management, and UI rendering. 