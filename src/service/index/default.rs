//! Default implementation of IndexService.

use std::path::Path;

use crate::domain::errors::index::IndexError;
use crate::indexer::{
	IndexManager, IndexResult, IndexStats,
};

use super::types::IndexOptions;
use super::IndexService;

/// Default index service backed by IndexManager
#[derive(Default)]
pub struct DefaultIndexService;

impl IndexService for DefaultIndexService {
	fn index_project(
		&self,
		path: &Path,
		opts: &IndexOptions,
	) -> Result<IndexResult, IndexError> {
		let mut manager = IndexManager::new()
			.with_semantic_analysis()
			.with_reference_extraction();
		if opts.persistence {
			manager = manager.with_persistence();
		}
		manager.index_project(path)
	}

	fn get_stats(
		&self,
		path: &Path,
	) -> Result<Option<IndexStats>, IndexError> {
		IndexManager::get_index_stats(path)
	}

	fn has_index(&self, path: &Path) -> bool {
		IndexManager::has_index(path)
	}

	fn clear_index(
		&self,
		path: &Path,
	) -> Result<(), IndexError> {
		IndexManager::clear_index(path)
	}
}
