//! Card Game AI Framework - Core Library
//!
//! This crate provides the core traits and types for building AI-powered
//! assistants for card and board games. It is game-agnostic and designed
//! to be implemented by specific game modules.
//!
//! # Architecture
//!
//! The framework separates concerns into:
//! - **Framework Layer**: Game-agnostic components (capture, recognition, sync, LLM)
//! - **Game Layer**: Game-specific implementations (MTG, Pokemon, etc.)
//!
//! # Core Traits
//!
//! - [`GamePiece`] - Any identifiable game component (card, token, tile)
//! - [`GameCollection`] - A collection of pieces (deck, hand, board)
//! - [`GameState`] - Current state of a game in progress
//! - [`GameRules`] - Game-specific rules engine
//! - [`GameDataSource`] - External data provider for game pieces
//! - [`PlayAdvisor`] - AI-powered play suggestions
//! - [`RecognitionProvider`] - Game-specific piece recognition
//!
//! # Example
//!
//! ```rust,ignore
//! use cardgame_core::prelude::*;
//!
//! // Implement GamePiece for your card type
//! impl GamePiece for MyCard {
//!     type Id = String;
//!     fn name(&self) -> &str { &self.name }
//!     fn id(&self) -> &Self::Id { &self.id }
//!     // ...
//! }
//! ```

pub mod error;
pub mod traits;
pub mod types;

/// Prelude for convenient imports
pub mod prelude {
    pub use crate::error::*;
    pub use crate::traits::*;
    pub use crate::types::*;
}

pub use error::{FrameworkError, FrameworkResult};
