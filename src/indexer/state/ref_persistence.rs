//! Reference persistence with per-file storage.
//!
//! Stores references in a `refs/` directory with one
//! JSON file per source file, enabling incremental
//! updates instead of rewriting a monolithic refs.json.

use std::path::Path;

use crate::indexer::semantic::SymbolReference;

use super::ref_persistence_helpers::{
	delete_stale_files, load_all_refs,
	migrate_if_needed, save_grouped_refs,
};
use super::types::{ChangeSet, IndexState};

/// Merge new refs with cached, remove stale, save per-file.
pub fn merge_and_save_refs(
	root: &Path,
	new_refs: &[SymbolReference],
	changes: &Option<ChangeSet>,
) -> std::io::Result<Vec<SymbolReference>> {
	let refs_dir = IndexState::refs_dir(root);
	migrate_if_needed(root, &refs_dir)?;

	delete_stale_files(&refs_dir, changes, root)?;
	std::fs::create_dir_all(&refs_dir)?;
	save_grouped_refs(&refs_dir, new_refs, root)?;
	load_all_refs(&refs_dir)
}
