//! Game Implementation Template
//!
//! Copy this crate and implement the traits for your game.
//! See README.md for detailed instructions.

use cardgame_core::prelude::*;

// ═══════════════════════════════════════════════════════════════════════════
// STEP 1: Define your game piece (card, token, tile, etc.)
// ═══════════════════════════════════════════════════════════════════════════

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct MyGamePiece {
    pub id: String,
    pub name: String,
    // Add game-specific fields here
}

impl GamePiece for MyGamePiece {
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
        "piece"
    }
}

// ═══════════════════════════════════════════════════════════════════════════
// STEP 2: Define your collection type (deck, hand, etc.)
// ═══════════════════════════════════════════════════════════════════════════

pub struct MyGameCollection {
    id: uuid::Uuid,
    name: String,
    entries: Vec<CollectionEntry<String>>,
}

impl GameCollection<MyGamePiece> for MyGameCollection {
    fn id(&self) -> uuid::Uuid {
        self.id
    }
    fn name(&self) -> &str {
        &self.name
    }
    fn entries(&self) -> Vec<CollectionEntry<String>> {
        self.entries.clone()
    }
    fn add(&mut self, piece_id: String, quantity: u32) -> u32 {
        self.entries.push(CollectionEntry::new(piece_id, quantity));
        quantity
    }
    fn remove(&mut self, piece_id: &String, _quantity: u32) -> bool {
        self.entries.retain(|e| &e.piece_id != piece_id);
        true
    }
    fn clear(&mut self) {
        self.entries.clear();
    }
}

// ═══════════════════════════════════════════════════════════════════════════
// STEP 3: Define game state, actions, and zones
// ═══════════════════════════════════════════════════════════════════════════

// See cardgame_core::traits::state for GameState trait requirements

// ═══════════════════════════════════════════════════════════════════════════
// STEP 4: Implement GameRules
// ═══════════════════════════════════════════════════════════════════════════

// See cardgame_core::traits::rules for GameRules trait requirements

// ═══════════════════════════════════════════════════════════════════════════
// STEP 5: Implement GameDataSource (connect to external API)
// ═══════════════════════════════════════════════════════════════════════════

// See cardgame_core::traits::datasource for GameDataSource trait requirements

// ═══════════════════════════════════════════════════════════════════════════
// STEP 6: Implement RecognitionProvider
// ═══════════════════════════════════════════════════════════════════════════

// See cardgame_core::traits::recognition for RecognitionProvider trait requirements

// ═══════════════════════════════════════════════════════════════════════════
// STEP 7: Implement PlayAdvisor
// ═══════════════════════════════════════════════════════════════════════════

// See cardgame_core::traits::advisor for PlayAdvisor trait requirements
