use crate::config::Config;
use crate::error::{CliError, Result};
use reqwest::{Client, RequestBuilder, Response};
use serde::de::DeserializeOwned;
use serde::Serialize;
use std::time::Duration;

#[derive(Clone)]
pub struct ApiClient {
    client: Client,
    base_url: String,
    api_key: Option<String>,
}

impl ApiClient {
    /// Create a new API client from config
    pub fn from_config(config: &Config) -> Result<Self> {
        let client = Client::builder()
            .timeout(Duration::from_secs(30))
            .build()
            .map_err(|e| CliError::Network(e))?;

        Ok(ApiClient {
            client,
            base_url: config.api.endpoint.clone(),
            api_key: config.auth.api_key.clone(),
        })
    }

    /// Create a new API client with custom base URL
    pub fn new(base_url: String, api_key: Option<String>) -> Result<Self> {
        let client = Client::builder()
            .timeout(Duration::from_secs(30))
            .build()
            .map_err(|e| CliError::Network(e))?;

        Ok(ApiClient {
            client,
            base_url,
            api_key,
        })
    }

    /// Build a GET request
    pub fn get(&self, path: &str) -> RequestBuilder {
        let url = format!("{}{}", self.base_url, path);
        let mut req = self.client.get(&url);
        if let Some(key) = &self.api_key {
            req = req.bearer_auth(key);
        }
        req
    }

    /// Build a POST request
    pub fn post(&self, path: &str) -> RequestBuilder {
        let url = format!("{}{}", self.base_url, path);
        let mut req = self.client.post(&url);
        if let Some(key) = &self.api_key {
            req = req.bearer_auth(key);
        }
        req
    }

    /// Build a PUT request
    pub fn put(&self, path: &str) -> RequestBuilder {
        let url = format!("{}{}", self.base_url, path);
        let mut req = self.client.put(&url);
        if let Some(key) = &self.api_key {
            req = req.bearer_auth(key);
        }
        req
    }

    /// Build a DELETE request
    pub fn delete(&self, path: &str) -> RequestBuilder {
        let url = format!("{}{}", self.base_url, path);
        let mut req = self.client.delete(&url);
        if let Some(key) = &self.api_key {
            req = req.bearer_auth(key);
        }
        req
    }

    /// Send request and parse JSON response
    pub async fn send_json<T: DeserializeOwned>(&self, req: RequestBuilder) -> Result<T> {
        let response = req.send().await?;
        self.handle_response(response).await
    }

    /// Handle response and parse JSON
    async fn handle_response<T: DeserializeOwned>(&self, response: Response) -> Result<T> {
        let status = response.status();

        if status.is_success() {
            response.json::<T>().await.map_err(|e| {
                CliError::InvalidResponse
            })
        } else if status == 401 {
            Err(CliError::Auth("Unauthorized. Please login again.".to_string()))
        } else if status == 403 {
            Err(CliError::Auth("Forbidden. You don't have permission to access this resource.".to_string()))
        } else if status == 404 {
            Err(CliError::Api("Resource not found".to_string()))
        } else {
            let error_text = response.text().await.unwrap_or_else(|_| "Unknown error".to_string());
            Err(CliError::Api(format!("API error ({}): {}", status, error_text)))
        }
    }

    /// Send request with no expected response
    pub async fn send(&self, req: RequestBuilder) -> Result<()> {
        let response = req.send().await?;
        let status = response.status();

        if status.is_success() {
            Ok(())
        } else if status == 401 {
            Err(CliError::Auth("Unauthorized. Please login again.".to_string()))
        } else if status == 403 {
            Err(CliError::Auth("Forbidden. You don't have permission to access this resource.".to_string()))
        } else if status == 404 {
            Err(CliError::Api("Resource not found".to_string()))
        } else {
            let error_text = response.text().await.unwrap_or_else(|_| "Unknown error".to_string());
            Err(CliError::Api(format!("API error ({}): {}", status, error_text)))
        }
    }
}
