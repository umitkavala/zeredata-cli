use ratatui::{
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, Paragraph},
    Frame,
};

#[derive(Debug, Clone)]
pub enum WizardStep {
    JobName,
    NumScenes,
    ConfigFile,
    Confirm,
}

pub struct JobWizard {
    pub step: WizardStep,
    pub job_name: String,
    pub num_scenes: String,
    pub config_file: String,
    pub cursor_position: usize,
}

impl JobWizard {
    pub fn new() -> Self {
        Self {
            step: WizardStep::JobName,
            job_name: String::new(),
            num_scenes: String::from("100"),
            config_file: String::new(),
            cursor_position: 0,
        }
    }

    pub fn next_step(&mut self) {
        self.step = match self.step {
            WizardStep::JobName => WizardStep::NumScenes,
            WizardStep::NumScenes => WizardStep::ConfigFile,
            WizardStep::ConfigFile => WizardStep::Confirm,
            WizardStep::Confirm => WizardStep::Confirm,
        };
    }

    pub fn previous_step(&mut self) {
        self.step = match self.step {
            WizardStep::JobName => WizardStep::JobName,
            WizardStep::NumScenes => WizardStep::JobName,
            WizardStep::ConfigFile => WizardStep::NumScenes,
            WizardStep::Confirm => WizardStep::ConfigFile,
        };
    }

    pub fn handle_char(&mut self, c: char) {
        let current_field = match self.step {
            WizardStep::JobName => &mut self.job_name,
            WizardStep::NumScenes => &mut self.num_scenes,
            WizardStep::ConfigFile => &mut self.config_file,
            WizardStep::Confirm => return,
        };

        current_field.insert(self.cursor_position, c);
        self.cursor_position += 1;
    }

    pub fn handle_backspace(&mut self) {
        if self.cursor_position > 0 {
            let current_field = match self.step {
                WizardStep::JobName => &mut self.job_name,
                WizardStep::NumScenes => &mut self.num_scenes,
                WizardStep::ConfigFile => &mut self.config_file,
                WizardStep::Confirm => return,
            };

            current_field.remove(self.cursor_position - 1);
            self.cursor_position -= 1;
        }
    }

    pub fn reset(&mut self) {
        self.step = WizardStep::JobName;
        self.job_name.clear();
        self.num_scenes = String::from("100");
        self.config_file.clear();
        self.cursor_position = 0;
    }

    pub fn is_valid(&self) -> bool {
        !self.job_name.is_empty() && self.num_scenes.parse::<i32>().is_ok()
    }

