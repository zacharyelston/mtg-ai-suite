//! Magic: The Gathering Implementation
//!
//! This crate implements the Card Game AI Framework traits for MTG.

pub mod card;
pub mod deck;
pub mod formats;
pub mod scryfall;

pub use card::MtgCard;
pub use deck::MtgDeck;
pub use formats::MtgFormat;
