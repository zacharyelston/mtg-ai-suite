//! MTG Deck - implements GameCollection trait

use crate::card::MtgCard;
use cardgame_core::prelude::*;
use uuid::Uuid;

/// MTG Deck
pub struct MtgDeck {
    pub id: Uuid,
    pub name: String,
    pub format: String,
    pub mainboard: Vec<CollectionEntry<String>>,
    pub sideboard: Vec<CollectionEntry<String>>,
}

impl MtgDeck {
    pub fn new(name: impl Into<String>, format: impl Into<String>) -> Self {
        Self {
            id: Uuid::new_v4(),
            name: name.into(),
            format: format.into(),
            mainboard: vec![],
            sideboard: vec![],
        }
    }
}

impl GameCollection<MtgCard> for MtgDeck {
    fn id(&self) -> Uuid {
        self.id
    }

    fn name(&self) -> &str {
        &self.name
    }

    fn format(&self) -> Option<&str> {
        Some(&self.format)
    }

    fn entries(&self) -> Vec<CollectionEntry<String>> {
        let mut all = self.mainboard.clone();
        for entry in &self.sideboard {
            all.push(CollectionEntry::with_zone(
                entry.piece_id.clone(),
                entry.quantity,
                "sideboard",
            ));
        }
        all
    }

    fn add(&mut self, piece_id: String, quantity: u32) -> u32 {
        if let Some(entry) = self.mainboard.iter_mut().find(|e| e.piece_id == piece_id) {
            entry.quantity += quantity;
            entry.quantity
        } else {
            self.mainboard
                .push(CollectionEntry::new(piece_id, quantity));
            quantity
        }
    }

    fn remove(&mut self, piece_id: &String, quantity: u32) -> bool {
        if let Some(entry) = self.mainboard.iter_mut().find(|e| &e.piece_id == piece_id) {
            if entry.quantity <= quantity {
                self.mainboard.retain(|e| &e.piece_id != piece_id);
            } else {
                entry.quantity -= quantity;
            }
            true
        } else {
            false
        }
    }

    fn clear(&mut self) {
        self.mainboard.clear();
        self.sideboard.clear();
    }
}
