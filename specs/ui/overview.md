# TUI (Terminal User Interface) Overview

## Introduction

The Groundhog TUI provides an interactive terminal-based interface built with [ratatui](https://ratatui.rs/), a modern Rust library for building rich terminal user interfaces.

## Architecture

The TUI system is organized into several key components:

```
src/tui/
├── mod.rs          # Module exports and main entry point
├── app.rs          # Application state and main event loop
├── ui.rs           # UI rendering and layout
└── event.rs        # Event handling system
```

## Key Features

### Interactive Hello World Demo
- **Welcome Message**: Displays a friendly greeting with the Groundhog mascot 🐹
- **Counter System**: Interactive counter that can be incremented and reset
- **Real-time Updates**: Live UI updates based on user input
- **Progress Visualization**: Visual progress gauge based on counter value

### User Controls
- **Space**: Increment the counter
- **R**: Reset the counter to zero
- **Q**: Quit the application

## UI Layout

The TUI uses a structured layout with the following sections:

1. **Header** (3 lines): Application title and branding
2. **Main Content** (flexible): Split into message area (60%) and status area (40%)
3. **Instructions** (3 lines): Control instructions for the user

### Main Content Areas

#### Message Display (Left Panel - 60%)
- Current message with dynamic content
- Usage instructions
- Descriptive text about the demo

#### Status Area (Right Panel - 40%)
- **Counter Display**: Shows current count value
- **Progress Gauge**: Visual representation of progress (0-100%)
- **Status List**: System status indicators

## Color Scheme

The TUI uses a carefully chosen color palette:

- **Header**: Yellow text with Cyan title
- **Message Area**: Green labels with White content, Magenta borders
- **Counter**: Blue labels with Yellow values, Red borders
- **Progress**: Green gauge
- **Status**: Green borders with White text
- **Instructions**: Gray text

## Technical Implementation

### Dependencies
- `ratatui = "0.28"`: Core TUI framework
- `crossterm = "0.28"`: Cross-platform terminal manipulation

### Key Components
- **App State**: Manages application state including quit flag, message, and counter
- **Event Loop**: Handles terminal events and user input
- **Rendering**: Modular rendering system with separate functions for each UI section
- **Error Handling**: Integrated with Groundhog's error system

## Usage

Launch the TUI with:
```bash
cargo run -- tui
```

Or with debug mode:
```bash
cargo run -- tui --debug
```

## Future Enhancements

The TUI system is designed for extensibility and can be enhanced with:

- Multiple screens/views
- Form input handling
- File browser integration
- AI interaction interface
- Configuration management UI
- Help system
- Themes and customization 