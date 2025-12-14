//! Card API endpoints

use axum::{
    extract::{Path, Query},
    Json,
};
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize)]
#[allow(dead_code)]
pub struct ListCardsQuery {
    pub q: Option<String>,
    pub limit: Option<u32>,
    pub offset: Option<u32>,
}

#[derive(Debug, Deserialize)]
pub struct AutocompleteQuery {
    pub q: String,
}

#[derive(Debug, Serialize)]
pub struct CardListResponse {
    pub success: bool,
    pub data: Vec<serde_json::Value>,
    pub meta: ResponseMeta,
}

#[derive(Debug, Serialize)]
pub struct CardResponse {
    pub success: bool,
    pub data: serde_json::Value,
}

#[derive(Debug, Serialize)]
pub struct AutocompleteResponse {
    pub success: bool,
    pub data: Vec<String>,
}

#[derive(Debug, Serialize)]
pub struct ResponseMeta {
    pub total: u32,
    pub limit: u32,
    pub offset: u32,
}

/// List cards with optional search
pub async fn list_cards(Query(params): Query<ListCardsQuery>) -> Json<CardListResponse> {
    let limit = params.limit.unwrap_or(20).min(100);
    let offset = params.offset.unwrap_or(0);

    // TODO: Implement actual card listing from database
    Json(CardListResponse {
        success: true,
        data: vec![],
        meta: ResponseMeta {
            total: 0,
            limit,
            offset,
        },
    })
}

/// Get card by ID
pub async fn get_card(Path(id): Path<String>) -> Json<CardResponse> {
    // TODO: Implement actual card lookup
    Json(CardResponse {
        success: true,
        data: serde_json::json!({
            "id": id,
            "name": "Placeholder Card",
            "message": "Card lookup not yet implemented"
        }),
    })
}

/// Autocomplete card names
pub async fn autocomplete(Query(params): Query<AutocompleteQuery>) -> Json<AutocompleteResponse> {
    // TODO: Implement actual autocomplete from card database
    let _query = params.q;

    Json(AutocompleteResponse {
        success: true,
        data: vec![],
    })
}
