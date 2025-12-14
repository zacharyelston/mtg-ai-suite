//! GamePiece trait - represents any identifiable game component

use serde::{de::DeserializeOwned, Serialize};
use std::fmt::Debug;
use std::hash::Hash;

/// A game piece that can be recognized and tracked.
///
/// This trait represents any identifiable component in a card or board game,
/// such as cards, tokens, tiles, or miniatures.
///
/// # Type Parameters
///
/// The associated `Id` type must be:
/// - `Clone` - IDs are frequently copied
/// - `Eq + Hash` - IDs are used as map keys
/// - `Serialize + DeserializeOwned` - IDs are stored and transmitted
///
/// # Example
///
/// ```rust,ignore
/// use cardgame_core::traits::GamePiece;
///
/// struct MtgCard {
///     scryfall_id: String,
///     name: String,
///     mana_cost: Option<String>,
///     image_url: String,
/// }
///
/// impl GamePiece for MtgCard {
///     type Id = String;
///
///     fn name(&self) -> &str { &self.name }
///     fn id(&self) -> &Self::Id { &self.scryfall_id }
///     fn image_url(&self) -> Option<&str> { Some(&self.image_url) }
///     fn searchable_text(&self) -> String {
///         format!("{} {}", self.name, self.mana_cost.as_deref().unwrap_or(""))
///     }
///     fn category(&self) -> &str { "card" }
/// }
/// ```
pub trait GamePiece: Send + Sync + Debug + Clone {
    /// The type used to uniquely identify this piece.
    ///
    /// Common choices:
    /// - `String` - For API-provided IDs (Scryfall, etc.)
    /// - `Uuid` - For locally-generated IDs
    /// - `u64` - For database IDs
    type Id: Clone + Eq + Hash + Serialize + DeserializeOwned + Debug + Send + Sync;

    /// Human-readable name of this piece.
    ///
    /// This is the primary display name shown to users.
    fn name(&self) -> &str;

    /// Unique identifier for this piece.
    ///
    /// This ID should be stable across sessions and unique within
    /// the game's piece universe.
    fn id(&self) -> &Self::Id;

    /// URL to an image of this piece.
    ///
    /// Returns `None` if no image is available.
    fn image_url(&self) -> Option<&str>;

    /// Text used for searching and matching.
    ///
    /// This should include all text that users might search for,
    /// such as name, description, keywords, etc.
    fn searchable_text(&self) -> String;

    /// Category or type of this piece.
    ///
    /// Used for filtering and grouping. Examples:
    /// - MTG: "creature", "instant", "land"
    /// - Pokemon: "pokemon", "trainer", "energy"
    /// - Chess: "pawn", "knight", "queen"
    fn category(&self) -> &str;

    /// Additional metadata as key-value pairs.
    ///
    /// Override this to provide game-specific metadata.
    fn metadata(&self) -> std::collections::HashMap<String, serde_json::Value> {
        std::collections::HashMap::new()
    }

    /// Check if this piece matches a search query.
    ///
    /// Default implementation does case-insensitive substring matching
    /// on the searchable text.
    fn matches_query(&self, query: &str) -> bool {
        self.searchable_text()
            .to_lowercase()
            .contains(&query.to_lowercase())
    }
}

/// A reference to a game piece by ID, with optional quantity.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PieceRef<Id> {
    /// The piece ID
    pub id: Id,
    /// Quantity (default 1)
    pub quantity: u32,
}

impl<Id: Clone> PieceRef<Id> {
    /// Create a new piece reference with quantity 1
    pub fn new(id: Id) -> Self {
        Self { id, quantity: 1 }
    }

    /// Create a new piece reference with specified quantity
    pub fn with_quantity(id: Id, quantity: u32) -> Self {
        Self { id, quantity }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[derive(Debug, Clone)]
    struct TestPiece {
        id: String,
        name: String,
    }

    impl GamePiece for TestPiece {
        type Id = String;

        fn name(&self) -> &str {
            &self.name
        }

        fn id(&self) -> &Self::Id {
            &self.id
        }

        fn image_url(&self) -> Option<&str> {
            None
        }

        fn searchable_text(&self) -> String {
            self.name.clone()
        }

        fn category(&self) -> &str {
            "test"
        }
    }

    #[test]
    fn test_matches_query() {
        let piece = TestPiece {
            id: "1".into(),
            name: "Lightning Bolt".into(),
        };

        assert!(piece.matches_query("lightning"));
        assert!(piece.matches_query("BOLT"));
        assert!(!piece.matches_query("counter"));
    }

    #[test]
    fn test_piece_ref() {
        let piece_ref = PieceRef::with_quantity("card-1".to_string(), 4);
        assert_eq!(piece_ref.quantity, 4);
    }
}
