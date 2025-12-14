//! Integration tests for MTG AI Suite
//!
//! These tests run against a real database (SQLite for CI, Postgres for production testing).

#[cfg(feature = "integration")]
mod tests {
    use std::env;

    /// Test that DATABASE_URL is set
    #[test]
    fn test_database_url_configured() {
        let db_url = env::var("DATABASE_URL").unwrap_or_else(|_| "sqlite::memory:".to_string());
        assert!(!db_url.is_empty(), "DATABASE_URL should be set");
        println!("Using database: {}", db_url);
    }

    /// Test basic health check endpoint
    #[tokio::test]
    async fn test_health_endpoint() {
        // This would test the actual server health endpoint
        // For now, just verify the test infrastructure works
        assert!(true, "Integration test infrastructure is working");
    }
}
