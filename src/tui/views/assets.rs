use ratatui::{
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
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
    let header_cells = ["ID", "Asset ID", "Name", "Type", "Size"]
        .iter()
        .map(|h| Cell::from(*h).style(Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD)));

    let header = Row::new(header_cells).height(1).bottom_margin(1);

    let filtered_assets = app.filtered_assets();
    let rows = filtered_assets.iter().enumerate().map(|(i, asset)| {
        let size_mb = (asset.size_bytes as f64 / 1_048_576.0).round() as i64;
        let size_display = if size_mb > 0 {
            format!("{} MB", size_mb)
        } else {
            format!("{} KB", (asset.size_bytes as f64 / 1024.0).round() as i64)
        };

        let cells = vec![
            Cell::from(asset.id.to_string()),
            Cell::from(asset.asset_id.clone()),
            Cell::from(asset.name.clone()),
            Cell::from(asset.file_type.clone()),
            Cell::from(size_display),
        ];

        let mut row = Row::new(cells).height(1);

        // Highlight selected row
        if i == app.selected_asset_index {
            row = row.style(Style::default().bg(Color::DarkGray));
        }

        row
    });

    let widths = [
        Constraint::Length(6),
        Constraint::Length(15),
        Constraint::Min(20),
        Constraint::Length(15),
        Constraint::Length(12),
    ];

    let title = if app.search_box.query.is_empty() {
        format!("Assets ({} total)", app.assets.len())
    } else {
        format!("Assets ({} of {} total)", filtered_assets.len(), app.assets.len())
    };

    let table = Table::new(rows, widths)
        .header(header)
        .block(Block::default().borders(Borders::ALL).title(title))
        .column_spacing(1)
        .highlight_style(Style::default().add_modifier(Modifier::BOLD))
        .highlight_symbol(">> ");

    f.render_widget(table, area);
}
