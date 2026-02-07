//! Error types for search operations.

use std::path::PathBuf;

/// Error type for search operations
#[derive(Debug, thiserror::Error)]
pub enum SearchError {
	#[error("Tantivy error: {0}")]
	Tantivy(#[from] tantivy::TantivyError),

	#[error("Query parse error: {0}")]
	QueryParse(#[from] tantivy::query::QueryParserError),

	#[error("Index not found at {0}")]
	IndexNotFound(PathBuf),

	#[error("IO error: {0}")]
	Io(#[from] std::io::Error),

	#[error("Schema field not found: {0}")]
	FieldNotFound(String),
}

/// Result type alias for search operations
pub type SearchResult<T> =
	std::result::Result<T, SearchError>;
