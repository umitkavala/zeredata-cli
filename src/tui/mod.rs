pub mod app;
pub mod ui;
pub mod views;
pub mod components;

pub use app::{App, AppState};

use crate::error::Result;
use crossterm::{
    event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode, KeyEventKind},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use ratatui::{
    backend::{Backend, CrosstermBackend},
    Terminal,
};
use std::io;

/// Run the TUI application
pub async fn run() -> Result<()> {
    // Setup terminal
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    // Create app state
    let mut app = App::new().await?;

    // Run app
    let res = run_app(&mut terminal, &mut app).await;

    // Restore terminal
    disable_raw_mode()?;
    execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen,
        DisableMouseCapture
    )?;
    terminal.show_cursor()?;

    if let Err(err) = res {
        println!("Error: {:?}", err);
    }

    Ok(())
}

async fn run_app<B: Backend>(terminal: &mut Terminal<B>, app: &mut App) -> Result<()> {
    loop {
        terminal.draw(|f| ui::draw(f, app))?;

        if event::poll(std::time::Duration::from_millis(100))? {
            if let Event::Key(key) = event::read()? {
                if key.kind == KeyEventKind::Press {
                    // Handle wizard mode separately
                    if app.state == app::AppState::CreateJob {
                        match key.code {
                            KeyCode::Esc => {
                                app.cancel_create_job();
                            }
                            KeyCode::Tab => {
                                app.job_wizard.next_step();
                            }
                            KeyCode::BackTab => {
                                app.job_wizard.previous_step();
                            }
                            KeyCode::Enter => {
                                app.submit_job().await?;
                            }
                            KeyCode::Char(c) => {
                                app.job_wizard.handle_char(c);
                            }
                            KeyCode::Backspace => {
                                app.job_wizard.handle_backspace();
                            }
                            _ => {}
                        }
                        continue;
                    }

                    // Handle search mode
                    if app.search_box.active {
                        match key.code {
                            KeyCode::Esc => {
                                app.clear_search();
                            }
                            KeyCode::Enter => {
                                app.search_box.deactivate();
                            }
                            KeyCode::Char(c) => {
                                app.search_box.handle_char(c);
                            }
                            KeyCode::Backspace => {
                                app.search_box.handle_backspace();
                            }
                            _ => {}
                        }
                        continue;
                    }

                    // Clear status message on any key press
                    if app.status_message.is_some() {
                        app.clear_status_message();
                        continue;
                    }

                    // Normal mode key handling
                    match key.code {
                        KeyCode::Char('q') => return Ok(()),
                        KeyCode::Char('r') => {
                            app.refresh().await?;
                        }
                        KeyCode::Char('c') => {
                            app.start_create_job();
                        }
                        KeyCode::Char('/') => {
                            app.toggle_search();
                        }
                        KeyCode::Tab => {
                            app.next_view();
                        }
                        KeyCode::BackTab => {
                            app.previous_view();
                        }
                        KeyCode::Down | KeyCode::Char('j') => {
                            app.on_down();
                        }
                        KeyCode::Up | KeyCode::Char('k') => {
                            app.on_up();
                        }
                        KeyCode::Enter => {
                            app.on_select().await?;
                        }
                        KeyCode::Char('?') => {
                            app.toggle_help();
                        }
                        _ => {}
                    }
                }
            }
        }

        // Auto-refresh every 5 seconds
        if app.should_refresh() {
            app.refresh().await?;
        }
    }
}
