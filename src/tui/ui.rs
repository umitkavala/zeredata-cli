use ratatui::{
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, Paragraph, Tabs},
    Frame,
};

use super::app::{App, AppState};

pub fn draw(f: &mut Frame, app: &App) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(3),  // Header
            Constraint::Min(0),      // Main content
            Constraint::Length(3),   // Footer
        ])
        .split(f.area());

    // Render header with tabs
    render_header(f, app, chunks[0]);

    // Render content based on current state
    if app.show_help {
        super::views::help::render(f, chunks[1]);
    } else {
        match app.state {
            AppState::Dashboard => super::views::dashboard::render(f, app, chunks[1]),
            AppState::Assets => super::views::assets::render(f, app, chunks[1]),
            AppState::Jobs => super::views::jobs::render(f, app, chunks[1]),
            AppState::Help => super::views::help::render(f, chunks[1]),
            AppState::CreateJob => app.job_wizard.render(f, chunks[1]),
        }
    }

    // Render status message if present
    if let Some(ref msg) = app.status_message {
        render_status_message(f, msg);
    }

    // Render footer with shortcuts
    render_footer(f, chunks[2]);
}

fn render_header(f: &mut Frame, app: &App, area: Rect) {
    let titles = vec!["Dashboard", "Jobs", "Assets", "Help"];
    let index = match app.state {
        AppState::Dashboard => 0,
        AppState::Jobs => 1,
        AppState::Assets => 2,
        AppState::Help => 3,
        AppState::CreateJob => 1, // Highlight Jobs tab when creating
    };

    let tabs = Tabs::new(titles)
        .block(Block::default().borders(Borders::ALL).title("Zere CLI"))
        .select(index)
        .style(Style::default().fg(Color::White))
        .highlight_style(
            Style::default()
                .fg(Color::Yellow)
                .add_modifier(Modifier::BOLD),
        );

    f.render_widget(tabs, area);
}

fn render_footer(f: &mut Frame, area: Rect) {
    let footer_text = vec![
        Line::from(vec![
            Span::styled("q", Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD)),
            Span::raw(": quit | "),
            Span::styled("Tab", Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD)),
            Span::raw(": next | "),
            Span::styled("r", Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD)),
            Span::raw(": refresh | "),
            Span::styled("c", Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD)),
            Span::raw(": create job | "),
            Span::styled("/", Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD)),
            Span::raw(": search | "),
            Span::styled("?", Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD)),
            Span::raw(": help"),
        ])
    ];

    let footer = Paragraph::new(footer_text)
        .block(Block::default().borders(Borders::ALL))
        .style(Style::default().fg(Color::Gray));

    f.render_widget(footer, area);
}

fn render_status_message(f: &mut Frame, msg: &str) {
    let area = f.area();
    let popup_area = Rect {
        x: area.width / 4,
        y: area.height / 2 - 2,
        width: area.width / 2,
        height: 5,
    };

    let color = if msg.contains("success") || msg.contains("created") {
        Color::Green
    } else if msg.contains("Failed") || msg.contains("error") {
        Color::Red
    } else {
        Color::Yellow
    };

    let text = vec![
        Line::from(""),
        Line::from(vec![
            Span::styled(msg, Style::default().fg(color).add_modifier(Modifier::BOLD)),
        ]),
    ];

    let popup = Paragraph::new(text)
        .block(Block::default()
            .borders(Borders::ALL)
            .border_style(Style::default().fg(color))
            .title("Status"))
        .style(Style::default().bg(Color::Black));

    f.render_widget(popup, popup_area);
}
