//! Error types for memory system operations.

/// Errors during memory operations
#[derive(Debug, thiserror::Error)]
pub enum MemoryError {
	#[error("IO error: {0}")]
	IoError(#[from] std::io::Error),

	#[error("Serialization error: {0}")]
	Serialize(String),

	#[error("Search error: {0}")]
	Search(String),

	#[error("Session not found: {0}")]
	SessionNotFound(String),
}

/// Result alias for memory operations
pub type MemoryResult<T> = Result<T, MemoryError>;
