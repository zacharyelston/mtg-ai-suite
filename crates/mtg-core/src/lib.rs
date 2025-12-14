//! MTG Core Library
//!
//! This crate provides the core types and algorithms shared between
//! the server, CLI, and WASM modules.
//!
//! # Modules
//!
//! - [`card`] - Card data structures and parsing (requires `full` feature)
//! - [`recognition`] - Card recognition algorithms (requires `full` feature)
//! - [`fuzzy`] - Fuzzy string matching for card names
//! - [`error`] - Error types
//! - [`image_processing`] - Image processing utilities (requires `full` feature)

pub mod error;
pub mod fuzzy;

#[cfg(feature = "full")]
pub mod card;
#[cfg(feature = "full")]
pub mod image_processing;
#[cfg(feature = "full")]
pub mod recognition;

pub use error::{Error, Result};
pub use fuzzy::FuzzyMatcher;

#[cfg(feature = "full")]
pub use card::{Card, CardColor, CardType};
#[cfg(feature = "full")]
pub use recognition::{RecognitionResult, RecognitionService};
