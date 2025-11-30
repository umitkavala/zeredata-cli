use crate::api::ApiClient;
use crate::api::assets::Asset;
use crate::api::jobs::Job;
use crate::config::Config;
use crate::error::Result;
use crate::tui::components::{JobWizard, SearchBox};
use std::time::{Duration, Instant};

#[derive(Debug, Clone, PartialEq)]
pub enum AppState {
    Dashboard,
    Assets,
    Jobs,
    Help,
    CreateJob,
}

pub struct App {
    pub state: AppState,
    pub client: ApiClient,
    pub jobs: Vec<Job>,
    pub assets: Vec<Asset>,
    pub selected_job_index: usize,
    pub selected_asset_index: usize,
    pub show_help: bool,
    pub job_wizard: JobWizard,
    pub search_box: SearchBox,
    pub status_message: Option<String>,
    last_refresh: Instant,
}

impl App {
    pub async fn new() -> Result<App> {
        let config = Config::load()?;
        let client = ApiClient::from_config(&config)?;

        let mut app = App {
            state: AppState::Dashboard,
            client,
            jobs: Vec::new(),
            assets: Vec::new(),
            selected_job_index: 0,
            selected_asset_index: 0,
            show_help: false,
            job_wizard: JobWizard::new(),
            search_box: SearchBox::new(),
            status_message: None,
            last_refresh: Instant::now(),
        };

        app.refresh().await?;
        Ok(app)
    }

    pub async fn refresh(&mut self) -> Result<()> {
        // Fetch jobs
        self.jobs = self.client.list_jobs().await.unwrap_or_default();

        // Fetch assets
        self.assets = self.client.list_assets().await.unwrap_or_default();

        self.last_refresh = Instant::now();
        Ok(())
    }

    pub fn should_refresh(&self) -> bool {
        self.last_refresh.elapsed() > Duration::from_secs(5)
    }

    pub fn next_view(&mut self) {
        self.state = match self.state {
            AppState::Dashboard => AppState::Jobs,
            AppState::Jobs => AppState::Assets,
            AppState::Assets => AppState::Dashboard,
            AppState::Help => AppState::Dashboard,
            AppState::CreateJob => AppState::CreateJob, // Stay in wizard
        };
    }

    pub fn previous_view(&mut self) {
        self.state = match self.state {
            AppState::Dashboard => AppState::Assets,
            AppState::Assets => AppState::Jobs,
            AppState::Jobs => AppState::Dashboard,
            AppState::Help => AppState::Dashboard,
            AppState::CreateJob => AppState::CreateJob, // Stay in wizard
        };
    }

    pub fn on_up(&mut self) {
        match self.state {
            AppState::Jobs => {
                if self.selected_job_index > 0 {
                    self.selected_job_index -= 1;
                }
            }
            AppState::Assets => {
                if self.selected_asset_index > 0 {
                    self.selected_asset_index -= 1;
                }
            }
            _ => {}
        }
    }

    pub fn on_down(&mut self) {
        match self.state {
            AppState::Jobs => {
                if self.selected_job_index < self.jobs.len().saturating_sub(1) {
                    self.selected_job_index += 1;
                }
            }
            AppState::Assets => {
                if self.selected_asset_index < self.assets.len().saturating_sub(1) {
                    self.selected_asset_index += 1;
                }
            }
            _ => {}
        }
    }

    pub async fn on_select(&mut self) -> Result<()> {
        // Handle selection based on current view
        // For now, just a placeholder
        Ok(())
    }

    pub fn toggle_help(&mut self) {
        self.show_help = !self.show_help;
        if self.show_help {
            self.state = AppState::Help;
        } else {
            self.state = AppState::Dashboard;
        }
    }

    pub fn running_jobs_count(&self) -> usize {
        self.jobs.iter().filter(|j| j.status == "running").count()
    }

    pub fn completed_jobs_count(&self) -> usize {
        self.jobs.iter().filter(|j| j.status == "completed").count()
    }

    pub fn queued_jobs_count(&self) -> usize {
        self.jobs.iter().filter(|j| j.status == "queued").count()
    }

    pub fn start_create_job(&mut self) {
        self.state = AppState::CreateJob;
        self.job_wizard.reset();
    }

    pub fn cancel_create_job(&mut self) {
        self.state = AppState::Jobs;
        self.job_wizard.reset();
    }

    pub async fn submit_job(&mut self) -> Result<()> {
        if !self.job_wizard.is_valid() {
            self.status_message = Some("Invalid job parameters".to_string());
            return Ok(());
        }

        let num_scenes = self.job_wizard.num_scenes.parse::<i32>().unwrap_or(100);
        let config_yaml = if self.job_wizard.config_file.is_empty() {
            None
        } else {
            Some(self.job_wizard.config_file.clone())
        };

        match self.client.create_job(
            self.job_wizard.job_name.clone(),
            num_scenes,
            config_yaml,
        ).await {
            Ok(_job) => {
                self.status_message = Some("Job created successfully!".to_string());
                self.state = AppState::Jobs;
                self.job_wizard.reset();
                self.refresh().await?;
            }
            Err(e) => {
                self.status_message = Some(format!("Failed to create job: {}", e));
            }
        }

        Ok(())
    }

    pub fn toggle_search(&mut self) {
        if self.search_box.active {
            self.search_box.deactivate();
        } else {
            self.search_box.activate();
        }
    }

    pub fn clear_search(&mut self) {
        self.search_box.clear();
        self.search_box.deactivate();
    }

    pub fn filtered_jobs(&self) -> Vec<&Job> {
        if self.search_box.query.is_empty() {
            self.jobs.iter().collect()
        } else {
            let mut results: Vec<_> = self.jobs.iter()
                .filter(|job| {
                    self.search_box.matches(&job.name) ||
                    self.search_box.matches(&job.status)
                })
                .collect();

            // Sort by match score
            results.sort_by(|a, b| {
                let score_a = self.search_box.match_score(&a.name);
                let score_b = self.search_box.match_score(&b.name);
                score_b.cmp(&score_a)
            });

            results
        }
    }

    pub fn filtered_assets(&self) -> Vec<&Asset> {
        if self.search_box.query.is_empty() {
            self.assets.iter().collect()
        } else {
            let mut results: Vec<_> = self.assets.iter()
                .filter(|asset| {
                    self.search_box.matches(&asset.name) ||
                    self.search_box.matches(&asset.file_type)
                })
                .collect();

            // Sort by match score
            results.sort_by(|a, b| {
                let score_a = self.search_box.match_score(&a.name);
                let score_b = self.search_box.match_score(&b.name);
                score_b.cmp(&score_a)
            });

            results
        }
    }

    pub fn clear_status_message(&mut self) {
        self.status_message = None;
    }
}
