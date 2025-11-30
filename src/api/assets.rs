use crate::api::ApiClient;
use crate::error::Result;
use reqwest::multipart::{Form, Part};
use serde::{Deserialize, Serialize};
use std::path::Path;
use tokio::fs::File;
use tokio::io::AsyncReadExt;

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Asset {
    pub id: i32,
    pub asset_id: String,
    pub name: String,
    pub file_type: String,
    pub size_bytes: i64,
    pub thumbnail_url: Option<String>,
    pub created_at: String,
}

#[derive(Debug, Deserialize)]
pub struct AssetListResponse {
    pub assets: Vec<Asset>,
    pub total: i32,
}

#[derive(Debug, Serialize)]
pub struct AssetUploadMetadata {
    pub name: String,
    pub category: Option<String>,
    pub tags: Vec<String>,
}

impl ApiClient {
    /// List all assets
    pub async fn list_assets(&self) -> Result<Vec<Asset>> {
        let req = self.get("/api/v1/assets");
        let response: AssetListResponse = self.send_json(req).await?;
        Ok(response.assets)
    }

    /// Get asset details
    pub async fn get_asset(&self, asset_id: &str) -> Result<Asset> {
        let req = self.get(&format!("/api/v1/assets/{}", asset_id));
        self.send_json(req).await
    }

    /// Upload asset
    pub async fn upload_asset(
        &self,
        file_path: &Path,
        name: String,
        category: Option<String>,
        tags: Vec<String>,
    ) -> Result<Asset> {
        // Read file
        let mut file = File::open(file_path).await?;
        let mut buffer = Vec::new();
        file.read_to_end(&mut buffer).await?;

        let file_name = file_path
            .file_name()
            .and_then(|n| n.to_str())
            .unwrap_or("upload")
            .to_string();

        // Create multipart form
        let file_part = Part::bytes(buffer).file_name(file_name);
        let form = Form::new()
            .part("file", file_part)
            .text("name", name)
            .text("tags", serde_json::to_string(&tags)?);

        let form = if let Some(cat) = category {
            form.text("category", cat)
        } else {
            form
        };

        let req = self.post("/api/v1/assets/upload").multipart(form);
        self.send_json(req).await
    }

    /// Delete asset
    pub async fn delete_asset(&self, asset_id: &str) -> Result<()> {
        let req = self.delete(&format!("/api/v1/assets/{}", asset_id));
        self.send(req).await
    }
}
