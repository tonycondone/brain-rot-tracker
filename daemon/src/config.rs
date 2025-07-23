use anyhow::Result;
use serde::{Deserialize, Serialize};
use std::path::PathBuf;
use tokio::fs;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Config {
    pub database_path: String,
    pub rules_path: String,
    pub api_port: u16,
    pub tracking_interval_seconds: u64,
    pub idle_threshold_minutes: u64,
    pub dashboard_path: String,
}

impl Default for Config {
    fn default() -> Self {
        let data_dir = dirs::data_dir()
            .unwrap_or_else(|| PathBuf::from("."))
            .join("brain-rot-tracker");

        Self {
            database_path: data_dir.join("data.db").to_string_lossy().to_string(),
            rules_path: data_dir.join("rules.yaml").to_string_lossy().to_string(),
            api_port: 8080,
            tracking_interval_seconds: 5,
            idle_threshold_minutes: 5,
            dashboard_path: "./dashboard/dist".to_string(),
        }
    }
}

impl Config {
    pub async fn load() -> Result<Self> {
        let config_path = Self::config_path();
        
        if config_path.exists() {
            let content = fs::read_to_string(&config_path).await?;
            let config: Config = serde_yaml::from_str(&content)?;
            Ok(config)
        } else {
            let config = Config::default();
            config.save().await?;
            Ok(config)
        }
    }

    pub async fn save(&self) -> Result<()> {
        let config_path = Self::config_path();
        
        if let Some(parent) = config_path.parent() {
            fs::create_dir_all(parent).await?;
        }

        let content = serde_yaml::to_string(self)?;
        fs::write(&config_path, content).await?;
        Ok(())
    }

    fn config_path() -> PathBuf {
        dirs::config_dir()
            .unwrap_or_else(|| PathBuf::from("."))
            .join("brain-rot-tracker")
            .join("config.yaml")
    }

    pub fn data_dir(&self) -> PathBuf {
        PathBuf::from(&self.database_path)
            .parent()
            .unwrap_or(&PathBuf::from("."))
            .to_path_buf()
    }
}