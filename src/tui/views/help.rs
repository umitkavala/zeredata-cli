use ratatui::{
    layout::Rect,
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, Paragraph},
    Frame,
};

pub fn render(f: &mut Frame, area: Rect) {
    let help_text = vec![
        Line::from(""),
        Line::from(vec![
            Span::styled("Zere CLI - Terminal UI Help", Style::default().fg(Color::Cyan).add_modifier(Modifier::BOLD)),
        ]),
        Line::from(""),
        Line::from(vec![
            Span::styled("Navigation:", Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD)),
        ]),
        Line::from(vec![
            Span::raw("  "),
            Span::styled("Tab", Style::default().fg(Color::Green)),
            Span::raw(" / "),
            Span::styled("Shift+Tab", Style::default().fg(Color::Green)),
            Span::raw(" - Switch between views"),
        ]),
        Line::from(vec![
            Span::raw("  "),
            Span::styled("↑/↓", Style::default().fg(Color::Green)),
            Span::raw(" or "),
            Span::styled("j/k", Style::default().fg(Color::Green)),
            Span::raw(" - Navigate lists"),
        ]),
        Line::from(vec![
            Span::raw("  "),
            Span::styled("Enter", Style::default().fg(Color::Green)),
            Span::raw(" - Select item (coming soon)"),
        ]),
        Line::from(""),
        Line::from(vec![
            Span::styled("Actions:", Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD)),
        ]),
        Line::from(vec![
            Span::raw("  "),
            Span::styled("r", Style::default().fg(Color::Green)),
            Span::raw(" - Refresh data from API"),
        ]),
        Line::from(vec![
            Span::raw("  "),
            Span::styled("c", Style::default().fg(Color::Green)),
            Span::raw(" - Create new job (launches wizard)"),
        ]),
        Line::from(vec![
            Span::raw("  "),
            Span::styled("/", Style::default().fg(Color::Green)),
            Span::raw(" - Toggle search mode (fuzzy search)"),
        ]),
        Line::from(vec![
            Span::raw("  "),
            Span::styled("?", Style::default().fg(Color::Green)),
            Span::raw(" - Toggle this help screen"),
        ]),
        Line::from(vec![
            Span::raw("  "),
            Span::styled("q", Style::default().fg(Color::Green)),
            Span::raw(" - Quit application"),
        ]),
        Line::from(""),
        Line::from(vec![
            Span::styled("Job Creation Wizard:", Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD)),
        ]),
        Line::from("  - Tab/Shift+Tab to navigate steps"),
        Line::from("  - Type to enter values"),
        Line::from("  - Enter to submit job"),
        Line::from("  - Esc to cancel"),
        Line::from(""),
        Line::from(vec![
            Span::styled("Search Mode:", Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD)),
        ]),
        Line::from("  - Type to filter results (fuzzy matching)"),
        Line::from("  - Results sorted by match score"),
        Line::from("  - Esc to clear and exit search"),
        Line::from(""),
        Line::from(vec![
            Span::styled("Views:", Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD)),
        ]),
        Line::from(vec![
            Span::raw("  "),
            Span::styled("Dashboard", Style::default().fg(Color::Cyan)),
            Span::raw(" - Overview of jobs and assets"),
        ]),
        Line::from(vec![
            Span::raw("  "),
            Span::styled("Jobs", Style::default().fg(Color::Cyan)),
            Span::raw(" - List of all rendering jobs"),
        ]),
        Line::from(vec![
            Span::raw("  "),
            Span::styled("Assets", Style::default().fg(Color::Cyan)),
            Span::raw(" - List of all uploaded assets"),
        ]),
        Line::from(""),
        Line::from(vec![
            Span::styled("Auto-refresh:", Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD)),
        ]),
        Line::from(vec![
            Span::raw("  Data automatically refreshes every "),
            Span::styled("5 seconds", Style::default().fg(Color::Green).add_modifier(Modifier::BOLD)),
        ]),
        Line::from(""),
        Line::from(vec![
            Span::styled("Coming Soon:", Style::default().fg(Color::Magenta).add_modifier(Modifier::BOLD)),
        ]),
        Line::from("  - Asset upload from TUI"),
        Line::from("  - Real-time job progress monitoring"),
        Line::from("  - Job cancellation"),
        Line::from("  - Advanced filtering options"),
        Line::from(""),
    ];

    let help = Paragraph::new(help_text)
        .block(Block::default().borders(Borders::ALL).title("Help (Press ? to close)"))
        .style(Style::default().fg(Color::White));

    f.render_widget(help, area);
}
