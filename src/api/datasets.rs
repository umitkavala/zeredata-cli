use crate::api::ApiClient;
use crate::error::Result;
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Dataset {
    pub id: i32,
    pub job_id: String,
    pub name: String,
    pub size_bytes: i64,
    pub format: String,
    pub created_at: String,
}

#[derive(Debug, Deserialize)]
pub struct DatasetListResponse {
    pub datasets: Vec<Dataset>,
    pub total: i32,
}

#[derive(Debug, Deserialize)]
pub struct DownloadUrlResponse {
    pub download_url: String,
    pub expires_at: String,
    pub size_bytes: i64,
    pub format: String,
}

impl ApiClient {
    /// List all datasets
    pub async fn list_datasets(&self) -> Result<Vec<Dataset>> {
        let req = self.get("/api/v1/datasets");
        let response: DatasetListResponse = self.send_json(req).await?;
        Ok(response.datasets)
    }

    /// Get dataset details
    pub async fn get_dataset(&self, dataset_id: &str) -> Result<Dataset> {
        let req = self.get(&format!("/api/v1/datasets/{}", dataset_id));
        self.send_json(req).await
    }

    /// Get download URL for a dataset (via job_id)
    pub async fn get_download_url(&self, job_id: &str) -> Result<DownloadUrlResponse> {
        let req = self.get(&format!("/api/v1/jobs/{}/download", job_id));
        self.send_json(req).await
    }

    /// Download dataset to file
    pub async fn download_dataset(&self, download_url: &str, output_path: &std::path::Path) -> Result<()> {
        let response = reqwest::get(download_url).await?;
        let bytes = response.bytes().await?;
        tokio::fs::write(output_path, bytes).await?;
        Ok(())
    }
}
