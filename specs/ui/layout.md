# Layout System Specification

## Overview

The Layout system in ratatui provides a flexible and responsive way to organize UI components within the terminal space. The Groundhog TUI uses a hierarchical layout structure to create a well-organized interface.

## Main Layout Structure

### Primary Layout
```rust
pub fn render(frame: &mut Frame, app: &App) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .margin(1)
        .constraints([
            Constraint::Length(3),   // Header
            Constraint::Min(7),      // Main content
            Constraint::Length(3),   // Instructions
        ])
        .split(frame.area());

    render_header(frame, chunks[0]);
    render_main_content(frame, app, chunks[1]);
    render_instructions(frame, chunks[2]);
}
```

**Structure**:
- **Vertical Direction**: Stacks components top to bottom
- **Margin**: 1-character margin around the entire interface
- **Three Sections**: Header, main content, and instructions

### Layout Constraints

#### Constraint Types Used
```rust
Constraint::Length(3)    // Fixed height of 3 lines
Constraint::Min(7)       // Minimum 7 lines, expands to fill space
```

**Benefits**:
- **Fixed Header/Footer**: Consistent positioning regardless of terminal size
- **Flexible Content**: Main area adapts to available space
- **Responsive Design**: Works well on different terminal sizes

## Secondary Layout (Main Content)

### Horizontal Split
```rust
fn render_main_content(frame: &mut Frame, app: &App, area: ratatui::layout::Rect) {
    let main_chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Percentage(60),  // Message area
            Constraint::Percentage(40),  // Counter and status
        ])
        .split(area);

    render_message_area(frame, app, main_chunks[0]);
    render_status_area(frame, app, main_chunks[1]);
}
```

**Structure**:
- **Horizontal Direction**: Side-by-side panels
- **60/40 Split**: Message area gets more space than status area
- **Percentage Constraints**: Proportional sizing

## Tertiary Layout (Status Area)

### Vertical Status Panel
```rust
fn render_status_area(frame: &mut Frame, app: &App, area: ratatui::layout::Rect) {
    let status_chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(5),   // Counter display
            Constraint::Length(3),   // Progress gauge
            Constraint::Min(3),      // Status info
        ])
        .split(area);

    render_counter_display(frame, app, status_chunks[0]);
    render_progress_gauge(frame, app, status_chunks[1]);
    render_status_info(frame, status_chunks[2]);
}
```

**Structure**:
- **Vertical Direction**: Stacked status components
- **Mixed Constraints**: Fixed sizes for counter/gauge, flexible for status list
- **Hierarchical Organization**: Nested within the main content area

## Layout Hierarchy

```
Terminal Area
├── Margin (1 char all sides)
└── Main Layout (Vertical)
    ├── Header (3 lines, fixed)
    ├── Main Content (7+ lines, flexible)
    │   ├── Message Area (60%, horizontal)
    │   └── Status Area (40%, horizontal)
    │       ├── Counter (5 lines, fixed)
    │       ├── Progress (3 lines, fixed)
    │       └── Status List (3+ lines, flexible)
    └── Instructions (3 lines, fixed)
```

## Constraint Types and Usage

### Length Constraints
```rust
Constraint::Length(3)    // Exactly 3 lines
Constraint::Length(5)    // Exactly 5 lines
```

**Use Cases**:
- Headers and footers
- Fixed-size widgets (counter, progress bar)
- Consistent spacing

### Minimum Constraints
```rust
Constraint::Min(7)       // At least 7 lines
Constraint::Min(3)       // At least 3 lines
```

**Use Cases**:
- Main content areas that should expand
- Lists that need minimum space but can grow
- Flexible panels

### Percentage Constraints
```rust
Constraint::Percentage(60)   // 60% of available space
Constraint::Percentage(40)   // 40% of available space
```

**Use Cases**:
- Proportional splits
- Responsive design
- Balanced layouts

## Responsive Behavior

### Terminal Resizing
The layout system automatically handles terminal resizing:

1. **Fixed Elements**: Headers, footers, and fixed widgets maintain their size
2. **Flexible Elements**: Main content and lists expand/contract
3. **Proportional Elements**: Percentage-based splits maintain ratios

### Minimum Size Handling
```rust
// Ensures minimum usable space
Constraint::Min(7)       // Main content needs at least 7 lines
Constraint::Min(3)       // Status list needs at least 3 lines
```

## Layout Best Practices

### Constraint Selection
1. **Fixed for UI Chrome**: Use `Length` for headers, footers, toolbars
2. **Flexible for Content**: Use `Min` for content areas that should expand
3. **Proportional for Balance**: Use `Percentage` for balanced splits

### Nesting Guidelines
1. **Logical Hierarchy**: Nest layouts to match UI structure
2. **Direction Alternation**: Alternate between vertical and horizontal
3. **Constraint Consistency**: Use consistent constraint types at each level

### Margin and Spacing
```rust
.margin(1)               // Outer margin for breathing room
Padding::uniform(1)      // Inner padding for text widgets
```

## Advanced Layout Patterns

### Conditional Layouts
```rust
fn adaptive_layout(terminal_width: u16) -> Vec<Constraint> {
    if terminal_width < 80 {
        // Narrow terminal: stack vertically
        vec![Constraint::Percentage(100)]
    } else {
        // Wide terminal: side-by-side
        vec![
            Constraint::Percentage(60),
            Constraint::Percentage(40),
        ]
    }
}
```

### Dynamic Constraints
```rust
fn dynamic_status_layout(item_count: usize) -> Vec<Constraint> {
    let list_height = (item_count + 2).min(10); // +2 for borders, max 10
    vec![
        Constraint::Length(5),                    // Counter
        Constraint::Length(3),                    // Progress
        Constraint::Length(list_height as u16),   // Dynamic list
    ]
}
```

## Integration with Widgets

### Widget Sizing
Widgets automatically adapt to their allocated area:
```rust
// Widget receives the area from layout
frame.render_widget(widget, chunks[0]);
```

### Border Considerations
```rust
// Borders consume space within the allocated area
Block::default().borders(Borders::ALL)  // Uses 2 lines (top+bottom), 2 cols (left+right)
```

### Content Overflow
```rust
// Text wrapping handles content overflow
Paragraph::new(text).wrap(Wrap { trim: true })
```

## Performance Considerations

### Layout Calculation
- Layout calculation is O(n) where n is the number of constraints
- Minimal performance impact for typical UI complexity
- Caching not necessary for most applications

### Memory Usage
- Layout structures are lightweight
- No significant memory overhead
- Temporary allocation during split operations

## Common Layout Patterns

### Three-Panel Layout
```rust
// Header, content, footer
vec![
    Constraint::Length(3),
    Constraint::Min(1),
    Constraint::Length(3),
]
```

### Sidebar Layout
```rust
// Sidebar, main content
vec![
    Constraint::Length(20),
    Constraint::Min(1),
]
```

### Dashboard Layout
```rust
// Multiple equal panels
vec![
    Constraint::Percentage(25),
    Constraint::Percentage(25),
    Constraint::Percentage(25),
    Constraint::Percentage(25),
]
```

## Debugging Layout Issues

### Common Problems
1. **Insufficient Space**: Use `Min` constraints with reasonable minimums
2. **Overflow**: Ensure total percentages don't exceed 100%
3. **Empty Areas**: Check constraint calculations and widget rendering

### Debug Techniques
```rust
// Log area dimensions
tracing::debug!("Area: {:?}", area);

// Render debug borders
Block::default()
    .borders(Borders::ALL)
    .title(&format!("{}x{}", area.width, area.height))
``` 