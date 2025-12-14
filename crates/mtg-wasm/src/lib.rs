//! MTG AI Suite WebAssembly Module
//!
//! This module provides client-side card recognition capabilities
//! that run directly in the browser via WebAssembly.

use mtg_core::fuzzy::{FuzzyMatcher, MatchResult};
use serde::{Deserialize, Serialize};
use wasm_bindgen::prelude::*;

// Initialize panic hook for better error messages
#[wasm_bindgen(start)]
pub fn init() {
    console_error_panic_hook::set_once();
}

/// Card matcher for client-side fuzzy matching
#[wasm_bindgen]
pub struct CardMatcher {
    matcher: FuzzyMatcher,
}

#[wasm_bindgen]
impl CardMatcher {
    /// Create a new CardMatcher from a JSON array of card names
    ///
    /// Expected format: [{"name": "Lightning Bolt", "scryfall_id": "abc123"}, ...]
    #[wasm_bindgen(constructor)]
    pub fn new(cards_json: &str) -> Result<CardMatcher, JsValue> {
        let cards: Vec<CardEntry> = serde_json::from_str(cards_json)
            .map_err(|e| JsValue::from_str(&format!("Failed to parse cards: {}", e)))?;

        let card_tuples: Vec<(String, String)> =
            cards.into_iter().map(|c| (c.name, c.scryfall_id)).collect();

        Ok(CardMatcher {
            matcher: FuzzyMatcher::new(card_tuples),
        })
    }

    /// Find the best match for a query string
    ///
    /// Returns JSON: {"name": "...", "scryfall_id": "...", "confidence": 0.95}
    /// or null if no match found
    #[wasm_bindgen]
    pub fn find(&self, query: &str) -> Option<String> {
        self.matcher
            .find(query)
            .map(|result| serde_json::to_string(&WasmMatchResult::from(result)).unwrap_or_default())
    }

    /// Find all matches for a query string
    ///
    /// Returns JSON array of matches
    #[wasm_bindgen]
    pub fn find_all(&self, query: &str) -> String {
        let results: Vec<WasmMatchResult> = self
            .matcher
            .find_all(query)
            .into_iter()
            .map(WasmMatchResult::from)
            .collect();

        serde_json::to_string(&results).unwrap_or_else(|_| "[]".to_string())
    }

    /// Check if a card name exists (exact match)
    #[wasm_bindgen]
    pub fn exists(&self, name: &str) -> bool {
        self.matcher.find_exact(name).is_some()
    }

    /// Get the number of cards in the matcher
    #[wasm_bindgen]
    pub fn count(&self) -> usize {
        self.matcher.len()
    }
}

/// Card entry for JSON parsing
#[derive(Debug, Deserialize)]
struct CardEntry {
    name: String,
    scryfall_id: String,
}

/// Match result for WASM export
#[derive(Debug, Serialize)]
struct WasmMatchResult {
    name: String,
    scryfall_id: String,
    confidence: f64,
    edit_distance: usize,
}

impl From<MatchResult> for WasmMatchResult {
    fn from(result: MatchResult) -> Self {
        Self {
            name: result.name,
            scryfall_id: result.scryfall_id,
            confidence: result.confidence,
            edit_distance: result.edit_distance,
        }
    }
}

/// Process an image and extract text region
///
/// This is a placeholder - actual implementation would use image processing
#[wasm_bindgen]
pub fn preprocess_image(image_data: &[u8]) -> Result<Vec<u8>, JsValue> {
    // TODO: Implement actual image preprocessing
    // For now, just return the input
    Ok(image_data.to_vec())
}

/// Calculate image quality score
#[wasm_bindgen]
pub fn calculate_quality(width: u32, height: u32, _image_data: &[u8]) -> f64 {
    // Simple quality heuristic based on resolution
    let min_dimension = width.min(height);
    (min_dimension as f64 / 1000.0).min(1.0)
}

/// Normalize text for matching (lowercase, remove special chars)
#[wasm_bindgen]
pub fn normalize_text(text: &str) -> String {
    mtg_core::fuzzy::normalize_card_name(text)
}

/// Correct common OCR errors
#[wasm_bindgen]
pub fn correct_ocr(text: &str) -> String {
    mtg_core::fuzzy::correct_ocr_errors(text)
}

#[cfg(test)]
mod tests {
    use super::*;
    use wasm_bindgen_test::*;

    #[wasm_bindgen_test]
    fn test_card_matcher() {
        let cards_json = r#"[
            {"name": "Lightning Bolt", "scryfall_id": "bolt-001"},
            {"name": "Counterspell", "scryfall_id": "counter-001"}
        ]"#;

        let matcher = CardMatcher::new(cards_json).unwrap();
        assert_eq!(matcher.count(), 2);

        let result = matcher.find("Lightning Bolt");
        assert!(result.is_some());

        let result_json = result.unwrap();
        assert!(result_json.contains("Lightning Bolt"));
    }

    #[wasm_bindgen_test]
    fn test_normalize_text() {
        assert_eq!(normalize_text("Lightning Bolt"), "lightning bolt");
        assert_eq!(normalize_text("  Multiple   Spaces  "), "multiple spaces");
    }

    #[wasm_bindgen_test]
    fn test_correct_ocr() {
        assert_eq!(correct_ocr("B0lt"), "BOlt");
    }
}
