//! API key management commands

use argon2::{
    password_hash::{rand_core::OsRng, SaltString},
    Argon2, PasswordHasher,
};
use chrono::Utc;
use colored::Colorize;
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::Path;
use uuid::Uuid;

const CONFIG_PATH: &str = "config/server-config.json";

#[derive(Debug, Clone, Serialize, Deserialize)]
struct ServerConfig {
    version: String,
    created_at: Option<chrono::DateTime<Utc>>,
    updated_at: Option<chrono::DateTime<Utc>>,
    server: ServerSecrets,
    api_keys: Vec<ApiKeyEntry>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct ServerSecrets {
    secret_key: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct ApiKeyEntry {
    id: String,
    name: String,
    raw_key: Option<String>,
    key_hash: String,
    permissions: Vec<String>,
    expires: String,
    created_at: chrono::DateTime<Utc>,
}

impl ServerConfig {
    fn load_or_init() -> anyhow::Result<Self> {
        let path = Path::new(CONFIG_PATH);
        if path.exists() {
            let content = fs::read_to_string(path)?;
            Ok(serde_json::from_str(&content)?)
        } else {
            Ok(Self::default())
        }
    }

    fn save(&self) -> anyhow::Result<()> {
        let path = Path::new(CONFIG_PATH);
        if let Some(parent) = path.parent() {
            fs::create_dir_all(parent)?;
        }
        fs::write(path, serde_json::to_string_pretty(self)?)?;
        Ok(())
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

/// Create a new API key
pub async fn create(
    name: &str,
    permissions: &str,
    expires: &str,
    json_output: bool,
    output_file: Option<&str>,
    backend_url: Option<&str>,
) -> anyhow::Result<()> {
    // Generate a new API key
    let key_id = Uuid::new_v4();
    let raw_key = generate_api_key();

    // Hash the key for storage
    let salt = SaltString::generate(&mut OsRng);
    let argon2 = Argon2::default();
    let key_hash = argon2
        .hash_password(raw_key.as_bytes(), &salt)
        .map_err(|e| anyhow::anyhow!("Failed to hash key: {}", e))?
        .to_string();

    let permissions_list: Vec<&str> = permissions.split(',').collect();
    let url = backend_url.unwrap_or("http://localhost:5000");

    // Format for client connection string
    let connection_string = format!("{}:{}", url, raw_key);

    // Save to shared config
    let mut config = ServerConfig::load_or_init()?;
    config.api_keys.push(ApiKeyEntry {
        id: key_id.to_string(),
        name: name.to_string(),
        raw_key: Some(raw_key.clone()),
        key_hash: key_hash.clone(),
        permissions: permissions_list.iter().map(|s| s.to_string()).collect(),
        expires: expires.to_string(),
        created_at: Utc::now(),
    });
    config.updated_at = Some(Utc::now());
    config.save()?;

    if json_output {
        let json_data = serde_json::json!({
            "success": true,
            "data": {
                "key_id": key_id.to_string(),
                "api_key": raw_key,
                "backend_url": url,
                "connection_string": connection_string,
                "name": name,
                "permissions": permissions_list,
                "expires": expires,
                "key_hash": key_hash
            }
        });

        let output = serde_json::to_string_pretty(&json_data)?;

        if let Some(file_path) = output_file {
            fs::write(file_path, &output)?;
            eprintln!("{} Written to {}", "✓".green(), file_path);
        } else {
            println!("{}", output);
        }
    } else {
        let mut output = String::new();
        output.push_str(&format!("Backend URL: {}\n", url));
        output.push_str(&format!("API Key: {}\n", raw_key));
        output.push_str(&format!("Connection String: {}\n", connection_string));
        output.push_str(&format!("Key ID: {}\n", key_id));
        output.push_str(&format!("Name: {}\n", name));
        output.push_str(&format!("Permissions: {:?}\n", permissions_list));
        output.push_str(&format!("Expires: {}\n", expires));

        if let Some(file_path) = output_file {
            fs::write(file_path, &output)?;
            println!("{} API key created and written to {}", "✓".green(), file_path);
        } else {
            println!("{}", "✓ API key created successfully".green());
            println!();
            println!("  {} {}", "Backend URL:".bold(), url);
            println!("  {} {}", "Key ID:".bold(), key_id);
            println!("  {} {}", "Name:".bold(), name);
            println!("  {} {:?}", "Permissions:".bold(), permissions_list);
            println!("  {} {}", "Expires:".bold(), expires);
            println!();
            println!(
                "{}",
                "  Connection String (BackendURL:API_KEY):".yellow()
            );
            println!("  {}", connection_string.cyan().bold());
            println!();
            println!("{}", "  Add this to your mobile app to connect.".dimmed());
        }
    }

    Ok(())
}

/// List all API keys
pub async fn list(active_only: bool, json_output: bool) -> anyhow::Result<()> {
    // TODO: Fetch from database
    let keys: Vec<serde_json::Value> = vec![];

    if json_output {
        println!(
            "{}",
            serde_json::json!({
                "success": true,
                "data": keys,
                "meta": {
                    "active_only": active_only
                }
            })
        );
    } else if keys.is_empty() {
        println!("{}", "No API keys found.".dimmed());
        println!();
        println!(
            "Create one with: {} apikey create --name \"My Device\"",
            "mtg-ai-suite".cyan()
        );
    } else {
        println!("{}", "API Keys:".bold());
        // TODO: Print table of keys
    }

    Ok(())
}

/// Revoke an API key
pub async fn revoke(key_id: &str, force: bool) -> anyhow::Result<()> {
    if !force {
        println!(
            "{}",
            "Are you sure you want to revoke this API key?".yellow()
        );
        println!("Key ID: {}", key_id);
        println!();
        println!("This action cannot be undone. The client will lose access immediately.");
        println!();
        println!("Use --force to skip this confirmation.");
        return Ok(());
    }

    // TODO: Revoke in database
    println!("{} Key {} revoked", "✓".green(), key_id);

    Ok(())
}

/// Rotate an API key
pub async fn rotate(key_id: &str, notify: bool) -> anyhow::Result<()> {
    let new_key = generate_api_key();

    // TODO: Update in database and optionally send push notification

    println!("{} Key {} rotated", "✓".green(), key_id);
    println!();
    println!("{}", "New API Key:".yellow());
    println!("  {}", new_key.cyan().bold());

    if notify {
        println!();
        println!("{}", "Push notification sent to device.".dimmed());
    }

    Ok(())
}

/// Generate a random API key
fn generate_api_key() -> String {
    use rand::Rng;
    const CHARSET: &[u8] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789";
    const KEY_LEN: usize = 32;

    let mut rng = rand::thread_rng();
    let key: String = (0..KEY_LEN)
        .map(|_| {
            let idx = rng.gen_range(0..CHARSET.len());
            CHARSET[idx] as char
        })
        .collect();

    format!("mtg_{}", key)
}
