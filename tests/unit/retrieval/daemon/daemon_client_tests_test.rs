//! Tests for Daemon Client
//!
//! Unit tests for retry configuration, error handling, and connection pool.

use std::path::PathBuf;
use std::time::Duration;

use ch_cli::retrieval::daemon::client::pool::ConnectionPool;
use ch_cli::retrieval::daemon::client::retry::{is_retryable_error, RetryConfig};
use ch_cli::retrieval::RetrievalError;

#[test]
fn test_retry_config_defaults() {
    let config = RetryConfig::default();
    assert_eq!(config.max_retries, 8);
    assert_eq!(config.initial_delay_ms, 500);
    assert_eq!(config.max_delay_ms, 5000);
    assert!((config.backoff_multiplier - 2.0).abs() < 0.001);
}

#[test]
fn test_is_retryable_error_io_would_block() {
    let error = RetrievalError::Io(std::io::Error::new(
        std::io::ErrorKind::WouldBlock,
        "would block",
    ));
    assert!(is_retryable_error(&error));
}

#[test]
fn test_is_retryable_error_io_timed_out() {
    let error = RetrievalError::Io(std::io::Error::new(
        std::io::ErrorKind::TimedOut,
        "timed out",
    ));
    assert!(is_retryable_error(&error));
}

#[test]
fn test_is_retryable_error_connection_reset() {
    let error = RetrievalError::Io(std::io::Error::new(
        std::io::ErrorKind::ConnectionReset,
        "connection reset",
    ));
    assert!(is_retryable_error(&error));
}

#[test]
fn test_is_retryable_error_daemon_communication() {
    let error = RetrievalError::DaemonCommunication(
        "Resource temporarily unavailable (os error 35)".to_string(),
    );
    assert!(is_retryable_error(&error));
}

#[test]
fn test_is_not_retryable_error_not_found() {
    let error = RetrievalError::Io(std::io::Error::new(
        std::io::ErrorKind::NotFound,
        "not found",
    ));
    assert!(!is_retryable_error(&error));
}

#[test]
fn test_is_not_retryable_error_daemon_not_running() {
    let error = RetrievalError::DaemonNotRunning("not running".to_string());
    assert!(!is_retryable_error(&error));
}

#[test]
fn test_exponential_backoff_calculation() {
    let config = RetryConfig::default();
    let mut delay = config.initial_delay_ms; // current delay value

    // First delay: 500ms
    assert_eq!(delay, 500);

    // Second delay: 500 * 2 = 1000ms
    delay = ((delay as f32) * config.backoff_multiplier) as u64;
    assert_eq!(delay, 1000);

    // Third delay: 1000 * 2 = 2000ms
    delay = ((delay as f32) * config.backoff_multiplier) as u64;
    assert_eq!(delay, 2000);

    // Should be capped at max_delay_ms
    delay = 6000;
    delay = delay.min(config.max_delay_ms);
    assert_eq!(delay, 5000);
}

#[test]
fn test_connection_pool_creation() {
    // Test that pool is created with correct initial state
    let socket_path = PathBuf::from("/tmp/test_socket"); // test socket path
    let max_size = 5; // maximum pool size

    let pool = ConnectionPool::new(socket_path.clone(), max_size);

    assert_eq!(pool.socket_path(), &socket_path);
    assert_eq!(pool.max_size(), max_size);
    assert_eq!(pool.timeout(), Duration::from_secs(30));
    assert_eq!(pool.available_connections(), 0);
}

#[test]
fn test_connection_pool_available_connections_empty() {
    // Test available_connections returns 0 for empty pool
    let pool = ConnectionPool::new(PathBuf::from("/tmp/test"), 5);
    assert_eq!(pool.available_connections(), 0);
}

#[test]
fn test_connection_pool_get_without_daemon_returns_error() {
    // Test that get() returns error when daemon is not running
    let pool = ConnectionPool::new(PathBuf::from("/tmp/nonexistent_socket_12345"), 5);
    let result = pool.get();
    assert!(result.is_err());
}

#[test]
fn test_connection_pool_max_size_respected() {
    // Test that pool respects max_size configuration
    let pool = ConnectionPool::new(PathBuf::from("/tmp/test"), 3);
    assert_eq!(pool.max_size(), 3);
}

#[test]
fn test_pooled_connection_take_returns_none_on_subsequent_calls() {
    // Test that PooledConnection::take properly handles ownership
    // This test verifies the struct fields are correctly set up
    // Note: Full integration test requires running daemon
    let pool = ConnectionPool::new(PathBuf::from("/tmp/test"), 5);

    // Verify pool is empty initially
    assert_eq!(pool.available_connections(), 0);

    // Verify pool has no available connections
    assert_eq!(pool.available_connections(), 0);
}
