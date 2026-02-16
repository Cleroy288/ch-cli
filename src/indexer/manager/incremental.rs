//! Incremental indexing and persistence logic.

use std::path::{Path, PathBuf};

use crate::indexer::crawler::FileResult;
use crate::indexer::search::SearchIndex;
use crate::indexer::state::{ChangeSet, IndexState};
use crate::indexer::Symbol;

use super::error::IndexManagerResult;

/// Input for merging new symbols and persisting state
pub struct MergePersistInput<'inp> {
	/// new symbols from parsed files
	pub new_symbols: &'inp [Symbol],
	/// parsed file results
	pub file_results: &'inp [FileResult],
	/// detected changes (None = full reindex)
	pub changes: &'inp Option<ChangeSet>,
	/// mutable index state to update
	pub index_state: &'inp mut IndexState,
}

/// Result of change detection: (files, changes, state, existed)
type ChangeDetectionResult = IndexManagerResult<
	(Vec<PathBuf>, Option<ChangeSet>, IndexState, bool),
>;

/// Result of merge: (merged symbols, optional search index)
type MergeResult =
	IndexManagerResult<(Vec<Symbol>, Option<SearchIndex>)>;

/// Batch of symbols grouped by file path
type FileSymbolBatch = Vec<(PathBuf, Vec<Symbol>)>;

/// Detect changes between current files and stored index state
pub fn detect_changes(
	root: &Path,
	current_files: &[PathBuf],
) -> ChangeDetectionResult {
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

/// Merge new symbols with existing index and persist
pub fn merge_and_persist<'inp>(
	root: &Path,
	mut input: MergePersistInput<'inp>,
) -> MergeResult {
	let tantivy_path = IndexState::tantivy_dir(root);
	let search_index =
		SearchIndex::open_or_create(&tantivy_path)?;

	let files_to_delete = collect_deleted(&input);
	let file_symbols = collect_successful(&input);

	update_index_state(&mut input)?;

	search_index
		.batch_update(&files_to_delete, &file_symbols)?;

	input.index_state.touch();
	input.index_state.save()?;

	Ok((input.new_symbols.to_vec(), Some(search_index)))
}

/// Collect deleted file paths from change set
fn collect_deleted<'inp>(input: &MergePersistInput<'inp>) -> Vec<PathBuf> {
	input
		.changes
		.as_ref()
		.map(|change_set| change_set.deleted.clone())
		.unwrap_or_default()
}

/// Collect successful parse results as (path, symbols) pairs
fn collect_successful<'inp>(
	input: &MergePersistInput<'inp>,
) -> FileSymbolBatch {
	input
		.file_results
		.iter()
		.filter(|res| res.error.is_none())
		.map(|res| (res.path.clone(), res.symbols.clone()))
		.collect()
}

/// Update index state for deleted and processed files
fn update_index_state<'inp>(
	input: &mut MergePersistInput<'inp>,
) -> IndexManagerResult<()> {
	// Remove deleted files from state
	if let Some(ref change_set) = input.changes {
		for path in &change_set.deleted {
			input.index_state.remove_file(path);
		}
	}
	// Update state for processed files
	for result in input.file_results {
		if result.error.is_none() {
			input.index_state.update_file(
				&result.path,
				result.symbols.len(),
			)?;
		}
	}
	Ok(())
}
