//! Helper functions for index cache operations.

use std::path::{Path, PathBuf};

use crate::domain::errors::search::SearchError;
use crate::indexer::{
	IndexManager, IndexResult, IndexState,
};

/// Canonicalize path, fallback to original
pub fn canonicalize(path: &Path) -> PathBuf {
	std::fs::canonicalize(path)
		.unwrap_or_else(|_| path.to_path_buf())
}

/// Read last_updated from persisted IndexState
pub fn read_disk_timestamp(
	root: &Path,
) -> u64 {
	IndexState::load(root)
		.map(|state| state.last_updated)
		.unwrap_or(0)
}

/// Build a fresh IndexResult via IndexManager
pub fn build_index(
	path: &Path,
	semantic: bool,
) -> Result<IndexResult, SearchError> {
	let manager = if semantic {
		IndexManager::new()
			.with_persistence()
			.with_semantic_analysis()
	} else {
		IndexManager::new().with_persistence()
	};
	manager.index_project(path).map_err(|err| {
		SearchError::IoError(std::io::Error::other(
			err.to_string(),
		))
	})
}

/// Create a SearchError from a message string
pub fn io_err(msg: &str) -> SearchError {
	SearchError::IoError(std::io::Error::other(
		msg.to_string(),
	))
}
