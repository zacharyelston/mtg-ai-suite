//! Card data structures
//!
//! This module defines the core card types used throughout the application.

use serde::{Deserialize, Serialize};
use uuid::Uuid;

/// Represents a Magic: The Gathering card
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Card {
    /// Internal UUID
    pub id: Uuid,

    /// Scryfall ID for external reference
    pub scryfall_id: String,

    /// Oracle ID (shared across printings)
    pub oracle_id: Option<String>,

    /// Card name
    pub name: String,

    /// Mana cost (e.g., "{2}{U}{U}")
    pub mana_cost: Option<String>,

    /// Converted mana cost / mana value
    pub cmc: f32,

    /// Type line (e.g., "Creature — Human Wizard")
    pub type_line: String,

    /// Oracle text (rules text)
    pub oracle_text: Option<String>,

    /// Power (for creatures)
    pub power: Option<String>,

    /// Toughness (for creatures)
    pub toughness: Option<String>,

    /// Card colors
    pub colors: Vec<CardColor>,

    /// Color identity (for Commander)
    pub color_identity: Vec<CardColor>,

    /// Keywords (Flying, Trample, etc.)
    pub keywords: Vec<String>,

    /// Set code (e.g., "MH2")
    pub set_code: String,

    /// Set name
    pub set_name: String,

    /// Rarity
    pub rarity: CardRarity,

    /// Image URIs
    pub image_uris: Option<ImageUris>,

    /// Format legalities
    pub legalities: Legalities,
}

/// Card colors
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq, Hash)]
#[serde(rename_all = "lowercase")]
pub enum CardColor {
    White,
    Blue,
    Black,
    Red,
    Green,
}

impl CardColor {
    /// Returns the single-letter code for this color
    pub fn code(&self) -> char {
        match self {
            CardColor::White => 'W',
            CardColor::Blue => 'U',
            CardColor::Black => 'B',
            CardColor::Red => 'R',
            CardColor::Green => 'G',
        }
    }

    /// Parse from single-letter code
    pub fn from_code(code: char) -> Option<Self> {
        match code.to_ascii_uppercase() {
            'W' => Some(CardColor::White),
            'U' => Some(CardColor::Blue),
            'B' => Some(CardColor::Black),
            'R' => Some(CardColor::Red),
            'G' => Some(CardColor::Green),
            _ => None,
        }
    }
}

/// Card types
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "lowercase")]
pub enum CardType {
    Creature,
    Instant,
    Sorcery,
    Enchantment,
    Artifact,
    Planeswalker,
    Land,
    Battle,
}

/// Card rarity
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "lowercase")]
pub enum CardRarity {
    Common,
    Uncommon,
    Rare,
    Mythic,
    Special,
    Bonus,
}

/// Image URIs from Scryfall
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ImageUris {
    pub small: Option<String>,
    pub normal: Option<String>,
    pub large: Option<String>,
    pub png: Option<String>,
    pub art_crop: Option<String>,
    pub border_crop: Option<String>,
}

/// Format legalities
#[derive(Debug, Clone, Default, Serialize, Deserialize, PartialEq)]
pub struct Legalities {
    pub standard: Legality,
    pub pioneer: Legality,
    pub modern: Legality,
    pub legacy: Legality,
    pub vintage: Legality,
    pub commander: Legality,
    pub pauper: Legality,
}

/// Legality status
#[derive(Debug, Clone, Copy, Default, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "lowercase")]
pub enum Legality {
    Legal,
    #[default]
    NotLegal,
    Banned,
    Restricted,
}

/// Simplified card info for matching
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CardName {
    pub name: String,
    pub scryfall_id: String,
}

impl Card {
    /// Check if card is a creature
    pub fn is_creature(&self) -> bool {
        self.type_line.to_lowercase().contains("creature")
    }

    /// Check if card is an instant
    pub fn is_instant(&self) -> bool {
        self.type_line.to_lowercase().contains("instant")
    }

    /// Check if card is a land
    pub fn is_land(&self) -> bool {
        self.type_line.to_lowercase().contains("land")
    }

    /// Get primary card type
    pub fn primary_type(&self) -> Option<CardType> {
        let type_lower = self.type_line.to_lowercase();
        if type_lower.contains("creature") {
            Some(CardType::Creature)
        } else if type_lower.contains("instant") {
            Some(CardType::Instant)
        } else if type_lower.contains("sorcery") {
            Some(CardType::Sorcery)
        } else if type_lower.contains("enchantment") {
            Some(CardType::Enchantment)
        } else if type_lower.contains("artifact") {
            Some(CardType::Artifact)
        } else if type_lower.contains("planeswalker") {
            Some(CardType::Planeswalker)
        } else if type_lower.contains("land") {
            Some(CardType::Land)
        } else if type_lower.contains("battle") {
            Some(CardType::Battle)
        } else {
            None
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_color_code() {
        assert_eq!(CardColor::White.code(), 'W');
        assert_eq!(CardColor::Blue.code(), 'U');
        assert_eq!(CardColor::Black.code(), 'B');
        assert_eq!(CardColor::Red.code(), 'R');
        assert_eq!(CardColor::Green.code(), 'G');
    }

    #[test]
    fn test_color_from_code() {
        assert_eq!(CardColor::from_code('W'), Some(CardColor::White));
        assert_eq!(CardColor::from_code('u'), Some(CardColor::Blue));
        assert_eq!(CardColor::from_code('X'), None);
    }

    #[test]
    fn test_card_type_detection() {
        let card = Card {
            id: Uuid::new_v4(),
            scryfall_id: "test".into(),
            oracle_id: None,
            name: "Lightning Bolt".into(),
            mana_cost: Some("{R}".into()),
            cmc: 1.0,
            type_line: "Instant".into(),
            oracle_text: Some("Lightning Bolt deals 3 damage to any target.".into()),
            power: None,
            toughness: None,
            colors: vec![CardColor::Red],
            color_identity: vec![CardColor::Red],
            keywords: vec![],
            set_code: "M10".into(),
            set_name: "Magic 2010".into(),
            rarity: CardRarity::Common,
            image_uris: None,
            legalities: Legalities::default(),
        };

        assert!(card.is_instant());
        assert!(!card.is_creature());
        assert_eq!(card.primary_type(), Some(CardType::Instant));
    }
}
