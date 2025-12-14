//! Fuzzy string matching for card names
//!
//! This module provides fast fuzzy matching using edit distance algorithms.
//! It's designed to handle OCR errors and typos in card name recognition.

use serde::{Deserialize, Serialize};
use strsim::{jaro_winkler, levenshtein};
use tracing::instrument;

/// Result of a fuzzy match operation
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MatchResult {
    /// The matched card name
    pub name: String,

    /// Scryfall ID of the matched card
    pub scryfall_id: String,

    /// Confidence score (0.0 to 1.0)
    pub confidence: f64,

    /// Edit distance from query
    pub edit_distance: usize,
}

/// Configuration for fuzzy matching
#[derive(Debug, Clone)]
pub struct FuzzyConfig {
    /// Maximum edit distance to consider a match
    pub max_edit_distance: usize,

    /// Minimum confidence threshold
    pub min_confidence: f64,

    /// Maximum number of results to return
    pub max_results: usize,

    /// Whether to use case-insensitive matching
    pub case_insensitive: bool,
}

impl Default for FuzzyConfig {
    fn default() -> Self {
        Self {
            max_edit_distance: 3,
            min_confidence: 0.7,
            max_results: 5,
            case_insensitive: true,
        }
    }
}

/// Fuzzy matcher for card names
#[derive(Debug)]
pub struct FuzzyMatcher {
    /// Card names and their Scryfall IDs
    cards: Vec<(String, String)>,

    /// Normalized names for matching (lowercase)
    normalized: Vec<String>,

    /// Configuration
    config: FuzzyConfig,
}

impl FuzzyMatcher {
    /// Create a new fuzzy matcher with card names
    ///
    /// # Arguments
    /// * `cards` - Vector of (name, scryfall_id) tuples
    ///
    /// # Example
    /// ```
    /// use mtg_core::FuzzyMatcher;
    ///
    /// let cards = vec![
    ///     ("Lightning Bolt".to_string(), "abc123".to_string()),
    ///     ("Lightning Helix".to_string(), "def456".to_string()),
    /// ];
    /// let matcher = FuzzyMatcher::new(cards);
    /// ```
    pub fn new(cards: Vec<(String, String)>) -> Self {
        Self::with_config(cards, FuzzyConfig::default())
    }

    /// Create a new fuzzy matcher with custom configuration
    pub fn with_config(cards: Vec<(String, String)>, config: FuzzyConfig) -> Self {
        let normalized = cards.iter().map(|(name, _)| name.to_lowercase()).collect();

        Self {
            cards,
            normalized,
            config,
        }
    }

    /// Find the best match for a query string
    ///
    /// # Arguments
    /// * `query` - The string to match (e.g., OCR result)
    ///
    /// # Returns
    /// The best matching card, or None if no match meets the threshold
    #[instrument(skip(self))]
    pub fn find(&self, query: &str) -> Option<MatchResult> {
        self.find_all(query).into_iter().next()
    }

    /// Find all matches for a query string, sorted by confidence
    ///
    /// # Arguments
    /// * `query` - The string to match
    ///
    /// # Returns
    /// Vector of matches sorted by confidence (highest first)
    #[instrument(skip(self))]
    pub fn find_all(&self, query: &str) -> Vec<MatchResult> {
        if self.cards.is_empty() {
            return vec![];
        }

        let query_normalized = if self.config.case_insensitive {
            query.to_lowercase()
        } else {
            query.to_string()
        };

        let mut results: Vec<MatchResult> = self
            .cards
            .iter()
            .zip(self.normalized.iter())
            .filter_map(|((name, scryfall_id), normalized)| {
                let target = if self.config.case_insensitive {
                    normalized
                } else {
                    name
                };

                let edit_distance = levenshtein(&query_normalized, target);

                // Skip if edit distance too high
                if edit_distance > self.config.max_edit_distance {
                    return None;
                }

                // Calculate confidence using Jaro-Winkler similarity
                let confidence = jaro_winkler(&query_normalized, target);

                // Skip if confidence too low
                if confidence < self.config.min_confidence {
                    return None;
                }

                Some(MatchResult {
                    name: name.clone(),
                    scryfall_id: scryfall_id.clone(),
                    confidence,
                    edit_distance,
                })
            })
            .collect();

        // Sort by confidence (highest first), then by edit distance (lowest first)
        results.sort_by(|a, b| {
            b.confidence
                .partial_cmp(&a.confidence)
                .unwrap_or(std::cmp::Ordering::Equal)
                .then_with(|| a.edit_distance.cmp(&b.edit_distance))
        });

        results.truncate(self.config.max_results);
        results
    }

    /// Check if the matcher has any cards loaded
    pub fn is_empty(&self) -> bool {
        self.cards.is_empty()
    }

