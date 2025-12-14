//! Card API endpoints

use axum::{
    extract::{Path, Query},
    http::StatusCode,
    Json,
};
use serde::{Deserialize, Serialize};

use crate::services::ScryfallService;

#[derive(Debug, Deserialize)]
#[allow(dead_code)]
pub struct ListCardsQuery {
    pub q: Option<String>,
    pub page: Option<u32>,
}

#[derive(Debug, Deserialize)]
pub struct AutocompleteQuery {
    pub q: String,
}

#[derive(Debug, Deserialize)]
pub struct RandomCardQuery {
    pub q: Option<String>,
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
    pub has_more: bool,
    pub page: u32,
}

/// List/search cards via Scryfall
pub async fn list_cards(
    Query(params): Query<ListCardsQuery>,
) -> Result<Json<CardListResponse>, (StatusCode, String)> {
    let query = params.q.unwrap_or_else(|| "*".to_string());
    let page = params.page.unwrap_or(1);

    let service = ScryfallService::new();

    match service.search_cards(&query, Some(page)).await {
        Ok(response) => {
            let cards: Vec<serde_json::Value> = response
                .data
                .into_iter()
                .map(|card| serde_json::to_value(card).unwrap_or_default())
                .collect();

            Ok(Json(CardListResponse {
                success: true,
                data: cards,
                meta: ResponseMeta {
                    total: response.total_cards.unwrap_or(0),
                    has_more: response.has_more.unwrap_or(false),
                    page,
                },
            }))
        }
        Err(e) => Err((StatusCode::INTERNAL_SERVER_ERROR, e.to_string())),
    }
}

/// Get card by ID
pub async fn get_card(
    Path(id): Path<String>,
) -> Result<Json<CardResponse>, (StatusCode, String)> {
    let service = ScryfallService::new();

    match service.get_card_by_id(&id).await {
        Ok(card) => Ok(Json(CardResponse {
            success: true,
            data: serde_json::to_value(card).unwrap_or_default(),
        })),
        Err(e) => Err((StatusCode::NOT_FOUND, e.to_string())),
    }
}

/// Autocomplete card names
pub async fn autocomplete(
    Query(params): Query<AutocompleteQuery>,
) -> Result<Json<AutocompleteResponse>, (StatusCode, String)> {
    let service = ScryfallService::new();

    match service.autocomplete(&params.q).await {
        Ok(suggestions) => Ok(Json(AutocompleteResponse {
            success: true,
            data: suggestions,
        })),
        Err(e) => Err((StatusCode::INTERNAL_SERVER_ERROR, e.to_string())),
    }
}

/// Get random card
pub async fn random_card(
    Query(params): Query<RandomCardQuery>,
) -> Result<Json<CardResponse>, (StatusCode, String)> {
    let service = ScryfallService::new();

    match service.get_random_card(params.q.as_deref()).await {
        Ok(card) => Ok(Json(CardResponse {
            success: true,
            data: serde_json::to_value(card).unwrap_or_default(),
        })),
        Err(e) => Err((StatusCode::INTERNAL_SERVER_ERROR, e.to_string())),
    }
}
