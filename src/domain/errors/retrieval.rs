//! Error types for the retrieval module.

/// Errors that can occur in the retrieval module
#[derive(Debug, thiserror::Error)]
pub enum RetrievalError {
	#[error("Daemon not running: {0}")]
	DaemonNotRunning(String),

	#[error("Daemon communication error: {0}")]
	DaemonCommunication(String),

	#[error("Model loading error: {0}")]
	ModelLoading(String),

	#[error("Embedding error: {0}")]
	Embedding(String),

	#[error("IO error: {0}")]
	IoError(#[from] std::io::Error),
}

/// Result type for retrieval operations
pub type RetrievalResult<T> =
	Result<T, RetrievalError>;
