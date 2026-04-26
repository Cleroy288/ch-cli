use std::path::Path;

use crate::domain::errors::index::IndexError;
use crate::indexer::{IndexResult, IndexStats};

use super::types::IndexOptions;

/// Project indexing operations
pub trait IndexService {
	fn index_project(
		&self,
		path: &Path,
		opts: &IndexOptions,
	) -> Result<IndexResult, IndexError>;

	fn get_stats(
		&self,
		path: &Path,
	) -> Result<Option<IndexStats>, IndexError>;

	fn has_index(&self, path: &Path) -> bool;

	fn clear_index(
		&self,
		path: &Path,
	) -> Result<(), IndexError>;
}
