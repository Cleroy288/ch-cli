//! Error types for CLI command execution.

use super::index::IndexError;
use super::retrieval::RetrievalError;
use super::search::SearchError;

/// Result type for CLI commands
pub type CommandResult = Result<(), CommandError>;

/// Errors that can occur during command execution
#[derive(Debug, thiserror::Error)]
pub enum CommandError {
	#[error("Index error: {0}")]
	IndexError(String),

	#[error("Index manager error: {0}")]
	IndexManagerError(#[from] IndexError),

	#[error("Search error: {0}")]
	SearchError(#[from] SearchError),

	#[error("IO error: {0}")]
	IoError(#[from] std::io::Error),

	#[error("Symbol not found: {0}")]
	SymbolNotFound(String),

	#[error("Invalid symbol kind: {0}")]
	InvalidKind(String),

	#[error("Retrieval error: {0}")]
	RetrievalError(#[from] RetrievalError),
}
