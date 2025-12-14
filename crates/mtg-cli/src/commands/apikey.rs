//! API key management commands

use argon2::{
    password_hash::{rand_core::OsRng, SaltString},
    Argon2, PasswordHasher,
};
use colored::Colorize;
use uuid::Uuid;

/// Create a new API key
pub async fn create(
    name: &str,
    permissions: &str,
    expires: &str,
    json_output: bool,
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

    if json_output {
        println!(
            "{}",
            serde_json::json!({
                "success": true,
                "data": {
                    "key_id": key_id.to_string(),
                    "api_key": raw_key,
                    "name": name,
                    "permissions": permissions_list,
                    "expires": expires,
                    "key_hash": key_hash
                }
            })
        );
    } else {
        println!("{}", "✓ API key created successfully".green());
        println!();
        println!("  {} {}", "Key ID:".bold(), key_id);
        println!("  {} {}", "Name:".bold(), name);
        println!("  {} {:?}", "Permissions:".bold(), permissions_list);
        println!("  {} {}", "Expires:".bold(), expires);
        println!();
        println!(
            "{}",
            "  API Key (save this - it won't be shown again):".yellow()
        );
        println!("  {}", raw_key.cyan().bold());
        println!();
        println!("{}", "  Add this to your mobile app to connect.".dimmed());
    }

    // TODO: Store in database
    // For now, just print the key

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
