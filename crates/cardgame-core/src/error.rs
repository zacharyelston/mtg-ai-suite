//! Framework error types

use thiserror::Error;

/// Result type alias for framework operations
pub type FrameworkResult<T> = Result<T, FrameworkError>;

/// Core framework errors
#[derive(Debug, Error)]
pub enum FrameworkError {
    // ═══════════════════════════════════════════════════════
    // Recognition Errors
    // ═══════════════════════════════════════════════════════
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

    /// No piece match found
    #[error("No match found for: '{text}'")]
    NoMatch { text: String },

    /// Multiple ambiguous matches
    #[error("Ambiguous match: found {count} candidates for '{text}'")]
    AmbiguousMatch { text: String, count: usize },

    // ═══════════════════════════════════════════════════════
    // Data Source Errors
    // ═══════════════════════════════════════════════════════
    /// Data source not available
    #[error("Data source unavailable: {name}")]
    DataSourceUnavailable { name: String },

    /// Piece not found in data source
    #[error("Piece not found: {id}")]
    PieceNotFound { id: String },

    /// Rate limited by data source
    #[error("Rate limited by {provider}, retry after {retry_after_secs}s")]
    RateLimited {
        provider: String,
        retry_after_secs: u64,
    },

    // ═══════════════════════════════════════════════════════
    // Game State Errors
    // ═══════════════════════════════════════════════════════
    /// Invalid game action
    #[error("Invalid action: {reason}")]
    InvalidAction { reason: String },

    /// Game rule violation
    #[error("Rule violation: {rule}")]
    RuleViolation { rule: String },

    /// Invalid game state
    #[error("Invalid game state: {reason}")]
    InvalidState { reason: String },

    // ═══════════════════════════════════════════════════════
    // Collection Errors
    // ═══════════════════════════════════════════════════════
    /// Collection validation failed
    #[error("Collection validation failed: {reason}")]
    ValidationFailed { reason: String },

    /// Piece not in collection
    #[error("Piece {piece_id} not in collection {collection_id}")]
    PieceNotInCollection {
        piece_id: String,
        collection_id: String,
    },

    // ═══════════════════════════════════════════════════════
    // LLM Errors
    // ═══════════════════════════════════════════════════════
    /// LLM provider error
    #[error("LLM error from {provider}: {message}")]
    LlmError { provider: String, message: String },

    /// LLM context too long
    #[error("Context too long: {tokens} tokens, max is {max_tokens}")]
    ContextTooLong { tokens: usize, max_tokens: usize },

    // ═══════════════════════════════════════════════════════
    // Infrastructure Errors
    // ═══════════════════════════════════════════════════════
    /// Network error
    #[error("Network error: {0}")]
    Network(String),

    /// Database error
    #[error("Database error: {0}")]
    Database(String),

    /// Serialization error
    #[error("Serialization error: {0}")]
    Serialization(#[from] serde_json::Error),

    /// IO error
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),

    /// Configuration error
    #[error("Configuration error: {0}")]
    Config(String),

    // ═══════════════════════════════════════════════════════
    // Game-Specific Errors
    // ═══════════════════════════════════════════════════════
    /// Game-specific error (wrapped)
    #[error("Game error: {0}")]
    GameSpecific(String),
}

impl FrameworkError {
    /// Check if this error is retryable
    pub fn is_retryable(&self) -> bool {
        matches!(
            self,
            FrameworkError::Network(_)
                | FrameworkError::RateLimited { .. }
                | FrameworkError::DataSourceUnavailable { .. }
        )
    }

    /// Get retry delay in seconds (if applicable)
    pub fn retry_delay(&self) -> Option<u64> {
        match self {
            FrameworkError::RateLimited {
                retry_after_secs, ..
            } => Some(*retry_after_secs),
            FrameworkError::Network(_) => Some(5),
            FrameworkError::DataSourceUnavailable { .. } => Some(30),
            _ => None,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_error_display() {
        let err = FrameworkError::ImageTooSmall {
            width: 100,
            height: 100,
            min_width: 640,
            min_height: 480,
        };
        assert!(err.to_string().contains("100x100"));
    }

    #[test]
    fn test_retryable() {
        let network_err = FrameworkError::Network("timeout".into());
        assert!(network_err.is_retryable());

        let match_err = FrameworkError::NoMatch {
            text: "test".into(),
        };
        assert!(!match_err.is_retryable());
    }

    #[test]
    fn test_retry_delay() {
        let rate_limit = FrameworkError::RateLimited {
            provider: "api".into(),
            retry_after_secs: 60,
        };
        assert_eq!(rate_limit.retry_delay(), Some(60));
    }
}
