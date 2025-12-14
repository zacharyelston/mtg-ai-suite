//! RecognitionProvider trait - game-specific piece recognition

use super::piece::GamePiece;
use crate::error::FrameworkResult;
use crate::types::{BoundingBox, ProcessedImage};
use async_trait::async_trait;
use serde::{Deserialize, Serialize};

/// Recognizes game pieces from images.
///
/// This trait provides game-specific recognition capabilities,
/// including single piece recognition and board state analysis.
///
/// # Example
///
/// ```rust,ignore
/// use cardgame_core::traits::RecognitionProvider;
///
/// struct MtgRecognition {
///     ocr_engine: TesseractEngine,
///     fuzzy_matcher: FuzzyMatcher,
///     card_database: Vec<(String, String)>,
/// }
///
/// #[async_trait]
/// impl RecognitionProvider<MtgCard> for MtgRecognition {
///     async fn recognize(
///         &self,
///         image: &ProcessedImage,
///     ) -> FrameworkResult<RecognitionResult<String>> {
///         // Extract text via OCR
///         // Fuzzy match against card database
///         // Return best match
///     }
/// }
/// ```
#[async_trait]
pub trait RecognitionProvider<P: GamePiece>: Send + Sync {
    /// Recognize a single piece from an image.
    async fn recognize(&self, image: &ProcessedImage) -> FrameworkResult<RecognitionResult<P::Id>>;

    /// Recognize multiple pieces from a board/table image.
    async fn recognize_board(
        &self,
        image: &ProcessedImage,
    ) -> FrameworkResult<Vec<BoardPieceRecognition<P::Id>>>;

    /// Get the piece name dictionary for fuzzy matching.
    ///
    /// Returns tuples of (name, piece_id).
    fn piece_dictionary(&self) -> &[(String, P::Id)];

    /// Get recognition configuration.
    fn config(&self) -> &RecognitionConfig;

    /// Update the piece dictionary (e.g., after data sync).
    fn update_dictionary(&mut self, pieces: Vec<(String, P::Id)>);
}

/// Result of recognizing a single piece.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RecognitionResult<Id> {
    /// Recognized piece ID
    pub piece_id: Id,

    /// Confidence score (0.0 to 1.0)
    pub confidence: f64,

    /// Raw OCR text (if applicable)
    pub ocr_text: Option<String>,

    /// Alternative matches
    pub alternatives: Vec<AlternativeMatch<Id>>,

    /// Recognition method used
    pub method: RecognitionMethod,

    /// Processing time in milliseconds
    pub processing_time_ms: u64,
}

/// An alternative match candidate.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AlternativeMatch<Id> {
    /// Piece ID
    pub piece_id: Id,

    /// Confidence score
    pub confidence: f64,

    /// Edit distance from OCR text
    pub edit_distance: Option<usize>,
}

/// Recognition of a piece on a board.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BoardPieceRecognition<Id> {
    /// Recognized piece ID
    pub piece_id: Id,

    /// Confidence score
    pub confidence: f64,

    /// Bounding box in the image
    pub bounding_box: BoundingBox,

    /// Detected state (tapped, face-down, etc.)
    pub detected_state: DetectedPieceState,

    /// Instance ID (for tracking across frames)
    pub instance_id: Option<String>,
}

/// Detected state of a piece on the board.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct DetectedPieceState {
    /// Whether the piece appears tapped/rotated
    pub tapped: Option<bool>,

    /// Whether the piece appears face-down
    pub face_down: Option<bool>,

    /// Detected counters (if visible)
    pub counters: std::collections::HashMap<String, i32>,

    /// Zone the piece appears to be in
    pub zone: Option<String>,
}

/// Method used for recognition.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum RecognitionMethod {
    /// OCR-based text recognition
    Ocr,

    /// Image embedding/similarity
    Embedding,

    /// Combined methods
    Ensemble,

    /// Manual user input
    Manual,

    /// Barcode/QR code
    Barcode,
}

/// Configuration for recognition.
#[derive(Debug, Clone)]
pub struct RecognitionConfig {
    /// Minimum confidence to accept a match
    pub min_confidence: f64,

    /// Maximum alternatives to return
    pub max_alternatives: usize,

    /// Region of piece where name is located (normalized coordinates)
    pub name_region: (f64, f64, f64, f64),

    /// Whether to use OCR
    pub use_ocr: bool,

    /// Whether to use image embeddings
    pub use_embeddings: bool,

    /// OCR language
    pub ocr_language: String,

    /// Maximum edit distance for fuzzy matching
    pub max_edit_distance: usize,
}

impl Default for RecognitionConfig {
    fn default() -> Self {
        Self {
            min_confidence: 0.7,
            max_alternatives: 5,
            name_region: (0.0, 0.0, 1.0, 0.15), // Top 15% of image
            use_ocr: true,
            use_embeddings: false,
            ocr_language: "eng".into(),
            max_edit_distance: 3,
        }
    }
}

impl RecognitionConfig {
    /// Create a new config with default values
    pub fn new() -> Self {
        Self::default()
    }

    /// Set minimum confidence
    pub fn with_min_confidence(mut self, confidence: f64) -> Self {
        self.min_confidence = confidence;
        self
    }

    /// Set name region
    pub fn with_name_region(mut self, x: f64, y: f64, w: f64, h: f64) -> Self {
        self.name_region = (x, y, w, h);
        self
    }

    /// Enable/disable OCR
    pub fn with_ocr(mut self, enabled: bool) -> Self {
        self.use_ocr = enabled;
        self
    }

    /// Enable/disable embeddings
    pub fn with_embeddings(mut self, enabled: bool) -> Self {
        self.use_embeddings = enabled;
        self
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_recognition_config() {
        let config = RecognitionConfig::new()
            .with_min_confidence(0.8)
            .with_name_region(0.0, 0.0, 1.0, 0.2);

        assert_eq!(config.min_confidence, 0.8);
        assert_eq!(config.name_region, (0.0, 0.0, 1.0, 0.2));
    }

    #[test]
    fn test_detected_state() {
        let mut state = DetectedPieceState {
            tapped: Some(true),
            ..Default::default()
        };
        state.counters.insert("+1/+1".into(), 3);

        assert_eq!(state.tapped, Some(true));
        assert_eq!(state.counters.get("+1/+1"), Some(&3));
    }
}
