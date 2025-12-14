//! Scryfall API client service

use reqwest::Client;
use serde::{Deserialize, Serialize};
use thiserror::Error;

const SCRYFALL_API_BASE: &str = "https://api.scryfall.com";
const USER_AGENT: &str = "MTGAISuite/0.1.0";

#[derive(Error, Debug)]
pub enum ScryfallError {
    #[error("HTTP request failed: {0}")]
    RequestFailed(#[from] reqwest::Error),
    #[error("Card not found: {0}")]
    NotFound(String),
    #[error("API error: {0}")]
    ApiError(String),
}

#[derive(Debug, Clone)]
pub struct ScryfallService {
    client: Client,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct ScryfallCard {
    pub id: String,
    pub name: String,
    pub mana_cost: Option<String>,
    pub cmc: Option<f32>,
    pub type_line: Option<String>,
    pub oracle_text: Option<String>,
    pub colors: Option<Vec<String>>,
    pub color_identity: Option<Vec<String>>,
    pub set: String,
    pub set_name: String,
    pub rarity: String,
    pub image_uris: Option<ImageUris>,
    pub prices: Option<Prices>,
    pub legalities: Option<serde_json::Value>,
    pub scryfall_uri: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct ImageUris {
    pub small: Option<String>,
    pub normal: Option<String>,
    pub large: Option<String>,
    pub art_crop: Option<String>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Prices {
    pub usd: Option<String>,
    pub usd_foil: Option<String>,
    pub eur: Option<String>,
}

#[derive(Debug, Deserialize)]
#[allow(dead_code)]
pub struct ScryfallSearchResponse {
    pub object: String,
    pub total_cards: Option<u32>,
    pub has_more: Option<bool>,
    pub data: Vec<ScryfallCard>,
}

#[derive(Debug, Deserialize)]
#[allow(dead_code)]
pub struct AutocompleteResponse {
    pub object: String,
    pub data: Vec<String>,
}

impl ScryfallService {
    pub fn new() -> Self {
        let client = Client::builder()
            .user_agent(USER_AGENT)
            .build()
            .expect("Failed to create HTTP client");

        Self { client }
    }

    pub async fn search_cards(
        &self,
        query: &str,
        page: Option<u32>,
    ) -> Result<ScryfallSearchResponse, ScryfallError> {
        let page = page.unwrap_or(1);
        let url = format!("{}/cards/search", SCRYFALL_API_BASE);

        let response = self
            .client
            .get(&url)
            .query(&[("q", query), ("page", &page.to_string())])
            .send()
            .await?;

        if response.status().is_success() {
            let data = response.json().await?;
            Ok(data)
        } else if response.status() == 404 {
            Ok(ScryfallSearchResponse {
                object: "list".to_string(),
                total_cards: Some(0),
                has_more: Some(false),
                data: vec![],
            })
        } else {
            let error_text = response.text().await.unwrap_or_default();
            Err(ScryfallError::ApiError(error_text))
        }
    }

    pub async fn get_card_by_id(&self, id: &str) -> Result<ScryfallCard, ScryfallError> {
        let url = format!("{}/cards/{}", SCRYFALL_API_BASE, id);

        let response = self.client.get(&url).send().await?;

        if response.status().is_success() {
            let card = response.json().await?;
            Ok(card)
        } else if response.status() == 404 {
            Err(ScryfallError::NotFound(id.to_string()))
        } else {
            let error_text = response.text().await.unwrap_or_default();
            Err(ScryfallError::ApiError(error_text))
        }
    }

    pub async fn autocomplete(&self, query: &str) -> Result<Vec<String>, ScryfallError> {
        let url = format!("{}/cards/autocomplete", SCRYFALL_API_BASE);

        let response = self.client.get(&url).query(&[("q", query)]).send().await?;

        if response.status().is_success() {
            let data: AutocompleteResponse = response.json().await?;
            Ok(data.data)
        } else {
            Ok(vec![])
        }
    }

    pub async fn get_random_card(
        &self,
        query: Option<&str>,
    ) -> Result<ScryfallCard, ScryfallError> {
        let url = format!("{}/cards/random", SCRYFALL_API_BASE);

        let mut request = self.client.get(&url);
        if let Some(q) = query {
            request = request.query(&[("q", q)]);
        }

        let response = request.send().await?;

        if response.status().is_success() {
            let card = response.json().await?;
            Ok(card)
        } else {
            let error_text = response.text().await.unwrap_or_default();
            Err(ScryfallError::ApiError(error_text))
        }
    }
}

impl Default for ScryfallService {
    fn default() -> Self {
        Self::new()
    }
}
