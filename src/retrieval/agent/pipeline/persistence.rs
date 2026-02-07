//! Persistence logic for graph and trigram index.

use crate::indexer::{
	IndexState, SemanticGraph, Symbol, SymbolReference, TrigramIndex,
};

use super::core::RetrievalPipeline;

/// Build or load trigram index for fast text pre-filtering
pub fn build_trigram_index(
	pipeline: &RetrievalPipeline,
	file_results: &[crate::indexer::FileResult],
	project_path: &std::path::Path,
	incremental: bool,
) -> Option<TrigramIndex> {
	if !pipeline.config.enable_persistence {
		return None;
	}

	let trigram_file = IndexState::trigram_file(project_path);

	// If incremental with no changes, try to load from cache
	if incremental && file_results.is_empty() {
		match TrigramIndex::load(&trigram_file) {
			Ok(index) => {
				let stats = index.stats();
				eprintln!(
					"[pipeline] Loaded trigram index ({} trigrams, {} files)",
					stats.trigram_count, stats.file_count
				);
				return Some(index);
			}
			Err(e) => {
				eprintln!("[pipeline] Warning: failed to load trigram index: {}", e);
			}
		}
	}

	// Build fresh trigram index from file results
	if !file_results.is_empty() {
		let mut index = TrigramIndex::new();

		for file_result in file_results {
			if file_result.error.is_none() {
				if let Ok(content) = std::fs::read_to_string(&file_result.path) {
					index.index_file(&file_result.path, &content);
				}
			}
		}

		let stats = index.stats();
		eprintln!(
			"[pipeline] Built trigram index ({} trigrams, {} files)",
			stats.trigram_count, stats.file_count
		);

		// Save to disk
		if let Err(e) = index.save(&trigram_file) {
			eprintln!("[pipeline] Warning: failed to save trigram index: {}", e);
		}

		return Some(index);
	}

	None
}

/// Build semantic graph with reference persistence
pub fn build_graph_with_persistence(
	pipeline: &RetrievalPipeline,
	graph: Option<SemanticGraph>,
	new_refs: &[crate::indexer::parser::ExtractedReference],
	symbols: &[Symbol],
	project_path: &std::path::Path,
	incremental: bool,
) -> Option<SemanticGraph> {
	if !pipeline.config.enable_persistence {
		return graph;
	}

	// If we have new references, save them
	if !new_refs.is_empty() {
		let refs: Vec<SymbolReference> = new_refs
			.iter()
			.map(|r| SymbolReference {
				name: r.name.clone(),
				location: r.location.clone(),
				context: r.context.clone(),
			})
			.collect();

		if let Err(e) = save_references(&refs, project_path) {
			eprintln!("[pipeline] Warning: failed to save references: {}", e);
		} else {
			eprintln!("[pipeline] Saved {} references to cache", refs.len());
		}
	}

	// If incremental with 0 changes, load references from cache
	if incremental && new_refs.is_empty() {
		match IndexState::load_references(project_path) {
			Ok(cached_refs) if !cached_refs.is_empty() => {
				eprintln!(
					"[pipeline] Loaded {} references from cache",
					cached_refs.len()
				);

				// Rebuild graph with cached references
				let mut new_graph = SemanticGraph::new();
				new_graph.add_symbols(symbols);
				for ref_item in cached_refs {
					new_graph.add_reference(ref_item);
				}
				return Some(new_graph);
			}
			Ok(_) => {
				eprintln!("[pipeline] No cached references found");
			}
			Err(e) => {
				eprintln!(
					"[pipeline] Warning: failed to load cached references: {}",
					e
				);
			}
		}
	}

	graph
}

/// Save references to cache
fn save_references(
	refs: &[SymbolReference],
	project_path: &std::path::Path,
) -> std::io::Result<()> {
	let index_dir = IndexState::index_dir(project_path);
	std::fs::create_dir_all(&index_dir)?;

	let refs_file = IndexState::refs_file(project_path);
	let content = serde_json::to_string(refs)
		.map_err(|e| std::io::Error::new(std::io::ErrorKind::InvalidData, e))?;
	std::fs::write(&refs_file, content)?;

	Ok(())
}
