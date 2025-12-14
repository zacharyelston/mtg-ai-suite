//! PlayAdvisor trait - AI-powered play suggestions

use super::state::GameState;
use crate::error::FrameworkResult;
use async_trait::async_trait;
use serde::{Deserialize, Serialize};

/// AI-powered play advisor.
///
/// This trait provides intelligent suggestions for gameplay,
/// leveraging LLMs or other AI systems to analyze game state
/// and recommend optimal actions.
///
/// # Example
///
/// ```rust,ignore
/// use cardgame_core::traits::PlayAdvisor;
///
/// struct MtgAdvisor {
///     llm_client: LlmClient,
///     rules_engine: MtgRules,
/// }
///
/// #[async_trait]
/// impl PlayAdvisor<MtgGameState> for MtgAdvisor {
///     async fn suggest_actions(
///         &self,
///         state: &MtgGameState,
///         context: &AdvisorContext,
///     ) -> FrameworkResult<Vec<ActionSuggestion<MtgAction>>> {
///         // Build prompt from state
///         // Query LLM
///         // Parse and validate suggestions
///     }
/// }
/// ```
#[async_trait]
pub trait PlayAdvisor<S: GameState>: Send + Sync {
    /// Suggest best actions from current state.
    ///
    /// Returns a list of suggested actions ranked by priority/confidence.
    async fn suggest_actions(
        &self,
        state: &S,
        context: &AdvisorContext,
    ) -> FrameworkResult<Vec<ActionSuggestion<S::Action>>>;

    /// Evaluate a potential action.
    ///
    /// Returns an evaluation of how good the action is.
    async fn evaluate_action(
        &self,
        state: &S,
        action: &S::Action,
    ) -> FrameworkResult<ActionEvaluation>;

    /// Explain reasoning for a suggestion.
    ///
    /// Returns a human-readable explanation of why an action was suggested.
    async fn explain(
        &self,
        state: &S,
        suggestion: &ActionSuggestion<S::Action>,
    ) -> FrameworkResult<String>;

    /// Analyze the current board state.
    ///
    /// Returns a general analysis of the game situation.
    async fn analyze_state(&self, state: &S) -> FrameworkResult<StateAnalysis>;

    /// Get mulligan advice (for card games).
    ///
    /// Default implementation returns no advice.
    async fn mulligan_advice(
        &self,
        _state: &S,
        _hand: &[<S::Piece as super::piece::GamePiece>::Id],
    ) -> FrameworkResult<MulliganAdvice> {
        Ok(MulliganAdvice::default())
    }
}

/// Context for advisor queries.
#[derive(Debug, Clone, Default)]
pub struct AdvisorContext {
    /// Player's skill level (for adjusting explanations)
    pub skill_level: SkillLevel,

    /// Maximum suggestions to return
    pub max_suggestions: usize,

    /// Whether to include detailed reasoning
    pub include_reasoning: bool,

    /// Game-specific context
    pub game_context: std::collections::HashMap<String, String>,

    /// Opponent modeling (if available)
    pub opponent_info: Option<OpponentInfo>,
}

impl AdvisorContext {
    /// Create a new context with defaults
    pub fn new() -> Self {
        Self {
            max_suggestions: 3,
            include_reasoning: true,
            ..Default::default()
        }
    }

    /// Set skill level
    pub fn with_skill_level(mut self, level: SkillLevel) -> Self {
        self.skill_level = level;
        self
    }

    /// Set max suggestions
    pub fn with_max_suggestions(mut self, max: usize) -> Self {
        self.max_suggestions = max;
        self
    }
}

/// Player skill level.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum SkillLevel {
    Beginner,
    #[default]
    Intermediate,
    Advanced,
    Expert,
}

/// Information about opponent (for modeling).
#[derive(Debug, Clone, Default)]
pub struct OpponentInfo {
    /// Known deck archetype
    pub archetype: Option<String>,

    /// Play style observations
    pub play_style: Option<String>,

    /// Cards seen this game
    pub cards_seen: Vec<String>,
}

/// A suggested action with metadata.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ActionSuggestion<A> {
    /// The suggested action
    pub action: A,

    /// Confidence score (0.0 to 1.0)
    pub confidence: f64,

    /// Priority ranking (1 = highest)
    pub priority: u32,

    /// Brief reasoning
    pub reasoning: String,

    /// Expected outcome
    pub expected_outcome: Option<String>,

    /// Risk level
    pub risk_level: RiskLevel,
}

impl<A> ActionSuggestion<A> {
    /// Create a new suggestion
    pub fn new(action: A, confidence: f64, reasoning: impl Into<String>) -> Self {
        Self {
            action,
            confidence,
            priority: 1,
            reasoning: reasoning.into(),
            expected_outcome: None,
            risk_level: RiskLevel::Medium,
        }
    }

    /// Set priority
    pub fn with_priority(mut self, priority: u32) -> Self {
        self.priority = priority;
        self
    }

    /// Set risk level
    pub fn with_risk(mut self, risk: RiskLevel) -> Self {
        self.risk_level = risk;
        self
    }
}

/// Risk level of an action.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum RiskLevel {
    Low,
    #[default]
    Medium,
    High,
    Critical,
}

/// Evaluation of an action.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ActionEvaluation {
    /// Overall score (-1.0 to 1.0, positive is good)
    pub score: f64,

    /// Pros of this action
    pub pros: Vec<String>,

    /// Cons of this action
    pub cons: Vec<String>,

    /// Alternative actions to consider
    pub alternatives: Vec<String>,
}

/// Analysis of game state.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StateAnalysis {
    /// Who is currently ahead
    pub advantage: Advantage,

    /// Key threats to address
    pub threats: Vec<String>,

    /// Key opportunities
    pub opportunities: Vec<String>,

    /// General assessment
    pub assessment: String,

    /// Win probability estimate (0.0 to 1.0)
    pub win_probability: Option<f64>,
}

/// Who has the advantage.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum Advantage {
    You,
    Opponent,
    Even,
    Unclear,
}

/// Mulligan advice.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct MulliganAdvice {
    /// Whether to keep the hand
    pub keep: bool,

    /// Confidence in the decision
    pub confidence: f64,

    /// Reasoning
    pub reasoning: String,

    /// What the hand is missing (if mulliganing)
    pub missing: Vec<String>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_advisor_context() {
        let context = AdvisorContext::new()
            .with_skill_level(SkillLevel::Advanced)
            .with_max_suggestions(5);

        assert_eq!(context.skill_level, SkillLevel::Advanced);
        assert_eq!(context.max_suggestions, 5);
    }

    #[test]
    fn test_action_suggestion() {
        let suggestion = ActionSuggestion::new("attack", 0.85, "Good attack opportunity")
            .with_priority(1)
            .with_risk(RiskLevel::Low);

        assert_eq!(suggestion.confidence, 0.85);
        assert_eq!(suggestion.priority, 1);
        assert_eq!(suggestion.risk_level, RiskLevel::Low);
    }
}
