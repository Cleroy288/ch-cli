//! Core indexing logic for IndexManager.

use std::path::{Path, PathBuf};
use std::sync::Arc;
use std::time::Instant;

use crate::indexer::Symbol;

use super::builder::IndexManager;
use super::error::IndexManagerResult;
use super::indexing_helpers::{
	build_stats, canonicalize_root,
	collect_symbols, to_sym_refs,
};
use super::types::IndexResult;

impl IndexManager {
	/// Index an entire project directory
	pub fn index_project<P: AsRef<Path>>(
		&self,
		root: P,
	) -> IndexManagerResult<IndexResult> {
		let root = canonicalize_root(root.as_ref());
		let start = Instant::now();

		let all_files = self.discover_all_files(&root);
		let total = all_files.len();

		let (to_process, changes, mut state, incr) =
			self.detect_or_full(&root, &all_files)?;
		let count = to_process.len();

		let (results, extracted) =
			self.parse_files_parallel(&to_process, count);
		let new_syms = collect_symbols(&results);

		let (syms, idx) = self.persist_symbols(
			&root, &new_syms, &results,
			&changes, &mut state, incr,
		)?;

		let raw_refs = extracted.clone();
		let all_refs =
			self.resolve_refs(&root, extracted, &changes)?;
		let graph =
			self.build_semantic_graph(&syms, all_refs);

		Ok(build_project_result(
			root, syms, raw_refs, results,
			total, count, start, graph,
			incr, changes, idx,
		))
	}

	/// Index specific files only (targeted re-indexing)
	pub fn index_files<P: AsRef<Path>>(
		&self,
		root: P,
		files: &[PathBuf],
	) -> IndexManagerResult<IndexResult> {
		let root = canonicalize_root(root.as_ref());
		let start = Instant::now();

		let (results, extracted) =
			self.parse_files_parallel(files, files.len());
		let syms = collect_symbols(&results);
		let raw_refs = extracted.clone();
		let refs = to_sym_refs(extracted);
		let graph =
			self.build_semantic_graph(&syms, refs);

		Ok(build_files_result(
			root, syms, raw_refs, results,
			files.len(), start, graph,
		))
	}
}

/// Build IndexResult for project indexing
#[allow(clippy::too_many_arguments)]
fn build_project_result(
	root: PathBuf,
	symbols: Vec<Symbol>,
	references: Vec<crate::indexer::parser::ExtractedReference>,
	file_results: Vec<crate::indexer::crawler::FileResult>,
	total: usize,
	processed: usize,
	start: Instant,
	semantic_graph: Option<crate::indexer::semantic::SemanticGraph>,
	incremental: bool,
	changes: Option<crate::indexer::state::ChangeSet>,
	search_index: Option<crate::indexer::search::SearchIndex>,
) -> IndexResult {
	IndexResult {
		root,
		symbols,
		references,
		stats: build_stats(
			total, processed, &file_results, start,
		),
		file_results,
		semantic_graph: semantic_graph.map(Arc::new),
		incremental,
		changes,
		search_index,
	}
}

/// Build IndexResult for file-only indexing
#[allow(clippy::too_many_arguments)]
fn build_files_result(
	root: PathBuf,
	symbols: Vec<Symbol>,
	references: Vec<crate::indexer::parser::ExtractedReference>,
	file_results: Vec<crate::indexer::crawler::FileResult>,
	file_count: usize,
	start: Instant,
	semantic_graph: Option<crate::indexer::semantic::SemanticGraph>,
) -> IndexResult {
	IndexResult {
		root,
		symbols,
		references,
		stats: build_stats(
			file_count, file_count, &file_results, start,
		),
		file_results,
		semantic_graph: semantic_graph.map(Arc::new),
		incremental: false,
		changes: None,
		search_index: None,
	}
}
