//! Hybrid search initialization logic.

use crate::indexer::{IndexState, Symbol};
use crate::retrieval::hybrid::{
	HybridSearch, HybridSearchConfig, TripleHybridSearch,
};
use crate::retrieval::RetrievalResult;

use super::core::RetrievalPipeline;

/// Initialize hybrid search with optional persistence
pub fn initialize_hybrid_search(
	pipeline: &mut RetrievalPipeline,
	symbols: &[Symbol],
) -> RetrievalResult<()> {
	let project_path = std::path::Path::new(&pipeline.config.project_path);
	let index_dir = IndexState::index_dir(project_path); // .ch-index directory
	let tantivy_path = index_dir.join("tantivy");
	let vector_path = index_dir.join("vectors.json");

	// Try to load from persistent paths if enabled and they exist
	let can_load_from_cache = pipeline.config.enable_persistence
		&& tantivy_path.exists()
		&& vector_path.exists();

	if can_load_from_cache {
		eprintln!("[pipeline] Loading cached hybrid search index...");
		match HybridSearch::with_paths(&tantivy_path, &vector_path) {
			Ok(hybrid) => {
				eprintln!("[pipeline] Hybrid index loaded from cache");
				pipeline.hybrid = Some(hybrid);
				return Ok(());
			}
			Err(e) => {
				eprintln!("[pipeline] Cache load failed, rebuilding: {}", e);
			}
		}
	}

	// Build fresh index
	eprintln!("[pipeline] Building hybrid search index...");
	let mut hybrid = if pipeline.config.enable_persistence {
		// Create with paths for persistence
		HybridSearch::with_paths(&tantivy_path, &vector_path)?
	} else {
		HybridSearch::new()?
	};

	hybrid.index_symbols(symbols)?;

	// Persist vectors if enabled
	if pipeline.config.enable_persistence {
		if let Err(e) = hybrid.persist() {
			eprintln!("[pipeline] Warning: failed to persist vectors: {}", e);
		}
	}

	pipeline.hybrid = Some(hybrid);
	Ok(())
}

/// Initialize structured hybrid search (code, doc, notes pipelines)
pub fn initialize_triple_hybrid(
	pipeline: &mut RetrievalPipeline,
	symbols: &[Symbol],
) -> RetrievalResult<()> {
	let project_path = std::path::Path::new(&pipeline.config.project_path);
	let index_dir = IndexState::index_dir(project_path);

	eprintln!("[pipeline] Building hybrid search index...");

	// Build HybridSearchConfig from PipelineConfig threshold values
	let hybrid_config = HybridSearchConfig {
		rrf_score_threshold: pipeline.config.rrf_score_threshold,
		min_results_per_type: pipeline.config.min_results_per_type,
		..HybridSearchConfig::default()
	};

	let mut hybrid = if pipeline.config.enable_persistence {
		TripleHybridSearch::with_persistence(&index_dir, pipeline.daemon.clone())?
			.with_config(hybrid_config)
	} else {
		TripleHybridSearch::new(pipeline.daemon.clone())?.with_config(hybrid_config)
	};

	let stats = hybrid.index_symbols(symbols)?;
	eprintln!(
		"[pipeline] Indexed: {} code, {} doc, {} notes",
		stats.code_keyword_count, stats.doc_keyword_count, stats.notes_keyword_count
	);

	if pipeline.config.enable_persistence {
		if let Err(e) = hybrid.persist() {
			eprintln!("[pipeline] Warning: failed to persist index: {}", e);
		}
	}

	pipeline.triple_hybrid = Some(hybrid);
	Ok(())
}
