//! Index query methods for IndexManager.

use std::path::Path;

use super::builder::IndexManager;
use super::error::IndexManagerResult;
use crate::indexer::state::IndexState;

impl IndexManager {
	/// Clear the index for a project (delete all persisted data)
	pub fn clear_index<P: AsRef<Path>>(root: P) -> IndexManagerResult<()> {
		let root = root.as_ref();
		let index_dir = IndexState::index_dir(root);

		if index_dir.exists() {
			std::fs::remove_dir_all(&index_dir)?;
		}

		Ok(())
	}

	/// Check if an index exists for the given project
	pub fn has_index<P: AsRef<Path>>(root: P) -> bool {
		IndexState::exists(root.as_ref())
	}

	/// Get index statistics without performing a full reindex
	pub fn get_index_stats<P: AsRef<Path>>(
		root: P,
	) -> IndexManagerResult<Option<super::types::IndexStats>> {
		let root = root.as_ref();

		if !IndexState::exists(root) {
			return Ok(None);
		}

		let state = IndexState::load(root)?;
		let tantivy_path = IndexState::tantivy_dir(root);

		let symbol_count = if tantivy_path.exists() {
			crate::indexer::search::SearchIndex::open_or_create(
				&tantivy_path
			)?
			.num_docs()
			.unwrap_or(0)
		} else {
			0
		};

		Ok(Some(super::types::IndexStats {
			root: root.to_path_buf(),
			file_count: state.files.len(),
			symbol_count: symbol_count as usize,
			last_updated: state.last_updated,
			version: state.version,
		}))
	}
}
