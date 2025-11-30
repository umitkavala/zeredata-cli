use ratatui::{
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    text::Span,
    widgets::{Block, Borders, Cell, Row, Table},
    Frame,
};

use crate::tui::app::App;

pub fn render(f: &mut Frame, app: &App, area: Rect) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(3),  // Search box
            Constraint::Min(0),     // Table
        ])
        .split(area);

    // Render search box
    app.search_box.render(f, chunks[0]);

    // Render table in second chunk
    render_table(f, app, chunks[1]);
}

fn render_table(f: &mut Frame, app: &App, area: Rect) {
    let header_cells = ["ID", "Name", "Status", "Scenes", "Progress"]
        .iter()
        .map(|h| Cell::from(*h).style(Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD)));

    let header = Row::new(header_cells).height(1).bottom_margin(1);

    let filtered_jobs = app.filtered_jobs();
    let rows = filtered_jobs.iter().enumerate().map(|(i, job)| {
        let status_color = match job.status.as_str() {
            "running" => Color::Yellow,
            "completed" => Color::Green,
            "failed" => Color::Red,
            "queued" => Color::Blue,
            _ => Color::Gray,
        };

        let cells = vec![
            Cell::from(job.id.to_string()),
            Cell::from(job.name.clone()),
            Cell::from(Span::styled(
                job.status.clone(),
                Style::default().fg(status_color).add_modifier(Modifier::BOLD),
            )),
            Cell::from(job.num_scenes.to_string()),
            Cell::from(format!("{}%", job.progress.unwrap_or(0))),
        ];

        let mut row = Row::new(cells).height(1);

        // Highlight selected row
        if i == app.selected_job_index {
            row = row.style(Style::default().bg(Color::DarkGray));
        }

        row
    });

    let widths = [
        Constraint::Length(6),
        Constraint::Min(20),
        Constraint::Length(12),
        Constraint::Length(8),
        Constraint::Length(10),
    ];

    let title = if app.search_box.query.is_empty() {
        format!("Jobs ({} total)", app.jobs.len())
    } else {
        format!("Jobs ({} of {} total)", filtered_jobs.len(), app.jobs.len())
    };

    let table = Table::new(rows, widths)
        .header(header)
        .block(Block::default().borders(Borders::ALL).title(title))
        .column_spacing(1)
        .highlight_style(Style::default().add_modifier(Modifier::BOLD))
        .highlight_symbol(">> ");

    f.render_widget(table, area);
}
