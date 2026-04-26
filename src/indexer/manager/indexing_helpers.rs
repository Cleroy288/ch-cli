//! IndexManager pipeline step methods.

use std::path::{Path, PathBuf};

use crate::indexer::crawler::{Crawler, FileResult};
use crate::indexer::parser::ExtractedReference;
use crate::indexer::search::SearchIndex;
use crate::indexer::semantic::{SemanticGraph, SymbolReference};
use crate::indexer::state::ref_persistence::merge_and_save_refs;
use crate::indexer::state::{ChangeSet, IndexState};
use crate::indexer::Symbol;

use super::builder::IndexManager;
use super::error::IndexManagerResult;
use super::incremental::{
	MergePersistInput, detect_changes, merge_and_persist,
};
use super::indexing_fns::{reload_if_empty, to_sym_refs};

impl IndexManager {
	/// Discover all indexable files in the project
	pub(super) fn discover_all_files(
		&self,
		root: &Path,
	) -> Vec<PathBuf> {
		let crawler =
			Crawler::with_config(self.crawler_config.clone());
		crawler.discover_files(root)
	}

	/// Detect changes or return full file list
	#[allow(clippy::type_complexity)]
	pub(super) fn detect_or_full(
		&self,
		root: &Path,
		all_files: &[PathBuf],
	) -> IndexManagerResult<(
		Vec<PathBuf>,
		Option<ChangeSet>,
		IndexState,
		bool,
	)> {
		if self.flags.persistence {
			detect_changes(root, all_files)
		} else {
			Ok((
				all_files.to_vec(),
				None,
				IndexState::new(root.to_path_buf()),
				false,
			))
		}
	}

	/// Persist symbols via merge or return as-is
	#[allow(clippy::type_complexity)]
	pub(super) fn persist_symbols(
		&self,
		root: &Path,
		new_syms: &[Symbol],
		file_results: &[FileResult],
		changes: &Option<ChangeSet>,
		state: &mut IndexState,
		incremental: bool,
	) -> IndexManagerResult<(
		Vec<Symbol>,
		Option<SearchIndex>,
	)> {
		if !self.flags.persistence {
			return Ok((new_syms.to_vec(), None));
		}
		let (syms, idx) = merge_and_persist(
			root,
			MergePersistInput {
				new_symbols: new_syms,
				file_results,
				changes,
				index_state: state,
			},
		)?;
		let syms =
			reload_if_empty(syms, &idx, incremental);
		Ok((syms, idx))
	}

	/// Resolve references: merge with cache if persistent
	pub(super) fn resolve_refs(
		&self,
		root: &Path,
		extracted: Vec<ExtractedReference>,
		changes: &Option<ChangeSet>,
	) -> IndexManagerResult<Vec<SymbolReference>> {
		let sym_refs = to_sym_refs(extracted);
		if !self.flags.persistence {
			return Ok(sym_refs);
		}
		if !sym_refs.is_empty() {
			Ok(merge_and_save_refs(
				root, &sym_refs, changes,
			)?)
		} else {
			Ok(IndexState::load_references(root)?)
		}
	}

	/// Build semantic graph from symbols and refs.
	/// Consumes refs to avoid cloning each one.
	pub(super) fn build_semantic_graph(
		&self,
		symbols: &[Symbol],
		references: Vec<SymbolReference>,
	) -> Option<SemanticGraph> {
		if !self.flags.semantic_analysis {
			return None;
		}
		let mut graph = SemanticGraph::new();
		graph.add_symbols(symbols);
		for reference in references {
			graph.add_reference(reference);
		}
		Some(graph)
	}
}
