//! Error types for MTG Core

use thiserror::Error;

/// Result type alias using the crate's Error type
pub type Result<T> = std::result::Result<T, Error>;

/// Core error types for the MTG AI Suite
#[derive(Debug, Error)]
pub enum Error {
    /// Image is too small for reliable recognition
    #[error("Image too small: {width}x{height}, minimum is {min_width}x{min_height}")]
    ImageTooSmall {
        width: u32,
        height: u32,
        min_width: u32,
        min_height: u32,
    },

    /// Image format is not supported
    #[error("Unsupported image format: {format}")]
    UnsupportedImageFormat { format: String },

    /// Image decoding failed
    #[error("Failed to decode image: {reason}")]
    ImageDecodeFailed { reason: String },

    /// OCR processing failed
    #[error("OCR processing failed: {reason}")]
    OcrFailed { reason: String },

    /// No card match found
    #[error("No card match found for text: '{text}'")]
    NoMatch { text: String },

    /// Multiple ambiguous matches
    #[error("Ambiguous match: found {count} candidates for '{text}'")]
    AmbiguousMatch { text: String, count: usize },

    /// Card dictionary not loaded
    #[error("Card dictionary not loaded")]
    DictionaryNotLoaded,

    /// Invalid card data
    #[error("Invalid card data: {reason}")]
    InvalidCardData { reason: String },

    /// Serialization error
    #[error("Serialization error: {0}")]
    Serialization(#[from] serde_json::Error),

    /// IO error
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_error_display() {
        let err = Error::ImageTooSmall {
            width: 100,
            height: 100,
            min_width: 640,
            min_height: 480,
        };
        assert!(err.to_string().contains("100x100"));
        assert!(err.to_string().contains("640x480"));
    }

    #[test]
    fn test_no_match_error() {
        let err = Error::NoMatch {
            text: "Lightening Bolt".into(),
        };
        assert!(err.to_string().contains("Lightening Bolt"));
    }
}
