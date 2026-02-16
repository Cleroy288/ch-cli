//! Error types for documentation generation.

/// Errors during doc generation operations
#[derive(Debug, thiserror::Error)]
pub enum DocGenError {
	#[error("Daemon not available: {0}")]
	DaemonUnavailable(String),

	#[error("Generation failed: {0}")]
	GenerationFailed(String),

	#[error("Symbol not found: {0}")]
	SymbolNotFound(String),

	#[error("IO error: {0}")]
	IoError(#[from] std::io::Error),
}