    pub fn render(&self, f: &mut Frame, area: Rect) {
        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .margin(2)
            .constraints([
                Constraint::Length(3),  // Title
                Constraint::Length(5),  // Progress
                Constraint::Min(8),     // Current step
                Constraint::Length(3),  // Instructions
            ])
            .split(area);

        // Title
        let title = Paragraph::new("Create New Job")
            .block(Block::default().borders(Borders::ALL))
            .style(Style::default().fg(Color::Cyan).add_modifier(Modifier::BOLD));
        f.render_widget(title, chunks[0]);

        // Progress indicator
        let step_num = match self.step {
            WizardStep::JobName => 1,
            WizardStep::NumScenes => 2,
            WizardStep::ConfigFile => 3,
            WizardStep::Confirm => 4,
        };

        let progress_text = vec![
            Line::from(vec![
                self.step_indicator(1, step_num, "Job Name"),
                Span::raw(" → "),
                self.step_indicator(2, step_num, "Scenes"),
                Span::raw(" → "),
                self.step_indicator(3, step_num, "Config"),
                Span::raw(" → "),
                self.step_indicator(4, step_num, "Confirm"),
            ])
        ];

        let progress = Paragraph::new(progress_text)
            .block(Block::default().borders(Borders::ALL).title("Progress"));
        f.render_widget(progress, chunks[1]);

        // Current step content
        match self.step {
            WizardStep::JobName => self.render_job_name_step(f, chunks[2]),
            WizardStep::NumScenes => self.render_num_scenes_step(f, chunks[2]),
            WizardStep::ConfigFile => self.render_config_file_step(f, chunks[2]),
            WizardStep::Confirm => self.render_confirm_step(f, chunks[2]),
        }

        // Instructions
        let instructions = match self.step {
            WizardStep::Confirm => {
                vec![Line::from(vec![
                    Span::styled("Enter", Style::default().fg(Color::Green).add_modifier(Modifier::BOLD)),
                    Span::raw(": Create Job | "),
                    Span::styled("Backspace", Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD)),
                    Span::raw(": Previous | "),
                    Span::styled("Esc", Style::default().fg(Color::Red).add_modifier(Modifier::BOLD)),
                    Span::raw(": Cancel"),
                ])]
            }
            _ => {
                vec![Line::from(vec![
                    Span::styled("Tab", Style::default().fg(Color::Green).add_modifier(Modifier::BOLD)),
                    Span::raw(": Next | "),
                    Span::styled("Shift+Tab", Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD)),
                    Span::raw(": Previous | "),
                    Span::styled("Esc", Style::default().fg(Color::Red).add_modifier(Modifier::BOLD)),
                    Span::raw(": Cancel"),
                ])]
            }
        };

        let help = Paragraph::new(instructions)
            .block(Block::default().borders(Borders::ALL));
        f.render_widget(help, chunks[3]);
    }

    fn step_indicator(&self, step: u8, current: u8, label: &str) -> Span {
        let style = if step == current {
            Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD)
        } else if step < current {
            Style::default().fg(Color::Green)
        } else {
            Style::default().fg(Color::Gray)
        };

        Span::styled(format!("{}", label), style)
    }

    fn render_job_name_step(&self, f: &mut Frame, area: Rect) {
        let text = vec![
            Line::from(""),
            Line::from(vec![
                Span::styled("Job Name:", Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD)),
            ]),
            Line::from(""),
            Line::from(vec![
                Span::styled(&self.job_name, Style::default().fg(Color::White)),
                Span::styled("_", Style::default().fg(Color::Yellow)),
            ]),
            Line::from(""),
            Line::from(vec![
                Span::styled("Enter a descriptive name for your rendering job", Style::default().fg(Color::Gray)),
            ]),
        ];

        let content = Paragraph::new(text)
            .block(Block::default().borders(Borders::ALL).title("Step 1: Job Name"));
        f.render_widget(content, area);
    }

    fn render_num_scenes_step(&self, f: &mut Frame, area: Rect) {
        let is_valid = self.num_scenes.parse::<i32>().is_ok();
        let validation_msg = if !is_valid && !self.num_scenes.is_empty() {
            Span::styled("⚠ Please enter a valid number", Style::default().fg(Color::Red))
        } else {
            Span::styled("Number of scenes to generate", Style::default().fg(Color::Gray))
        };

        let text = vec![
            Line::from(""),
            Line::from(vec![
                Span::styled("Number of Scenes:", Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD)),
            ]),
            Line::from(""),
            Line::from(vec![
                Span::styled(&self.num_scenes, Style::default().fg(Color::White)),
                Span::styled("_", Style::default().fg(Color::Yellow)),
            ]),
            Line::from(""),
            Line::from(vec![validation_msg]),
        ];

        let content = Paragraph::new(text)
            .block(Block::default().borders(Borders::ALL).title("Step 2: Number of Scenes"));
        f.render_widget(content, area);
    }

    fn render_config_file_step(&self, f: &mut Frame, area: Rect) {
        let text = vec![
            Line::from(""),
            Line::from(vec![
                Span::styled("Config File (Optional):", Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD)),
            ]),
            Line::from(""),
            Line::from(vec![
                Span::styled(&self.config_file, Style::default().fg(Color::White)),
                Span::styled("_", Style::default().fg(Color::Yellow)),
            ]),
            Line::from(""),
            Line::from(vec![
                Span::styled("Path to YAML config file (leave empty for defaults)", Style::default().fg(Color::Gray)),
            ]),
        ];

        let content = Paragraph::new(text)
            .block(Block::default().borders(Borders::ALL).title("Step 3: Configuration"));
        f.render_widget(content, area);
    }

    fn render_confirm_step(&self, f: &mut Frame, area: Rect) {
        let num_scenes_val = self.num_scenes.parse::<i32>().unwrap_or(0);

        let text = vec![
            Line::from(""),
            Line::from(vec![
                Span::styled("Review Your Job:", Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD)),
            ]),
            Line::from(""),
            Line::from(vec![
                Span::styled("  Name: ", Style::default().fg(Color::Gray)),
                Span::styled(&self.job_name, Style::default().fg(Color::White).add_modifier(Modifier::BOLD)),
            ]),
            Line::from(vec![
                Span::styled("  Scenes: ", Style::default().fg(Color::Gray)),
                Span::styled(format!("{}", num_scenes_val), Style::default().fg(Color::White).add_modifier(Modifier::BOLD)),
            ]),
            Line::from(vec![
                Span::styled("  Config: ", Style::default().fg(Color::Gray)),
                Span::styled(
                    if self.config_file.is_empty() { "Default" } else { &self.config_file },
                    Style::default().fg(Color::White).add_modifier(Modifier::BOLD)
                ),
            ]),
            Line::from(""),
            Line::from(vec![
                Span::styled("Press Enter to create this job", Style::default().fg(Color::Green)),
            ]),
        ];

        let content = Paragraph::new(text)
            .block(Block::default().borders(Borders::ALL).title("Step 4: Confirm"));
        f.render_widget(content, area);
    }
}
