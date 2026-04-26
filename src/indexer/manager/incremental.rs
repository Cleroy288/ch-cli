use std::path::{Path, PathBuf};

use crate::indexer::crawler::FileResult;
use crate::indexer::search::SearchIndex;
use crate::indexer::state::{ChangeSet, IndexState};
use crate::indexer::Symbol;

use super::error::IndexManagerResult;

pub struct MergePersistInput<'inp> {
	pub new_symbols: &'inp [Symbol],
	pub file_results: &'inp [FileResult],
	/// None = full reindex
	pub changes: &'inp Option<ChangeSet>,
	pub index_state: &'inp mut IndexState,
}

type ChangeDetectionResult = IndexManagerResult<
	(Vec<PathBuf>, Option<ChangeSet>, IndexState, bool),
>;

type MergeResult =
	IndexManagerResult<(Vec<Symbol>, Option<SearchIndex>)>;

type FileSymbolBatch = Vec<(PathBuf, Vec<Symbol>)>;

pub fn detect_changes(
	root: &Path,
	current_files: &[PathBuf],
) -> ChangeDetectionResult {
	if !IndexState::exists(root) {
		return Ok((
			current_files.to_vec(),
			None,
			IndexState::new(root.to_path_buf()),
			false,
		));
	}

	let state = IndexState::load(root)?;
	let changes = state.detect_changes(current_files);

	if changes.has_changes() {
		let to_process = changes
			.added
			.iter()
			.chain(changes.modified.iter())
			.cloned()
			.collect();
		Ok((to_process, Some(changes), state, true))
	} else {
		Ok((Vec::new(), Some(changes), state, true))
	}
}

pub fn merge_and_persist(
	root: &Path,
	mut input: MergePersistInput,
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

fn collect_deleted(input: &MergePersistInput) -> Vec<PathBuf> {
	input
		.changes
		.as_ref()
		.map(|cs| cs.deleted.clone())
		.unwrap_or_default()
}

fn collect_successful(
	input: &MergePersistInput,
) -> FileSymbolBatch {
	input
		.file_results
		.iter()
		.filter(|res| res.error.is_none())
		.map(|res| (res.path.clone(), res.symbols.clone()))
		.collect()
}

fn update_index_state(
	input: &mut MergePersistInput,
) -> IndexManagerResult<()> {
	if let Some(ref cs) = input.changes {
		for path in &cs.deleted {
			input.index_state.remove_file(path);
		}
	}
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
