//! Server configuration

use serde::Deserialize;

/// Server configuration
#[derive(Debug, Clone, Deserialize)]
#[allow(dead_code)]
pub struct Config {
    /// Server port
    #[serde(default = "default_port")]
    pub port: u16,

    /// Redis URL (optional)
    pub redis_url: Option<String>,

    /// LLM API key (optional)
    pub llm_api_key: Option<String>,

    /// LLM provider (openai, anthropic, ollama)
    #[serde(default = "default_llm_provider")]
    pub llm_provider: String,

    /// Secret key for JWT signing
    #[serde(default = "default_secret_key")]
    pub secret_key: String,

    /// API key hash rounds
    #[serde(default = "default_hash_rounds")]
    pub hash_rounds: u32,
}

fn default_port() -> u16 {
    8080
}

fn default_llm_provider() -> String {
    "openai".to_string()
}

fn default_secret_key() -> String {
    "change-me-in-production".to_string()
}

fn default_hash_rounds() -> u32 {
    12
}

impl Config {
    /// Load configuration from environment variables
    pub fn from_env() -> anyhow::Result<Self> {
        Ok(Self {
            port: std::env::var("PORT")
                .ok()
                .and_then(|p| p.parse().ok())
                .unwrap_or_else(default_port),
            redis_url: std::env::var("REDIS_URL").ok(),
            llm_api_key: std::env::var("LLM_API_KEY").ok(),
            llm_provider: std::env::var("LLM_PROVIDER").unwrap_or_else(|_| default_llm_provider()),
            secret_key: std::env::var("SECRET_KEY").unwrap_or_else(|_| default_secret_key()),
            hash_rounds: std::env::var("HASH_ROUNDS")
                .ok()
                .and_then(|r| r.parse().ok())
                .unwrap_or_else(default_hash_rounds),
        })
    }
}
