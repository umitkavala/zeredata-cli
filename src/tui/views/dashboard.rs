use ratatui::{
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, Gauge, Paragraph},
    Frame,
};

use crate::tui::app::App;

pub fn render(f: &mut Frame, app: &App, area: Rect) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .margin(2)
        .constraints([
            Constraint::Length(3),  // Title
            Constraint::Length(5),  // Stats
            Constraint::Length(5),  // Job counts
            Constraint::Min(0),     // Recent activity
        ])
        .split(area);

    // Title
    let title = Paragraph::new("Dashboard")
        .block(Block::default().borders(Borders::ALL))
        .style(Style::default().fg(Color::Cyan).add_modifier(Modifier::BOLD));
    f.render_widget(title, chunks[0]);

    // Stats
    let stats_text = vec![
        Line::from(vec![
            Span::styled("Total Jobs: ", Style::default().fg(Color::White)),
            Span::styled(
                app.jobs.len().to_string(),
                Style::default().fg(Color::Green).add_modifier(Modifier::BOLD),
            ),
            Span::raw("  "),
            Span::styled("Total Assets: ", Style::default().fg(Color::White)),
            Span::styled(
                app.assets.len().to_string(),
                Style::default().fg(Color::Green).add_modifier(Modifier::BOLD),
            ),
        ]),
    ];

    let stats = Paragraph::new(stats_text)
        .block(Block::default().borders(Borders::ALL).title("Statistics"));
    f.render_widget(stats, chunks[1]);

    // Job counts breakdown
    let running = app.running_jobs_count();
    let completed = app.completed_jobs_count();
    let queued = app.queued_jobs_count();
    let total = app.jobs.len();

    let completion_ratio = if total > 0 {
        (completed as f64 / total as f64) * 100.0
    } else {
        0.0
    };

    let job_counts = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(1),
            Constraint::Length(1),
            Constraint::Length(1),
        ])
        .split(chunks[2]);

    // Running jobs gauge
    let running_gauge = Gauge::default()
        .block(Block::default().borders(Borders::ALL).title("Running"))
        .gauge_style(Style::default().fg(Color::Yellow))
        .percent(if total > 0 { ((running as f64 / total as f64) * 100.0) as u16 } else { 0 })
        .label(format!("{}/{}", running, total));
    f.render_widget(running_gauge, job_counts[0]);

    // Queued jobs gauge
    let queued_gauge = Gauge::default()
        .block(Block::default().borders(Borders::ALL).title("Queued"))
        .gauge_style(Style::default().fg(Color::Blue))
        .percent(if total > 0 { ((queued as f64 / total as f64) * 100.0) as u16 } else { 0 })
        .label(format!("{}/{}", queued, total));
    f.render_widget(queued_gauge, job_counts[1]);

    // Completed jobs gauge
    let completed_gauge = Gauge::default()
        .block(Block::default().borders(Borders::ALL).title("Completed"))
        .gauge_style(Style::default().fg(Color::Green))
        .percent(completion_ratio as u16)
        .label(format!("{}/{} ({:.1}%)", completed, total, completion_ratio));
    f.render_widget(completed_gauge, job_counts[2]);

    // Recent activity
    let recent_activity = if app.jobs.is_empty() {
        vec![Line::from("No recent jobs")]
    } else {
        let recent_jobs: Vec<Line> = app.jobs.iter().take(5).map(|job| {
            let status_color = match job.status.as_str() {
                "running" => Color::Yellow,
                "completed" => Color::Green,
                "failed" => Color::Red,
                "queued" => Color::Blue,
                _ => Color::Gray,
            };

            Line::from(vec![
                Span::styled(
                    format!("{:12}", job.status),
                    Style::default().fg(status_color).add_modifier(Modifier::BOLD),
                ),
                Span::raw(" | "),
                Span::styled(&job.name, Style::default().fg(Color::White)),
                Span::raw(" | "),
                Span::styled(
                    format!("{} scenes", job.num_scenes),
                    Style::default().fg(Color::Gray),
                ),
            ])
        }).collect();

        recent_jobs
    };

    let activity = Paragraph::new(recent_activity)
        .block(Block::default().borders(Borders::ALL).title("Recent Jobs"));
    f.render_widget(activity, chunks[3]);
}