    /// Get the number of cards in the matcher
    pub fn len(&self) -> usize {
        self.cards.len()
    }

    /// Find exact match (case-insensitive)
    pub fn find_exact(&self, query: &str) -> Option<MatchResult> {
        let query_lower = query.to_lowercase();

        self.cards
            .iter()
            .zip(self.normalized.iter())
            .find(|(_, normalized)| *normalized == &query_lower)
            .map(|((name, scryfall_id), _)| MatchResult {
                name: name.clone(),
                scryfall_id: scryfall_id.clone(),
                confidence: 1.0,
                edit_distance: 0,
            })
    }
}

/// Normalize a card name for matching
///
/// Removes special characters and normalizes whitespace
pub fn normalize_card_name(name: &str) -> String {
    name.chars()
        .filter(|c| c.is_alphanumeric() || c.is_whitespace())
        .collect::<String>()
        .split_whitespace()
        .collect::<Vec<_>>()
        .join(" ")
        .to_lowercase()
}

/// Common OCR error corrections
pub fn correct_ocr_errors(text: &str) -> String {
    text.replace('0', "O")
        .replace(['1', '|'], "l")
        .replace("rn", "m")
        .replace("vv", "w")
}

#[cfg(test)]
mod tests {
    use super::*;

    fn test_cards() -> Vec<(String, String)> {
        vec![
            ("Lightning Bolt".to_string(), "bolt-001".to_string()),
            ("Lightning Helix".to_string(), "helix-001".to_string()),
            ("Counterspell".to_string(), "counter-001".to_string()),
            ("Counter".to_string(), "counter-002".to_string()),
            (
                "Monastery Swiftspear".to_string(),
                "swiftspear-001".to_string(),
            ),
        ]
    }

    #[test]
    fn test_exact_match() {
        let matcher = FuzzyMatcher::new(test_cards());
        let result = matcher.find("Lightning Bolt").unwrap();

        assert_eq!(result.name, "Lightning Bolt");
        assert_eq!(result.confidence, 1.0);
        assert_eq!(result.edit_distance, 0);
    }

    #[test]
    fn test_case_insensitive() {
        let matcher = FuzzyMatcher::new(test_cards());
        let result = matcher.find("lightning bolt").unwrap();

        assert_eq!(result.name, "Lightning Bolt");
    }

    #[test]
    fn test_typo_correction() {
        let matcher = FuzzyMatcher::new(test_cards());
        let result = matcher.find("Lightening Bolt").unwrap(); // Common typo

        assert_eq!(result.name, "Lightning Bolt");
        assert!(result.confidence > 0.8);
    }

    #[test]
    fn test_ocr_error() {
        let matcher = FuzzyMatcher::new(test_cards());
        let result = matcher.find("Lightn1ng Bolt").unwrap(); // OCR error: 1 instead of i

        assert_eq!(result.name, "Lightning Bolt");
    }

    #[test]
    fn test_no_match() {
        let matcher = FuzzyMatcher::new(test_cards());
        let result = matcher.find("Completely Different Card");

        assert!(result.is_none());
    }

    #[test]
    fn test_find_all() {
        // Use a config with higher max_edit_distance for partial matching
        let config = FuzzyConfig {
            max_edit_distance: 10,
            min_confidence: 0.5,
            ..Default::default()
        };
        let matcher = FuzzyMatcher::with_config(test_cards(), config);
        let results = matcher.find_all("Lightning");

        assert!(results.len() >= 2);
        assert!(results.iter().any(|r| r.name == "Lightning Bolt"));
        assert!(results.iter().any(|r| r.name == "Lightning Helix"));
    }

    #[test]
    fn test_find_exact() {
        let matcher = FuzzyMatcher::new(test_cards());

        let result = matcher.find_exact("Counterspell").unwrap();
        assert_eq!(result.name, "Counterspell");

        let no_result = matcher.find_exact("Countersp");
        assert!(no_result.is_none());
    }

    #[test]
    fn test_normalize_card_name() {
        assert_eq!(normalize_card_name("Lightning Bolt"), "lightning bolt");
        assert_eq!(
            normalize_card_name("  Multiple   Spaces  "),
            "multiple spaces"
        );
        assert_eq!(normalize_card_name("Card's Name"), "cards name");
    }

    #[test]
    fn test_correct_ocr_errors() {
        assert_eq!(correct_ocr_errors("B0lt"), "BOlt");
        assert_eq!(correct_ocr_errors("He1ix"), "Helix");
    }

    #[test]
    fn test_empty_matcher() {
        let matcher = FuzzyMatcher::new(vec![]);
        assert!(matcher.is_empty());
        assert!(matcher.find("anything").is_none());
    }
}
