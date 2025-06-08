# Paragraph Widget Specification

## Overview

The Paragraph widget is a fundamental component in the Groundhog TUI, used for displaying text content with formatting, styling, and layout control.

## Usage in Groundhog TUI

### Header Section
```rust
let header = Paragraph::new("🐹 Groundhog TUI - Hello World Demo")
    .style(Style::default().fg(Color::Yellow))
    .alignment(Alignment::Center)
    .block(
        Block::default()
            .borders(Borders::ALL)
            .title("Groundhog AI Assistant")
            .title_style(Style::default().fg(Color::Cyan))
    );
```

**Purpose**: Displays the main application title and branding
**Features**:
- Centered alignment
- Yellow text color
- Cyan title border
- Full border decoration

### Message Display Area
```rust
let message = Paragraph::new(message_lines)
    .style(Style::default().fg(Color::White))
    .block(
        Block::default()
            .borders(Borders::ALL)
            .title("Message Display")
            .title_style(Style::default().fg(Color::Magenta))
            .padding(Padding::uniform(1))
    )
    .wrap(Wrap { trim: true });
```

**Purpose**: Shows dynamic messages and instructions
**Features**:
- Multi-line text support with `Line` and `Span` components
- Text wrapping with trimming
- Uniform padding for better readability
- Magenta title styling
- White text content

### Counter Display
```rust
let counter = Paragraph::new(counter_text)
    .style(Style::default())
    .alignment(Alignment::Center)
    .block(
        Block::default()
            .borders(Borders::ALL)
            .title("Counter")
            .title_style(Style::default().fg(Color::Red))
    );
```

**Purpose**: Displays the current counter value
**Features**:
- Centered text alignment
- Red title border
- Styled counter value with Blue/Yellow color scheme

### Instructions Footer
```rust
let instructions = Paragraph::new("Controls: [Space] Increment | [R] Reset | [Q] Quit")
    .style(Style::default().fg(Color::Gray))
    .alignment(Alignment::Center)
    .block(
        Block::default()
            .borders(Borders::ALL)
            .title("Instructions")
    );
```

**Purpose**: Provides user control instructions
**Features**:
- Gray text for subtle appearance
- Centered alignment
- Clear control descriptions

## Text Composition

### Using Line and Span
```rust
let message_lines = vec![
    Line::from(vec![
        Span::styled("Message: ", Style::default().fg(Color::Green)),
        Span::raw(&app.message),
    ]),
    Line::from(""),
    Line::from("This is a basic ratatui demonstration."),
    // ... more lines
];
```

**Benefits**:
- **Mixed Styling**: Different parts of a line can have different colors/styles
- **Dynamic Content**: Combine static and dynamic text seamlessly
- **Structured Layout**: Organize content with empty lines and sections

## Styling Options

### Colors Used
- `Color::Yellow`: Header text
- `Color::Cyan`: Header title
- `Color::White`: Main content text
- `Color::Green`: Labels and highlights
- `Color::Magenta`: Section titles
- `Color::Red`: Counter title
- `Color::Blue`: Counter labels
- `Color::Gray`: Subtle instructions

### Alignment Options
- `Alignment::Center`: Headers, counters, instructions
- `Alignment::Left`: Default for content (implicit)

## Block Configuration

### Border Styles
All paragraphs use `Borders::ALL` for consistent visual separation.

### Padding
Message display uses `Padding::uniform(1)` for better text readability.

### Titles
Each section has descriptive titles with custom styling to create visual hierarchy.

## Text Wrapping

The message display area uses:
```rust
.wrap(Wrap { trim: true })
```

This ensures:
- Long text wraps to fit the available width
- Leading/trailing whitespace is trimmed
- Content remains readable in narrow terminals

## Best Practices

### Content Organization
1. Use `Line::from("")` for spacing between sections
2. Combine `Span::styled()` and `Span::raw()` for mixed formatting
3. Keep titles concise and descriptive

### Color Consistency
1. Use consistent colors for similar types of content
2. Maintain sufficient contrast for readability
3. Use subtle colors (like Gray) for less important information

### Layout Considerations
1. Center-align headers and summaries
2. Left-align detailed content (default)
3. Use padding for dense text areas

## Integration with Layout

Paragraphs work seamlessly with ratatui's Layout system:
```rust
let chunks = Layout::default()
    .direction(Direction::Vertical)
    .constraints([
        Constraint::Length(3),   // Header paragraph
        Constraint::Min(7),      // Content paragraphs
        Constraint::Length(3),   // Instructions paragraph
    ])
    .split(frame.area());
```

This creates a responsive layout where paragraphs adapt to available space while maintaining minimum requirements. 