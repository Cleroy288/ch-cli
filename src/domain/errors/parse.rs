//! Error types for parsing operations.

/// Errors during Tree-sitter parsing operations.
/// Covers IO, language setup, parse failures, and
/// query compilation errors.
#[derive(Debug, thiserror::Error)]
pub enum ParseError {
	#[error("Failed to read file: {0}")]
	IoError(#[from] std::io::Error),

	#[error("Failed to set parser language: {0}")]
	LanguageError(String),

	#[error("Failed to parse file: {0}")]
	ParseFailed(String),

	#[error("Failed to compile query: {0}")]
	QueryError(String),
}

/// Result type alias for parser operations
pub type ParseResult<T> =
	std::result::Result<T, ParseError>;
