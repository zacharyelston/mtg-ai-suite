//! MTG Card - implements GamePiece trait

use cardgame_core::prelude::*;
use serde::{Deserialize, Serialize};

/// Magic: The Gathering card
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MtgCard {
    pub scryfall_id: String,
    pub oracle_id: Option<String>,
    pub name: String,
    pub mana_cost: Option<String>,
    pub cmc: f32,
    pub type_line: String,
    pub oracle_text: Option<String>,
    pub power: Option<String>,
    pub toughness: Option<String>,
    pub colors: Vec<MtgColor>,
    pub color_identity: Vec<MtgColor>,
    pub keywords: Vec<String>,
    pub set_code: String,
    pub rarity: MtgRarity,
    pub image_uri: Option<String>,
}

impl GamePiece for MtgCard {
    type Id = String;

    fn name(&self) -> &str {
        &self.name
    }

    fn id(&self) -> &Self::Id {
        &self.scryfall_id
    }

    fn image_url(&self) -> Option<&str> {
        self.image_uri.as_deref()
    }

    fn searchable_text(&self) -> String {
        format!(
            "{} {} {} {}",
            self.name,
            self.type_line,
            self.oracle_text.as_deref().unwrap_or(""),
            self.keywords.join(" ")
        )
    }

    fn category(&self) -> &str {
        if self.type_line.to_lowercase().contains("creature") {
            "creature"
        } else if self.type_line.to_lowercase().contains("instant") {
            "instant"
        } else if self.type_line.to_lowercase().contains("sorcery") {
            "sorcery"
        } else if self.type_line.to_lowercase().contains("land") {
            "land"
        } else if self.type_line.to_lowercase().contains("enchantment") {
            "enchantment"
        } else if self.type_line.to_lowercase().contains("artifact") {
            "artifact"
        } else if self.type_line.to_lowercase().contains("planeswalker") {
            "planeswalker"
        } else {
            "other"
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum MtgColor {
    White,
    Blue,
    Black,
    Red,
    Green,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum MtgRarity {
    Common,
    Uncommon,
    Rare,
    Mythic,
}
