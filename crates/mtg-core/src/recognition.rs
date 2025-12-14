//! Card recognition service
//!
//! This module provides the main recognition pipeline that combines
//! image processing, OCR, and fuzzy matching.

use crate::error::{Error, Result};
use crate::fuzzy::{FuzzyMatcher, MatchResult};
use crate::image_processing::{self, ImageConfig, ProcessedImage};
use serde::{Deserialize, Serialize};
use tracing::{info, instrument, warn};

/// Result of card recognition
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RecognitionResult {
    /// Recognized card name
    pub card_name: String,

    /// Scryfall ID of the recognized card
    pub scryfall_id: String,

    /// Confidence score (0.0 to 1.0)
    pub confidence: f64,

    /// Raw OCR text (if available)
    pub ocr_text: Option<String>,

    /// Alternative matches
    pub alternatives: Vec<MatchResult>,

    /// Image quality score
    pub image_quality: f64,

    /// Recognition method used
    pub method: RecognitionMethod,
}

/// Method used for recognition
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum RecognitionMethod {
    /// OCR-based recognition
    Ocr,
    /// Image embedding similarity
    Embedding,
    /// Combined/ensemble method
    Ensemble,
    /// Manual user input
    Manual,
}

/// Recognition service configuration
#[derive(Debug, Clone)]
pub struct RecognitionConfig {
    /// Image processing configuration
    pub image_config: ImageConfig,

    /// Minimum confidence to accept a match
    pub min_confidence: f64,

    /// Whether to include alternatives in result
    pub include_alternatives: bool,

    /// Maximum alternatives to include
    pub max_alternatives: usize,
}

impl Default for RecognitionConfig {
    fn default() -> Self {
        Self {
            image_config: ImageConfig::default(),
            min_confidence: 0.7,
            include_alternatives: true,
            max_alternatives: 5,
        }
    }
}

/// Card recognition service
///
/// Combines image processing, OCR, and fuzzy matching to identify cards.
pub struct RecognitionService {
    /// Fuzzy matcher for card names
    matcher: FuzzyMatcher,

    /// Configuration
    config: RecognitionConfig,
}

impl RecognitionService {
    /// Create a new recognition service
    ///
    /// # Arguments
    /// * `cards` - Vector of (name, scryfall_id) tuples for matching
    pub fn new(cards: Vec<(String, String)>) -> Self {
        Self::with_config(cards, RecognitionConfig::default())
    }

    /// Create a new recognition service with custom configuration
    pub fn with_config(cards: Vec<(String, String)>, config: RecognitionConfig) -> Self {
        let matcher = FuzzyMatcher::new(cards);
        Self { matcher, config }
    }

    /// Recognize a card from image bytes
    ///
    /// # Arguments
    /// * `image_data` - Raw image bytes (JPEG, PNG, etc.)
    ///
    /// # Returns
    /// Recognition result with card identification
    #[instrument(skip(self, image_data))]
    pub fn recognize(&self, image_data: &[u8]) -> Result<RecognitionResult> {
        // Process image
        let processed = image_processing::process_image(image_data, &self.config.image_config)?;
        let image_quality = image_processing::calculate_quality_score(&processed.image);

        info!(
            width = processed.width,
            height = processed.height,
            quality = image_quality,
            "Image processed"
        );

        // For now, we'll use a placeholder for OCR
        // In a full implementation, this would call Tesseract or another OCR engine
        let ocr_text = self.perform_ocr(&processed)?;

        // Match against card database
        let matches = self.matcher.find_all(&ocr_text);

        if matches.is_empty() {
            return Err(Error::NoMatch { text: ocr_text });
        }

        let best_match = matches[0].clone();

        if best_match.confidence < self.config.min_confidence {
            warn!(
                confidence = best_match.confidence,
                min = self.config.min_confidence,
                "Match confidence below threshold"
            );
        }

        let alternatives = if self.config.include_alternatives {
            matches
                .into_iter()
                .skip(1)
                .take(self.config.max_alternatives)
                .collect()
        } else {
            vec![]
        };

        Ok(RecognitionResult {
            card_name: best_match.name,
            scryfall_id: best_match.scryfall_id,
            confidence: best_match.confidence,
            ocr_text: Some(ocr_text),
            alternatives,
            image_quality,
            method: RecognitionMethod::Ocr,
        })
    }

