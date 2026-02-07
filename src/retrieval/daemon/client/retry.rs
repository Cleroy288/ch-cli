//! Retry Configuration
//!
//! Provides retry logic with exponential backoff for daemon communication.

use crate::retrieval::RetrievalError;

/// Configuration for retry behavior with exponential backoff
#[derive(Debug, Clone)]
pub struct RetryConfig {
	/// Maximum number of retry attempts
	pub max_retries: usize,
	/// Initial delay between retries in milliseconds
	pub initial_delay_ms: u64,
	/// Maximum delay between retries in milliseconds
	pub max_delay_ms: u64,
	/// Multiplier for exponential backoff
	pub backoff_multiplier: f32,
}

impl Default for RetryConfig {
	fn default() -> Self {
		Self {
			max_retries: 8,
			initial_delay_ms: 500,
			max_delay_ms: 5000,
			backoff_multiplier: 2.0,
		}
	}
}

/// Check if an error is transient and worth retrying
pub fn is_retryable_error(error: &RetrievalError) -> bool {
	match error {
		RetrievalError::Io(io_err) => {
			// Retry on transient IO errors
			matches!(
				io_err.kind(),
				std::io::ErrorKind::WouldBlock |
				std::io::ErrorKind::TimedOut |
				std::io::ErrorKind::Interrupted |
				std::io::ErrorKind::ConnectionReset |
				std::io::ErrorKind::BrokenPipe
			)
		}
		RetrievalError::DaemonCommunication(msg) => {
			// Retry on known transient daemon communication errors
			msg.contains("temporarily unavailable") ||
			msg.contains("Resource temporarily unavailable") ||
			msg.contains("connection reset") ||
			msg.contains("os error 35")
		}
		_ => false,
	}
}
