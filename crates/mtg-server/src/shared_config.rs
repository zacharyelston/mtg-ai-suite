//! Shared server configuration that persists API keys

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::Path;

const CONFIG_PATH: &str = "config/server-config.json";

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ServerConfig {
    pub version: String,
    pub created_at: Option<DateTime<Utc>>,
    pub updated_at: Option<DateTime<Utc>>,
    pub server: ServerSecrets,
    pub api_keys: Vec<ApiKeyEntry>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ServerSecrets {
    pub secret_key: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ApiKeyEntry {
    pub id: String,
    pub name: String,
    pub raw_key: Option<String>,
    pub key_hash: String,
    pub permissions: Vec<String>,
    pub expires: String,
    pub created_at: DateTime<Utc>,
}

impl ServerConfig {
    pub fn load_or_init() -> anyhow::Result<Self> {
        let path = Path::new(CONFIG_PATH);
        
        if path.exists() {
            let content = fs::read_to_string(path)?;
            let config: ServerConfig = serde_json::from_str(&content)?;
            tracing::info!("Loaded existing server config with {} API keys", config.api_keys.len());
            Ok(config)
        } else {
            let config = Self::default();
            config.save()?;
            tracing::info!("Created new server config");
            Ok(config)
        }
    }

    pub fn save(&self) -> anyhow::Result<()> {
        let path = Path::new(CONFIG_PATH);
        if let Some(parent) = path.parent() {
            fs::create_dir_all(parent)?;
        }
        let content = serde_json::to_string_pretty(self)?;
        fs::write(path, content)?;
        Ok(())
    }

    pub fn add_api_key(&mut self, entry: ApiKeyEntry) -> anyhow::Result<()> {
        self.api_keys.push(entry);
        self.updated_at = Some(Utc::now());
        self.save()
    }

    pub fn find_key_by_raw(&self, raw_key: &str) -> Option<&ApiKeyEntry> {
        self.api_keys.iter().find(|k| k.raw_key.as_deref() == Some(raw_key))
    }

    pub fn get_all_keys(&self) -> &[ApiKeyEntry] {
        &self.api_keys
    }
}

impl Default for ServerConfig {
    fn default() -> Self {
        Self {
            version: "1.0".to_string(),
            created_at: Some(Utc::now()),
            updated_at: Some(Utc::now()),
            server: ServerSecrets { secret_key: None },
            api_keys: vec![],
        }
    }
}
