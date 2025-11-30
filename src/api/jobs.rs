use crate::api::ApiClient;
use crate::error::Result;
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Job {
    pub id: i32,
    pub job_id: String,
    pub name: String,
    pub status: String,
    pub num_scenes: i32,
    pub progress: Option<i32>,
    pub created_at: String,
    pub completed_at: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct JobListResponse {
    pub jobs: Vec<Job>,
    pub total: i32,
}

#[derive(Debug, Serialize)]
pub struct CreateJobRequest {
    pub name: String,
    pub num_scenes: i32,
    pub config_yaml: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct JobProgress {
    pub status: String,
    pub progress: i32,
    pub progress_percent: f32,
    pub scenes_generated: i32,
    pub eta_seconds: Option<i32>,
}

impl ApiClient {
    /// List all jobs
    pub async fn list_jobs(&self) -> Result<Vec<Job>> {
        let req = self.get("/api/v1/jobs");
        let response: JobListResponse = self.send_json(req).await?;
        Ok(response.jobs)
    }

    /// Get job details
    pub async fn get_job(&self, job_id: &str) -> Result<Job> {
        let req = self.get(&format!("/api/v1/jobs/{}", job_id));
        self.send_json(req).await
    }

    /// Create a new job
    pub async fn create_job(
        &self,
        name: String,
        num_scenes: i32,
        config_yaml: Option<String>,
    ) -> Result<Job> {
        let req = self.post("/api/v1/jobs").json(&CreateJobRequest {
            name,
            num_scenes,
            config_yaml,
        });
        self.send_json(req).await
    }

    /// Get job progress
    pub async fn get_job_progress(&self, job_id: &str) -> Result<JobProgress> {
        let req = self.get(&format!("/api/v1/jobs/{}/progress", job_id));
        self.send_json(req).await
    }

    /// Cancel a job
    pub async fn cancel_job(&self, job_id: &str) -> Result<()> {
        let req = self.post(&format!("/api/v1/jobs/{}/cancel", job_id));
        self.send(req).await
    }
}
