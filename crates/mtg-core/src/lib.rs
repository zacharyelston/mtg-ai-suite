//! MTG Core Library
//!
//! This crate provides the core types and algorithms shared between
//! the server, CLI, and WASM modules.
//!
//! # Modules
//!
//! - [`card`] - Card data structures and parsing
//! - [`recognition`] - Card recognition algorithms
//! - [`fuzzy`] - Fuzzy string matching for card names
//! - [`error`] - Error types
//! - [`image`] - Image processing utilities

pub mod card;
pub mod error;
pub mod fuzzy;
pub mod image_processing;
pub mod recognition;

pub use card::{Card, CardColor, CardType};
pub use error::{Error, Result};
pub use fuzzy::FuzzyMatcher;
pub use recognition::{RecognitionResult, RecognitionService};
