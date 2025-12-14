//! Scryfall API - implements GameDataSource trait

use crate::card::MtgCard;
use async_trait::async_trait;
use cardgame_core::prelude::*;
use chrono::{DateTime, Utc};

pub struct ScryfallDataSource {
    base_url: String,
    #[allow(dead_code)]
    client: reqwest::Client,
}

impl ScryfallDataSource {
    pub fn new() -> Self {
        Self {
            base_url: "https://api.scryfall.com".into(),
            client: reqwest::Client::builder()
                .user_agent("CardGameAI/0.1.0")
                .build()
                .unwrap(),
        }
    }
}

impl Default for ScryfallDataSource {
    fn default() -> Self {
        Self::new()
    }
}

#[async_trait]
impl GameDataSource<MtgCard> for ScryfallDataSource {
    async fn get_piece(&self, id: &String) -> FrameworkResult<MtgCard> {
        // TODO: Implement actual API call
        Err(FrameworkError::PieceNotFound { id: id.clone() })
    }

    async fn search(
        &self,
        _query: &str,
        _options: SearchOptions,
    ) -> FrameworkResult<SearchResult<MtgCard>> {
        // TODO: Implement actual API call
        Ok(SearchResult {
            pieces: vec![],
            total_count: 0,
            has_more: false,
            next_page_token: None,
        })
    }

    async fn autocomplete(&self, _prefix: &str, _limit: usize) -> FrameworkResult<Vec<String>> {
        // TODO: Implement actual API call
        Ok(vec![])
    }

    async fn bulk_download(&self) -> FrameworkResult<BulkData<MtgCard>> {
        // TODO: Implement actual API call
        Ok(BulkData {
            pieces: vec![],
            generated_at: Utc::now(),
            version: "0".into(),
            size_bytes: 0,
        })
    }

    async fn updates_since(&self, _since: DateTime<Utc>) -> FrameworkResult<Vec<MtgCard>> {
        Ok(vec![])
    }

    fn metadata(&self) -> DataSourceMetadata {
        DataSourceMetadata {
            name: "Scryfall".into(),
            base_url: self.base_url.clone(),
            api_version: "1.0".into(),
            rate_limit: Some(10.0),
            requires_auth: false,
            attribution: Some("Card data provided by Scryfall".into()),
        }
    }
}
