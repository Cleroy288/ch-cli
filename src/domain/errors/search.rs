use std::path::PathBuf;

#[derive(Debug, thiserror::Error)]
pub enum SearchError {
	#[error("tantivy: {0}")]
	Tantivy(String),

	#[error("query parse: {0}")]
	QueryParse(String),

	#[error("index not found at {0}")]
	IndexNotFound(PathBuf),

	#[error("{0}")]
	Io(#[from] std::io::Error),

	#[error("schema field not found: {0}")]
	FieldNotFound(String),
}

pub type SearchResult<T> =
	Result<T, SearchError>;
