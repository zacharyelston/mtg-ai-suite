//! GameDataSource trait - external data provider for game pieces

use super::piece::GamePiece;
use crate::error::FrameworkResult;
use async_trait::async_trait;
use chrono::{DateTime, Utc};

/// External data source for game pieces.
///
/// This trait provides access to game piece data from external APIs
/// or databases, such as Scryfall for MTG or pokemontcg.io for Pokemon.
///
/// # Example
///
/// ```rust,ignore
/// use cardgame_core::traits::GameDataSource;
///
/// struct ScryfallDataSource {
///     client: reqwest::Client,
///     base_url: String,
/// }
///
/// #[async_trait]
/// impl GameDataSource<MtgCard> for ScryfallDataSource {
///     async fn get_piece(&self, id: &String) -> FrameworkResult<MtgCard> {
///         // Fetch from Scryfall API
///     }
///     // ...
/// }
/// ```
#[async_trait]
pub trait GameDataSource<P: GamePiece>: Send + Sync {
    /// Fetch a single piece by ID.
    async fn get_piece(&self, id: &P::Id) -> FrameworkResult<P>;

    /// Fetch multiple pieces by ID.
    ///
    /// Default implementation calls `get_piece` for each ID.
    async fn get_pieces(&self, ids: &[P::Id]) -> FrameworkResult<Vec<P>> {
        let mut pieces = Vec::with_capacity(ids.len());
        for id in ids {
            pieces.push(self.get_piece(id).await?);
        }
        Ok(pieces)
    }

    /// Search for pieces matching a query.
    async fn search(&self, query: &str, options: SearchOptions)
        -> FrameworkResult<SearchResult<P>>;

    /// Autocomplete piece names.
    ///
    /// Returns a list of matching names for the given prefix.
    async fn autocomplete(&self, prefix: &str, limit: usize) -> FrameworkResult<Vec<String>>;

    /// Get bulk data for offline use.
    ///
    /// This may return a large dataset; use with caution.
    async fn bulk_download(&self) -> FrameworkResult<BulkData<P>>;

    /// Check for updates since a timestamp.
    ///
    /// Returns pieces that have been added or modified since the given time.
    async fn updates_since(&self, since: DateTime<Utc>) -> FrameworkResult<Vec<P>>;

    /// Get data source metadata.
    fn metadata(&self) -> DataSourceMetadata;
}

/// Options for searching pieces.
#[derive(Debug, Clone, Default)]
pub struct SearchOptions {
    /// Maximum results to return
    pub limit: usize,

    /// Offset for pagination
    pub offset: usize,

    /// Sort field
    pub sort_by: Option<String>,

    /// Sort direction
    pub sort_desc: bool,

    /// Filter by category
    pub category: Option<String>,

    /// Additional filters (game-specific)
    pub filters: std::collections::HashMap<String, String>,
}

impl SearchOptions {
    /// Create new search options with default limit
    pub fn new() -> Self {
        Self {
            limit: 20,
            ..Default::default()
        }
    }

    /// Set limit
    pub fn with_limit(mut self, limit: usize) -> Self {
        self.limit = limit;
        self
    }

    /// Set offset
    pub fn with_offset(mut self, offset: usize) -> Self {
        self.offset = offset;
        self
    }

    /// Set sort
    pub fn with_sort(mut self, field: impl Into<String>, desc: bool) -> Self {
        self.sort_by = Some(field.into());
        self.sort_desc = desc;
        self
    }

    /// Add filter
    pub fn with_filter(mut self, key: impl Into<String>, value: impl Into<String>) -> Self {
        self.filters.insert(key.into(), value.into());
        self
    }
}

/// Result of a search operation.
#[derive(Debug, Clone)]
pub struct SearchResult<P> {
    /// Matching pieces
    pub pieces: Vec<P>,

    /// Total count (may be more than returned)
    pub total_count: usize,

    /// Whether there are more results
    pub has_more: bool,

    /// Token for fetching next page (if applicable)
    pub next_page_token: Option<String>,
}

/// Bulk data download result.
#[derive(Debug)]
pub struct BulkData<P> {
    /// All pieces
    pub pieces: Vec<P>,

    /// When this data was generated
    pub generated_at: DateTime<Utc>,

    /// Data version/hash
    pub version: String,

    /// Size in bytes (compressed)
    pub size_bytes: usize,
}

/// Metadata about a data source.
#[derive(Debug, Clone)]
pub struct DataSourceMetadata {
    /// Data source name
    pub name: String,

    /// Base URL
    pub base_url: String,

    /// API version
    pub api_version: String,

    /// Rate limit (requests per second)
    pub rate_limit: Option<f64>,

    /// Whether authentication is required
    pub requires_auth: bool,

    /// Attribution text (for display)
    pub attribution: Option<String>,
}

impl DataSourceMetadata {
    /// Create new metadata
    pub fn new(name: impl Into<String>, base_url: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            base_url: base_url.into(),
            api_version: "1.0".into(),
            rate_limit: None,
            requires_auth: false,
            attribution: None,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_search_options() {
        let options = SearchOptions::new()
            .with_limit(50)
            .with_offset(100)
            .with_sort("name", false)
            .with_filter("color", "red");

        assert_eq!(options.limit, 50);
        assert_eq!(options.offset, 100);
        assert_eq!(options.sort_by, Some("name".into()));
        assert!(!options.sort_desc);
        assert_eq!(options.filters.get("color"), Some(&"red".into()));
    }
}
