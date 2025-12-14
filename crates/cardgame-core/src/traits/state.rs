//! GameState trait - represents the current state of a game

use super::piece::GamePiece;
use serde::{de::DeserializeOwned, Deserialize, Serialize};
use std::fmt::Debug;

/// Current state of a game in progress.
///
/// This trait represents a snapshot of the game at a point in time,
/// including all players, pieces, and game-specific state.
///
/// # Design Notes
///
/// - State should be immutable; actions produce new states
/// - State should be serializable for persistence and LLM context
/// - State should support cloning for exploring action trees
///
/// # Example
///
/// ```rust,ignore
/// use cardgame_core::traits::GameState;
///
/// struct MtgGameState {
///     turn: u32,
///     phase: MtgPhase,
///     active_player: PlayerId,
///     players: Vec<MtgPlayer>,
///     battlefield: Vec<Permanent>,
///     stack: Vec<StackObject>,
/// }
///
/// impl GameState for MtgGameState {
///     type Piece = MtgCard;
///     type Action = MtgAction;
///     // ...
/// }
/// ```
pub trait GameState: Send + Sync + Clone + Debug + Serialize + DeserializeOwned {
    /// The type of game piece in this game
    type Piece: GamePiece;

    /// The type of action that can be taken
    type Action: GameAction;

    /// The type of zone/location in the game
    type Zone: GameZone;

    /// Current turn number (1-indexed)
    fn turn(&self) -> u32;

    /// Current phase or step name
    fn phase(&self) -> &str;

    /// ID of the active player (whose turn it is)
    fn active_player_id(&self) -> &str;

    /// IDs of all players in the game
    fn player_ids(&self) -> Vec<String>;

    /// Get pieces in a specific zone for a player
    fn pieces_in_zone(&self, player_id: &str, zone: &Self::Zone) -> Vec<PieceInPlay<Self::Piece>>;

    /// Get all legal actions from current state
    fn legal_actions(&self) -> Vec<Self::Action>;

    /// Check if a specific action is legal
    fn is_legal(&self, action: &Self::Action) -> bool {
        self.legal_actions().contains(action)
    }

    /// Check if the game is over
    fn is_game_over(&self) -> bool;

    /// Get the winner (if game is over)
    fn winner(&self) -> Option<String>;

    /// Serialize state to a string for LLM context
    fn to_llm_context(&self) -> String;

    /// Get a summary of the current state (for display)
    fn summary(&self) -> String;
}

/// A game action that can be taken.
pub trait GameAction:
    Send + Sync + Clone + Debug + PartialEq + Serialize + DeserializeOwned
{
    /// Human-readable description of this action
    fn description(&self) -> String;

    /// Short name for this action type
    fn action_type(&self) -> &str;

    /// Whether this action passes priority/ends turn
    fn is_pass(&self) -> bool;
}

/// A zone or location in the game.
pub trait GameZone: Send + Sync + Clone + Debug + PartialEq + Serialize + DeserializeOwned {
    /// Zone name
    fn name(&self) -> &str;

    /// Whether pieces in this zone are visible to all players
    fn is_public(&self) -> bool;

    /// Whether pieces in this zone are ordered
    fn is_ordered(&self) -> bool;
}

/// A piece that is in play (on the board, in hand, etc.)
#[derive(Debug, Clone, Serialize)]
pub struct PieceInPlay<P: GamePiece> {
    /// Unique instance ID (different from piece type ID)
    pub instance_id: String,

    /// The piece type ID
    pub piece_id: P::Id,

    /// Owner player ID
    pub owner_id: String,

    /// Controller player ID (may differ from owner)
    pub controller_id: String,

    /// Current state (tapped, face-down, etc.)
    pub state: PieceState,
}

/// State of a piece in play
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct PieceState {
    /// Whether the piece is tapped/exhausted
    pub tapped: bool,

    /// Whether the piece is face-down
    pub face_down: bool,

    /// Counters on this piece
    pub counters: std::collections::HashMap<String, i32>,

    /// Other state flags
    pub flags: std::collections::HashMap<String, bool>,

    /// Attached pieces (auras, equipment, etc.)
    pub attachments: Vec<String>,
}

impl PieceState {
    /// Create a new default state
    pub fn new() -> Self {
        Self::default()
    }

    /// Create a tapped state
    pub fn tapped() -> Self {
        Self {
            tapped: true,
            ..Default::default()
        }
    }

    /// Add a counter
    pub fn with_counter(mut self, counter_type: impl Into<String>, count: i32) -> Self {
        self.counters.insert(counter_type.into(), count);
        self
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_piece_state() {
        let state = PieceState::new()
            .with_counter("+1/+1", 3)
            .with_counter("loyalty", 4);

        assert!(!state.tapped);
        assert_eq!(state.counters.get("+1/+1"), Some(&3));
        assert_eq!(state.counters.get("loyalty"), Some(&4));
    }

    #[test]
    fn test_tapped_state() {
        let state = PieceState::tapped();
        assert!(state.tapped);
    }
}
