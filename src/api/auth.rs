use crate::api::ApiClient;
use crate::error::Result;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize)]
pub struct LoginRequest {
    pub email: String,
    pub password: String,
}

#[derive(Debug, Deserialize)]
pub struct LoginResponse {
    pub access_token: String,
}

#[derive(Debug, Deserialize)]
pub struct UserInfo {
    pub id: i32,
    pub email: String,
    pub organization_id: Option<i32>,
    pub organization_name: Option<String>,
    pub role: Option<String>,
}

impl ApiClient {
    /// Login with email and password
    pub async fn login(&self, email: String, password: String) -> Result<String> {
        let req = self.post("/api/v1/auth/login")
            .json(&LoginRequest { email, password });

        let response: LoginResponse = self.send_json(req).await?;
        Ok(response.access_token)
    }

    /// Get current user info
    pub async fn whoami(&self) -> Result<UserInfo> {
        let req = self.get("/api/v1/auth/me");
        self.send_json(req).await
    }

    /// Logout (server-side)
    pub async fn logout(&self) -> Result<()> {
        let req = self.post("/api/v1/auth/logout");
        self.send(req).await
    }
}
