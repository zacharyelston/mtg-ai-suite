//! GameRules trait - encapsulates game-specific rules

use super::{GameCollection, GamePiece, GameState};
use crate::error::FrameworkResult;
use crate::types::ValidationResult;

/// Game rules engine.
///
/// This trait encapsulates all game-specific rules, including:
/// - Collection validation (deck legality)
/// - Action legality
/// - State transitions
/// - Win conditions
///
/// # Example
///
/// ```rust,ignore
/// use cardgame_core::traits::GameRules;
///
/// struct MtgRules {
///     comprehensive_rules: ComprehensiveRules,
/// }
///
/// impl GameRules for MtgRules {
///     type Piece = MtgCard;
///     type State = MtgGameState;
///     type Format = MtgFormat;
///     // ...
/// }
/// ```
pub trait GameRules: Send + Sync {
    /// The type of game piece
    type Piece: GamePiece;

    /// The type of game state
    type State: GameState<Piece = Self::Piece>;

    /// The type of game format
    type Format: GameFormat;

    /// Validate a collection for a specific format.
    ///
    /// Returns a `ValidationResult` with any errors or warnings.
    fn validate_collection(
        &self,
        collection: &dyn GameCollection<Self::Piece>,
        format: &Self::Format,
    ) -> ValidationResult;

    /// Check if an action is legal in the current state.
    fn is_legal_action(
        &self,
        state: &Self::State,
        action: &<Self::State as GameState>::Action,
    ) -> bool;

    /// Get all legal actions from the current state.
    fn legal_actions(&self, state: &Self::State) -> Vec<<Self::State as GameState>::Action>;

    /// Apply an action to a state, producing a new state.
    ///
    /// Returns an error if the action is illegal.
    fn apply_action(
        &self,
        state: &Self::State,
        action: &<Self::State as GameState>::Action,
    ) -> FrameworkResult<Self::State>;

    /// Check if the game is over.
    fn is_game_over(&self, state: &Self::State) -> bool;

    /// Determine the winner (if game is over).
    ///
    /// Returns `None` if game is not over or is a draw.
    fn winner(&self, state: &Self::State) -> Option<String>;

    /// Get available formats for this game.
    fn available_formats(&self) -> Vec<Self::Format>;

    /// Get the default format.
    fn default_format(&self) -> Self::Format;
}

/// A game format (e.g., Standard, Modern, Commander).
pub trait GameFormat: Send + Sync + Clone + std::fmt::Debug {
    /// Format identifier (e.g., "standard", "modern")
    fn id(&self) -> &str;

    /// Display name
    fn name(&self) -> &str;

    /// Description
    fn description(&self) -> &str;

    /// Minimum collection size (e.g., 60 for Standard)
    fn min_size(&self) -> Option<u32>;

    /// Maximum collection size (e.g., 100 for Commander)
    fn max_size(&self) -> Option<u32>;

    /// Maximum copies of a single piece (e.g., 4 for most formats)
    fn max_copies(&self) -> Option<u32>;

    /// Whether this format is currently active/supported
    fn is_active(&self) -> bool {
        true
    }
}

/// Standard format configuration.
#[derive(Debug, Clone)]
pub struct FormatConfig {
    pub id: String,
    pub name: String,
    pub description: String,
    pub min_size: Option<u32>,
    pub max_size: Option<u32>,
    pub max_copies: Option<u32>,
    pub is_active: bool,
}

impl GameFormat for FormatConfig {
    fn id(&self) -> &str {
        &self.id
    }

    fn name(&self) -> &str {
        &self.name
    }

    fn description(&self) -> &str {
        &self.description
    }

    fn min_size(&self) -> Option<u32> {
        self.min_size
    }

    fn max_size(&self) -> Option<u32> {
        self.max_size
    }

    fn max_copies(&self) -> Option<u32> {
        self.max_copies
    }

    fn is_active(&self) -> bool {
        self.is_active
    }
}

impl FormatConfig {
    /// Create a new format configuration
    pub fn new(id: impl Into<String>, name: impl Into<String>) -> Self {
        Self {
            id: id.into(),
            name: name.into(),
            description: String::new(),
            min_size: None,
            max_size: None,
            max_copies: None,
            is_active: true,
        }
    }

    /// Set description
    pub fn with_description(mut self, desc: impl Into<String>) -> Self {
        self.description = desc.into();
        self
    }

    /// Set size constraints
    pub fn with_size(mut self, min: u32, max: Option<u32>) -> Self {
        self.min_size = Some(min);
        self.max_size = max;
        self
    }

    /// Set max copies
    pub fn with_max_copies(mut self, max: u32) -> Self {
        self.max_copies = Some(max);
        self
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_format_config() {
        let format = FormatConfig::new("standard", "Standard")
            .with_description("Rotating format with recent sets")
            .with_size(60, None)
            .with_max_copies(4);

        assert_eq!(format.id(), "standard");
        assert_eq!(format.min_size(), Some(60));
        assert_eq!(format.max_copies(), Some(4));
    }
}
