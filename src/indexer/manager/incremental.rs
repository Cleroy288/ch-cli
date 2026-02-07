//! Incremental indexing and persistence logic.

use std::path::{Path, PathBuf};

use crate::indexer::crawler::FileResult;
use crate::indexer::search::SearchIndex;
use crate::indexer::state::{ChangeSet, IndexState};
use crate::indexer::Symbol;

use super::error::IndexManagerResult;

/// Detect changes between current files and stored index state
pub fn detect_changes(
	root: &Path,
	current_files: &[PathBuf],
) -> IndexManagerResult<(Vec<PathBuf>, Option<ChangeSet>, IndexState, bool)> {
	if IndexState::exists(root) {
		// Load existing state
		let state = IndexState::load(root)?;
		let changes = state.detect_changes(current_files);

		if changes.has_changes() {
			// Incremental: only process changed files
			let files_to_process: Vec<PathBuf> =
				changes.files_to_index().into_iter().cloned().collect();
			Ok((files_to_process, Some(changes), state, true))
		} else {
			// No changes - still return the state but nothing to process
			Ok((Vec::new(), Some(changes), state, true))
		}
	} else {
		// No existing index - full reindex
		Ok((
			current_files.to_vec(),
			None,
			IndexState::new(root.to_path_buf()),
			false,
		))
	}
}

/// Merge new symbols with existing index and persist to disk
pub fn merge_and_persist(
	root: &Path,
	new_symbols: &[Symbol],
	file_results: &[FileResult],
	changes: &Option<ChangeSet>,
	index_state: &mut IndexState,
) -> IndexManagerResult<(Vec<Symbol>, Option<SearchIndex>)> {
	let tantivy_path = IndexState::tantivy_dir(root);
	let search_index = SearchIndex::open_or_create(&tantivy_path)?;

	// Collect files to delete and file results for batch update
	let mut files_to_delete: Vec<PathBuf> = Vec::new();

	// If incremental, collect deleted files
	if let Some(ref change_set) = changes {
		// Add deleted files to the delete list
		files_to_delete.extend(change_set.deleted.iter().cloned());

		// Update index state for deleted files
		for deleted_path in &change_set.deleted {
			index_state.remove_file(deleted_path);
		}
	}

	// Collect successful file results for batch update
	let file_symbols: Vec<(PathBuf, Vec<Symbol>)> = file_results
		.iter()
		.filter(|r| r.error.is_none())
		.map(|r| (r.path.clone(), r.symbols.clone()))
		.collect();

	// Update index state for processed files
	for result in file_results {
		if result.error.is_none() {
			index_state.update_file(&result.path, result.symbols.len())?;
		}
	}

	// Batch update Tantivy index (single commit!)
	search_index.batch_update(&files_to_delete, &file_symbols)?;

	// Update timestamp and save state
	index_state.touch();
	index_state.save()?;

	Ok((new_symbols.to_vec(), Some(search_index)))
}
