use crate::error::{CliError, Result};
use directories::ProjectDirs;
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::PathBuf;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Config {
    #[serde(default = "default_api_endpoint")]
    pub api: ApiConfig,
    #[serde(default)]
    pub auth: AuthConfig,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ApiConfig {
    pub endpoint: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct AuthConfig {
    pub api_key: Option<String>,
}

fn default_api_endpoint() -> ApiConfig {
    ApiConfig {
        endpoint: "https://api.zeredata.com".to_string(),
    }
}

impl Default for Config {
    fn default() -> Self {
        Config {
            api: default_api_endpoint(),
            auth: AuthConfig::default(),
        }
    }
}

impl Config {
    /// Get the config directory path
    pub fn config_dir() -> Result<PathBuf> {
        ProjectDirs::from("com", "zeredata", "zere")
            .map(|proj_dirs| proj_dirs.config_dir().to_path_buf())
            .ok_or_else(|| CliError::Config("Could not determine config directory".to_string()))
    }

    /// Get the config file path
    pub fn config_path() -> Result<PathBuf> {
        Ok(Self::config_dir()?.join("config.toml"))
    }

    /// Load config from file, or create default if it doesn't exist
    pub fn load() -> Result<Self> {
        let path = Self::config_path()?;

        if !path.exists() {
            let config = Config::default();
            config.save()?;
            return Ok(config);
        }

        let contents = fs::read_to_string(&path)
            .map_err(|e| CliError::Config(format!("Failed to read config file: {}", e)))?;

        toml::from_str(&contents)
            .map_err(|e| CliError::Config(format!("Failed to parse config file: {}", e)))
    }

    /// Save config to file
    pub fn save(&self) -> Result<()> {
        let dir = Self::config_dir()?;
        fs::create_dir_all(&dir)
            .map_err(|e| CliError::Config(format!("Failed to create config directory: {}", e)))?;

        let path = Self::config_path()?;
        let contents = toml::to_string_pretty(self)
            .map_err(|e| CliError::Config(format!("Failed to serialize config: {}", e)))?;

        fs::write(&path, contents)
            .map_err(|e| CliError::Config(format!("Failed to write config file: {}", e)))?;

        Ok(())
    }

    /// Set API endpoint
    pub fn set_endpoint(&mut self, endpoint: String) {
        self.api.endpoint = endpoint;
    }

    /// Set API key
    pub fn set_api_key(&mut self, api_key: Option<String>) {
        self.auth.api_key = api_key;
    }

    /// Check if user is authenticated
    pub fn is_authenticated(&self) -> bool {
        self.auth.api_key.is_some()
    }

    /// Get API key or return error
    pub fn require_auth(&self) -> Result<&str> {
        self.auth.api_key
            .as_deref()
            .ok_or(CliError::NotAuthenticated)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_default_config() {
        let config = Config::default();
        assert_eq!(config.api.endpoint, "https://api.zeredata.com");
        assert!(config.auth.api_key.is_none());
        assert!(!config.is_authenticated());
    }

    #[test]
    fn test_config_serialization() {
        let config = Config::default();
        let toml_str = toml::to_string(&config).unwrap();
        let parsed: Config = toml::from_str(&toml_str).unwrap();
        assert_eq!(parsed.api.endpoint, config.api.endpoint);
    }
}
