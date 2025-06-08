use ratatui::{
    layout::{Alignment, Constraint, Direction, Layout},
    style::{Color, Style},
    text::{Line, Span},
    widgets::{Block, Borders, Gauge, List, ListItem, Padding, Paragraph, Wrap},
    Frame,
};

use super::app::App;

/// Render the main UI
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

/// Render the header section
fn render_header(frame: &mut Frame, area: ratatui::layout::Rect) {
    let header = Paragraph::new("🐹 Groundhog TUI - Hello World Demo")
        .style(Style::default().fg(Color::Yellow))
        .alignment(Alignment::Center)
        .block(
            Block::default()
                .borders(Borders::ALL)
                .title("Groundhog AI Assistant")
                .title_style(Style::default().fg(Color::Cyan))
        );
    frame.render_widget(header, area);
}

/// Render the main content area
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

/// Render the message display area
fn render_message_area(frame: &mut Frame, app: &App, area: ratatui::layout::Rect) {
    let message_lines = vec![
        Line::from(vec![
            Span::styled("Message: ", Style::default().fg(Color::Green)),
            Span::raw(&app.message),
        ]),
        Line::from(""),
        Line::from("This is a basic ratatui demonstration."),
        Line::from("Press Space to increment the counter."),
        Line::from("Press 'r' to reset the counter."),
        Line::from("Press 'q' to quit the application."),
    ];

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

    frame.render_widget(message, area);
}

/// Render the status and counter area
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

/// Render the counter display
fn render_counter_display(frame: &mut Frame, app: &App, area: ratatui::layout::Rect) {
    let counter_text = vec![
        Line::from(vec![
            Span::styled("Count: ", Style::default().fg(Color::Blue)),
            Span::styled(
                format!("{}", app.counter),
                Style::default().fg(Color::Yellow)
            ),
        ]),
    ];

    let counter = Paragraph::new(counter_text)
        .style(Style::default())
        .alignment(Alignment::Center)
        .block(
            Block::default()
                .borders(Borders::ALL)
                .title("Counter")
                .title_style(Style::default().fg(Color::Red))
        );

    frame.render_widget(counter, area);
}

/// Render a progress gauge based on counter
fn render_progress_gauge(frame: &mut Frame, app: &App, area: ratatui::layout::Rect) {
    let progress = (app.counter % 100) as f64 / 100.0;
    let gauge = Gauge::default()
        .block(Block::default().borders(Borders::ALL).title("Progress"))
        .gauge_style(Style::default().fg(Color::Green))
        .ratio(progress);

    frame.render_widget(gauge, area);
}

/// Render status information
fn render_status_info(frame: &mut Frame, area: ratatui::layout::Rect) {
    let status_items = vec![
        ListItem::new("✓ TUI Active"),
        ListItem::new("✓ Input Handling"),
        ListItem::new("✓ Real-time Updates"),
    ];

    let status_list = List::new(status_items)
        .block(
            Block::default()
                .borders(Borders::ALL)
                .title("Status")
                .title_style(Style::default().fg(Color::Green))
        )
        .style(Style::default().fg(Color::White));

    frame.render_widget(status_list, area);
}

/// Render instructions footer
fn render_instructions(frame: &mut Frame, area: ratatui::layout::Rect) {
    let instructions = Paragraph::new("Controls: [Space] Increment | [R] Reset | [Q] Quit")
        .style(Style::default().fg(Color::Gray))
        .alignment(Alignment::Center)
        .block(
            Block::default()
                .borders(Borders::ALL)
                .title("Instructions")
        );

    frame.render_widget(instructions, area);
} 