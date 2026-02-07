//! Index service — project indexing use cases.

mod default;
pub mod types;

pub use default::DefaultIndexService;

use std::path::Path;

use crate::domain::errors::index::IndexError;
use crate::indexer::{IndexResult, IndexStats};

use types::IndexOptions;

/// Service trait for project indexing operations
pub trait IndexService {
	/// Index a project at the given path
	fn index_project(
		&self,
		path: &Path,
		opts: &IndexOptions,
	) -> Result<IndexResult, IndexError>;

	/// Get stats for an existing index
	fn get_stats(
		&self,
		path: &Path,
	) -> Result<Option<IndexStats>, IndexError>;

	/// Check if an index exists for a path
	fn has_index(&self, path: &Path) -> bool;

	/// Clear an existing index
	fn clear_index(
		&self,
		path: &Path,
	) -> Result<(), IndexError>;
}
