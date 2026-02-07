//! Error types for index manager operations.

use super::search::SearchError;
use super::watcher::WatcherError;

/// Error type for index manager operations
#[derive(Debug, thiserror::Error)]
pub enum IndexError {
	#[error("IO error: {0}")]
	Io(#[from] std::io::Error),

	#[error("Search error: {0}")]
	Search(#[from] SearchError),

	#[error("Watcher error: {0}")]
	Watcher(#[from] WatcherError),

	#[error("Parser error: {0}")]
	Parser(String),
}

/// Result type alias for index manager operations
pub type IndexManagerResult<T> =
	std::result::Result<T, IndexError>;
