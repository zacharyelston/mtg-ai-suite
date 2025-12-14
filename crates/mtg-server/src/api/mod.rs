//! API routes and handlers

pub mod captures;
pub mod cards;
pub mod health;

use axum::{routing::get, routing::post, Router};

/// Create API v1 routes
pub fn v1_routes() -> Router {
    Router::new()
        // Auth
        .route("/auth/verify", post(auth_verify))
        // Cards
        .route("/cards", get(cards::list_cards))
        .route("/cards/autocomplete", get(cards::autocomplete))
        .route("/cards/:id", get(cards::get_card))
        // Captures
        .route("/captures", get(captures::list_captures))
        .route("/captures", post(captures::create_capture))
        .route("/captures/:id", get(captures::get_capture))
        .route(
            "/captures/:id",
            axum::routing::patch(captures::update_capture),
        )
}

/// Verify API key and return client info
async fn auth_verify() -> axum::Json<serde_json::Value> {
    // TODO: Implement actual auth verification
    axum::Json(serde_json::json!({
        "success": true,
        "data": {
            "client_id": "placeholder",
            "permissions": ["read", "write", "ai"],
            "server_version": env!("CARGO_PKG_VERSION")
        }
    }))
}
