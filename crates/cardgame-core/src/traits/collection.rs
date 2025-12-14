//! GameCollection trait - represents a collection of game pieces

use super::piece::GamePiece;
use crate::types::ValidationResult;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use uuid::Uuid;

/// A collection of game pieces (deck, hand, board, etc.)
///
/// This trait represents any grouping of game pieces, such as:
/// - A constructed deck
/// - A player's hand
/// - Cards on the battlefield
/// - A sideboard
///
/// # Example
///
/// ```rust,ignore
/// use cardgame_core::traits::{GameCollection, GamePiece};
///
/// struct MtgDeck {
///     id: Uuid,
///     name: String,
///     format: String,
///     cards: Vec<DeckEntry>,
/// }
///
/// impl GameCollection<MtgCard> for MtgDeck {
///     fn id(&self) -> Uuid { self.id }
///     fn name(&self) -> &str { &self.name }
///     // ...
/// }
/// ```
pub trait GameCollection<P: GamePiece>: Send + Sync {
    /// Unique identifier for this collection.
    fn id(&self) -> Uuid;

    /// Human-readable name of this collection.
    fn name(&self) -> &str;

    /// Optional description of this collection.
    fn description(&self) -> Option<&str> {
        None
    }

    /// Get all entries in this collection.
    fn entries(&self) -> Vec<CollectionEntry<P::Id>>;

    /// Get total count of pieces (sum of quantities).
    fn total_count(&self) -> u32 {
        self.entries().iter().map(|e| e.quantity).sum()
    }

    /// Get count of unique pieces.
    fn unique_count(&self) -> usize {
        self.entries().len()
    }

    /// Check if collection contains a specific piece.
    fn contains(&self, piece_id: &P::Id) -> bool {
        self.entries().iter().any(|e| &e.piece_id == piece_id)
    }

    /// Get quantity of a specific piece.
    fn quantity_of(&self, piece_id: &P::Id) -> u32 {
        self.entries()
            .iter()
            .find(|e| &e.piece_id == piece_id)
            .map(|e| e.quantity)
            .unwrap_or(0)
    }

    /// Add a piece to the collection.
    ///
    /// Returns the new quantity of that piece.
    fn add(&mut self, piece_id: P::Id, quantity: u32) -> u32;

    /// Remove a piece from the collection.
    ///
    /// Returns `true` if the piece was removed, `false` if not found
    /// or insufficient quantity.
    fn remove(&mut self, piece_id: &P::Id, quantity: u32) -> bool;

    /// Clear all pieces from the collection.
    fn clear(&mut self);

    /// Get the format/type of this collection (e.g., "standard", "commander").
    fn format(&self) -> Option<&str> {
        None
    }

    /// Get collection metadata.
    fn metadata(&self) -> HashMap<String, serde_json::Value> {
        HashMap::new()
    }
}

/// An entry in a game collection.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CollectionEntry<Id> {
    /// The piece ID
    pub piece_id: Id,

    /// Quantity of this piece
    pub quantity: u32,

    /// Zone or section within the collection (e.g., "mainboard", "sideboard")
    pub zone: Option<String>,

    /// Additional metadata for this entry
    #[serde(default)]
    pub metadata: HashMap<String, serde_json::Value>,
}

impl<Id: Clone> CollectionEntry<Id> {
    /// Create a new collection entry
    pub fn new(piece_id: Id, quantity: u32) -> Self {
        Self {
            piece_id,
            quantity,
            zone: None,
            metadata: HashMap::new(),
        }
    }

    /// Create entry with zone
    pub fn with_zone(piece_id: Id, quantity: u32, zone: impl Into<String>) -> Self {
        Self {
            piece_id,
            quantity,
            zone: Some(zone.into()),
            metadata: HashMap::new(),
        }
    }
}

/// A mutable collection that can be validated.
pub trait ValidatableCollection<P: GamePiece>: GameCollection<P> {
    /// Validate this collection against game rules.
    ///
    /// Returns a `ValidationResult` with any errors or warnings.
    fn validate(&self) -> ValidationResult;
}

/// Builder for creating collections.
pub struct CollectionBuilder<Id> {
    entries: Vec<CollectionEntry<Id>>,
}

impl<Id: Clone> CollectionBuilder<Id> {
    /// Create a new builder
    pub fn new() -> Self {
        Self { entries: vec![] }
    }

    /// Add a piece
    pub fn add(mut self, piece_id: Id, quantity: u32) -> Self {
        self.entries.push(CollectionEntry::new(piece_id, quantity));
        self
    }

    /// Add a piece to a specific zone
    pub fn add_to_zone(mut self, piece_id: Id, quantity: u32, zone: impl Into<String>) -> Self {
        self.entries
            .push(CollectionEntry::with_zone(piece_id, quantity, zone));
        self
    }

    /// Build the entries
    pub fn build(self) -> Vec<CollectionEntry<Id>> {
        self.entries
    }
}

impl<Id: Clone> Default for CollectionBuilder<Id> {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_collection_entry() {
        let entry = CollectionEntry::new("card-1".to_string(), 4);
        assert_eq!(entry.quantity, 4);
        assert!(entry.zone.is_none());
    }

    #[test]
    fn test_collection_entry_with_zone() {
        let entry = CollectionEntry::with_zone("card-1".to_string(), 4, "sideboard");
        assert_eq!(entry.zone, Some("sideboard".to_string()));
    }

    #[test]
    fn test_collection_builder() {
        let entries: Vec<CollectionEntry<String>> = CollectionBuilder::new()
            .add("card-1".into(), 4)
            .add("card-2".into(), 2)
            .add_to_zone("card-3".into(), 1, "sideboard")
            .build();

        assert_eq!(entries.len(), 3);
        assert_eq!(entries[0].quantity, 4);
        assert_eq!(entries[2].zone, Some("sideboard".to_string()));
    }
}
