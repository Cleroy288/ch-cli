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

pub fn read_disk_timestamp(
	root: &Path,
) -> u64 {
	IndexState::load(root)
		.map(|state| state.last_updated)
		.unwrap_or(0)
}

pub fn build_index(
	path: &Path,
) -> Result<IndexResult, SearchError> {
	let manager = IndexManager::new()
		.with_persistence()
		.with_semantic_analysis();
	manager.index_project(path).map_err(|err| {
		SearchError::Io(std::io::Error::other(
			err.to_string(),
		))
	})
}

/// Create a SearchError from a message string
pub fn io_err(msg: &str) -> SearchError {
	SearchError::Io(std::io::Error::other(
		msg.to_string(),
	))
}
