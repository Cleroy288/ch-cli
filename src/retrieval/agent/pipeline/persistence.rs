//! Persistence logic for graph and trigram index.

use std::io::Write;
use std::sync::Arc;

use crate::indexer::{
	IndexState, SemanticGraph, Symbol,
	SymbolReference, TrigramIndex,
};

use super::core::RetrievalPipeline;

/// Input for building a semantic graph with persistence
pub struct GraphPersistInput<'life> {
	/// existing graph from indexing (if any, Arc-wrapped)
	pub graph: Option<Arc<SemanticGraph>>,
	/// newly extracted references
	pub new_refs: &'life [crate::indexer::parser::ExtractedReference],
	/// all symbols in the project
	pub symbols: &'life [Symbol],
	/// path to the project root
	pub project_path: &'life std::path::Path,
	/// whether this is an incremental update
	pub incremental: bool,
}

/// Build or load trigram index for fast text pre-filtering
///
/// Cache-first: loads persisted trigram index when available.
/// Rebuilds only when cache is missing or load fails.
pub fn build_trigram_index(
	pipeline: &RetrievalPipeline,
	file_results: &[crate::indexer::FileResult],
	project_path: &std::path::Path,
	_incremental: bool,
) -> Option<TrigramIndex> {
	if !pipeline.config.flags.enable_persistence {
		return None;
	}

	let trigram_file =
		IndexState::trigram_file(project_path);

	// Try loading cached trigram index first
	if trigram_file.exists() {
		if let Some(idx) =
			try_load_trigram_cache(&trigram_file)
		{
			return Some(idx);
		}
	}

	// Cache miss or load failed: rebuild from files
	if !file_results.is_empty() {
		return Some(build_fresh_trigram(
			file_results,
			&trigram_file,
		));
	}

	None
}

/// Try loading trigram index from cache file
fn try_load_trigram_cache(
	trigram_file: &std::path::Path,
) -> Option<TrigramIndex> {
	let _ = writeln!(
		std::io::stderr().lock(),
		"[pipeline] Loading cached trigram index..."
	);

	match TrigramIndex::load(trigram_file) {
		Ok(index) => {
			let stats = index.stats();
			let _ = writeln!(
				std::io::stderr().lock(),
				"[pipeline] Trigram index loaded \
				({} trigrams, {} files)",
				stats.trigram_count, stats.file_count
			);
			Some(index)
		}
		Err(err) => {
			let _ = writeln!(
				std::io::stderr().lock(),
				"[pipeline] Warning: trigram cache \
				load failed: {}",
				err
			);
			None
		}
	}
}

/// Build fresh trigram index from file results
fn build_fresh_trigram(
	file_results: &[crate::indexer::FileResult],
	trigram_file: &std::path::Path,
) -> TrigramIndex {
	let _ = writeln!(
		std::io::stderr().lock(),
		"[pipeline] Building trigram index..."
	);

	let mut index = TrigramIndex::new();
	for file_result in file_results {
		if file_result.error.is_some() {
			continue;
		}
		let Ok(content) =
			std::fs::read_to_string(&file_result.path)
		else {
			continue;
		};
		index.index_file(&file_result.path, &content);
	}

	log_trigram_stats(&index);
	save_trigram_index(&index, trigram_file);
	index
}

/// Log trigram index build stats
fn log_trigram_stats(index: &TrigramIndex) {
	let stats = index.stats();
	let _ = writeln!(
		std::io::stderr().lock(),
		"[pipeline] Built trigram index \
		({} trigrams, {} files)",
		stats.trigram_count, stats.file_count
	);
}

/// Persist trigram index to disk
fn save_trigram_index(
	index: &TrigramIndex,
	trigram_file: &std::path::Path,
) {
	if let Err(err) = index.save(trigram_file) {
		let _ = writeln!(
			std::io::stderr().lock(),
			"[pipeline] Warning: failed to \
			save trigram index: {}",
			err
		);
	}
}

/// Build semantic graph with reference persistence
pub fn build_graph_with_persistence<'ctx>(
	pipeline: &RetrievalPipeline,
	input: GraphPersistInput<'ctx>,
) -> Option<Arc<SemanticGraph>> {
	if !pipeline.config.flags.enable_persistence {
		return input.graph;
	}

	if !input.new_refs.is_empty() {
		persist_new_references(&input);
	}

	if input.incremental && input.new_refs.is_empty() {
		return try_load_cached_graph(&input);
	}

	input.graph
}

/// Save newly extracted references to disk
fn persist_new_references<'ctx>(
	input: &GraphPersistInput<'ctx>,
) {
	let refs: Vec<SymbolReference> = input
		.new_refs
		.iter()
		.map(|ref_item| SymbolReference {
			name: ref_item.name.clone(),
			location: ref_item.location.clone(),
			context: ref_item.context,
		})
		.collect();

	if let Err(err) =
		save_references(&refs, input.project_path)
	{
		let _ = writeln!(
			std::io::stderr().lock(),
			"[pipeline] Warning: failed to \
			save references: {}",
			err
		);
	} else {
		let _ = writeln!(
			std::io::stderr().lock(),
			"[pipeline] Saved {} references to cache",
			refs.len()
		);
	}
}

/// Try loading cached references and rebuild graph
fn try_load_cached_graph<'ctx>(
	input: &GraphPersistInput<'ctx>,
) -> Option<Arc<SemanticGraph>> {
	match IndexState::load_references(input.project_path)
	{
		Ok(cached_refs) if !cached_refs.is_empty() => {
			rebuild_graph_from_cache(
				input.symbols, cached_refs,
			)
		}
		Ok(_) => {
			let _ = writeln!(
				std::io::stderr().lock(),
				"[pipeline] No cached refs found"
			);
			None
		}
		Err(err) => {
			let _ = writeln!(
				std::io::stderr().lock(),
				"[pipeline] Warning: failed to \
				load cached refs: {}",
				err
			);
			None
		}
	}
}

/// Rebuild graph from cached symbol references
fn rebuild_graph_from_cache(
	symbols: &[Symbol],
	cached_refs: Vec<SymbolReference>,
) -> Option<Arc<SemanticGraph>> {
	let _ = writeln!(
		std::io::stderr().lock(),
		"[pipeline] Loaded {} refs from cache",
		cached_refs.len()
	);
	let mut graph = SemanticGraph::new();
	graph.add_symbols(symbols);
	for ref_item in cached_refs {
		graph.add_reference(ref_item);
	}
	Some(Arc::new(graph))
}

/// Save references to per-file cache
fn save_references(
	refs: &[SymbolReference],
	project_path: &std::path::Path,
) -> std::io::Result<()> {
	use crate::indexer::state::ref_persistence_helpers::save_grouped_refs;

	let refs_dir = IndexState::refs_dir(project_path);
	std::fs::create_dir_all(&refs_dir)?;

	save_grouped_refs(&refs_dir, refs, project_path)
}
