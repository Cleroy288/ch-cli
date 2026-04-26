#[derive(Debug, thiserror::Error)]
pub enum MemoryError {
	#[error("{0}")]
	Io(#[from] std::io::Error),

	#[error("serialization failed: {0}")]
	Serialize(String),

	#[error("memory search failed: {0}")]
	Search(String),

	#[error("session not found: {0}")]
	SessionNotFound(String),
}

pub type MemoryResult<T> = Result<T, MemoryError>;
