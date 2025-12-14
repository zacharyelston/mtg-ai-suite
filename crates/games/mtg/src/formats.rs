//! MTG Formats - implements GameFormat trait

use cardgame_core::traits::GameFormat;

#[derive(Debug, Clone)]
pub struct MtgFormat {
    pub id: String,
    pub name: String,
    pub min_deck: u32,
    pub max_deck: Option<u32>,
    pub max_copies: u32,
}

impl GameFormat for MtgFormat {
    fn id(&self) -> &str {
        &self.id
    }
    fn name(&self) -> &str {
        &self.name
    }
    fn description(&self) -> &str {
        ""
    }
    fn min_size(&self) -> Option<u32> {
        Some(self.min_deck)
    }
    fn max_size(&self) -> Option<u32> {
        self.max_deck
    }
    fn max_copies(&self) -> Option<u32> {
        Some(self.max_copies)
    }
}

pub fn standard() -> MtgFormat {
    MtgFormat {
        id: "standard".into(),
        name: "Standard".into(),
        min_deck: 60,
        max_deck: None,
        max_copies: 4,
    }
}

pub fn modern() -> MtgFormat {
    MtgFormat {
        id: "modern".into(),
        name: "Modern".into(),
        min_deck: 60,
        max_deck: None,
        max_copies: 4,
    }
}

pub fn commander() -> MtgFormat {
    MtgFormat {
        id: "commander".into(),
        name: "Commander".into(),
        min_deck: 100,
        max_deck: Some(100),
        max_copies: 1,
    }
}