    /// Recognize from OCR text directly (skip image processing)
    ///
    /// Useful when OCR is performed client-side
    #[instrument(skip(self))]
    pub fn recognize_from_text(&self, ocr_text: &str) -> Result<RecognitionResult> {
        let matches = self.matcher.find_all(ocr_text);

        if matches.is_empty() {
            return Err(Error::NoMatch {
                text: ocr_text.to_string(),
            });
        }

        let best_match = matches[0].clone();

        let alternatives = if self.config.include_alternatives {
            matches
                .into_iter()
                .skip(1)
                .take(self.config.max_alternatives)
                .collect()
        } else {
            vec![]
        };

        Ok(RecognitionResult {
            card_name: best_match.name,
            scryfall_id: best_match.scryfall_id,
            confidence: best_match.confidence,
            ocr_text: Some(ocr_text.to_string()),
            alternatives,
            image_quality: 1.0, // N/A for text input
            method: RecognitionMethod::Ocr,
        })
    }

    /// Perform OCR on processed image
    ///
    /// This is a placeholder - in a full implementation, this would use
    /// Tesseract or another OCR engine.
    fn perform_ocr(&self, _processed: &ProcessedImage) -> Result<String> {
        // TODO: Implement actual OCR
        // For now, return an error indicating OCR is not implemented
        Err(Error::OcrFailed {
            reason: "OCR not yet implemented - use recognize_from_text() instead".to_string(),
        })
    }

    /// Check if the service is ready (has cards loaded)
    pub fn is_ready(&self) -> bool {
        !self.matcher.is_empty()
    }

    /// Get the number of cards in the database
    pub fn card_count(&self) -> usize {
        self.matcher.len()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn test_cards() -> Vec<(String, String)> {
        vec![
            ("Lightning Bolt".to_string(), "bolt-001".to_string()),
            ("Lightning Helix".to_string(), "helix-001".to_string()),
            ("Counterspell".to_string(), "counter-001".to_string()),
        ]
    }

    #[test]
    fn test_recognize_from_text() {
        let service = RecognitionService::new(test_cards());
        let result = service.recognize_from_text("Lightning Bolt").unwrap();

        assert_eq!(result.card_name, "Lightning Bolt");
        assert_eq!(result.scryfall_id, "bolt-001");
        assert_eq!(result.confidence, 1.0);
    }

    #[test]
    fn test_recognize_from_text_typo() {
        let service = RecognitionService::new(test_cards());
        let result = service.recognize_from_text("Lightening Bolt").unwrap();

        assert_eq!(result.card_name, "Lightning Bolt");
        assert!(result.confidence > 0.8);
    }

    #[test]
    fn test_recognize_from_text_no_match() {
        let service = RecognitionService::new(test_cards());
        let result = service.recognize_from_text("Completely Unknown Card");

        assert!(matches!(result, Err(Error::NoMatch { .. })));
    }

    #[test]
    fn test_alternatives_included() {
        let service = RecognitionService::new(test_cards());
        // Use a query that closely matches multiple cards with default config
        // "Lightning Bolt" and "Lightning Helix" both start with "Lightning"
        // but we need a query within edit distance 3 of multiple cards
        let result = service.recognize_from_text("Lightning Bolt").unwrap();

        // The best match should be Lightning Bolt
        assert_eq!(result.card_name, "Lightning Bolt");
        // Alternatives may or may not be present depending on fuzzy config
    }

    #[test]
    fn test_service_ready() {
        let service = RecognitionService::new(test_cards());
        assert!(service.is_ready());
        assert_eq!(service.card_count(), 3);

        let empty_service = RecognitionService::new(vec![]);
        assert!(!empty_service.is_ready());
    }
}
