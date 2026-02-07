//! Pipeline initialization logic.

use crate::indexer::{IndexManager, IndexState};
use crate::retrieval::docgen::DocStore;
use crate::retrieval::{RetrievalError, RetrievalResult};

use super::core::RetrievalPipeline;
use super::hybrid_init::{initialize_hybrid_search, initialize_triple_hybrid};
use super::persistence::{build_graph_with_persistence, build_trigram_index};

/// Initialize with local indexing (primary path)
pub fn initialize_local(
	pipeline: &mut RetrievalPipeline,
) -> RetrievalResult<()> {
	eprintln!(
		"[pipeline] Indexing project: {}",
		pipeline.config.project_path
	);

	// Build IndexManager with optional persistence
	// Enable reference extraction for cross-file usage tracking
	let manager = IndexManager::new()
		.with_semantic_analysis()
		.with_reference_extraction();
	let manager = if pipeline.config.enable_persistence {
		eprintln!("[pipeline] Persistence enabled (incremental indexing)");
		manager.with_persistence()
	} else {
		manager
	};

	let result = manager
		.index_project(&pipeline.config.project_path)
		.map_err(|e| RetrievalError::Embedding(e.to_string()))?;

	// Log incremental vs full index info
	if result.incremental {
		let changes = result
			.changes
			.as_ref()
			.map(|c| c.total_changes())
			.unwrap_or(0);
		eprintln!("[pipeline] Incremental index: {} changes", changes);
	}
	eprintln!("[pipeline] Found {} symbols", result.symbols.len());

	// Store symbols
	pipeline.symbols = Some(result.symbols.clone());

	// Clone project path to avoid borrow issues
	let project_path_owned =
		std::path::PathBuf::from(&pipeline.config.project_path);
	let project_path = project_path_owned.as_path();

	// Handle semantic graph with reference persistence
	pipeline.graph = build_graph_with_persistence(
		pipeline,
		result.semantic_graph,
		&result.references,
		&result.symbols,
		project_path,
		result.incremental,
	);

	// Build trigram index for fast pre-filtering
	pipeline.trigram_index = build_trigram_index(
		pipeline,
		&result.file_results,
		project_path,
		result.incremental,
	);

	// Build hybrid search index if semantic search enabled
	if pipeline.config.semantic_search {
		if pipeline.config.use_triple_pipeline {
			initialize_triple_hybrid(pipeline, &result.symbols)?;
		} else {
			initialize_hybrid_search(pipeline, &result.symbols)?;
		}
	}

	// Load doc store if doc context is enabled
	if pipeline.config.use_doc_context {
		load_doc_store(pipeline, project_path);
	}

	Ok(())
}

/// Initialize via daemon-side caching (alternative path)
#[allow(dead_code)]
pub fn initialize_via_daemon(
	pipeline: &mut RetrievalPipeline,
) -> RetrievalResult<()> {
	let project_path = std::fs::canonicalize(&pipeline.config.project_path)
		.map_err(|e| RetrievalError::Embedding(e.to_string()))?;
	let project_str = project_path.display().to_string();

	// Check if project is already cached in daemon
	let (cached, symbol_count, _last_indexed) =
		pipeline.daemon.project_status(&project_str)?;

	if cached {
		eprintln!("[pipeline] Using daemon cache ({} symbols)", symbol_count);
	} else {
		eprintln!(
			"[pipeline] Indexing project via daemon: {}",
			project_str
		);
		let (count, was_cached, time_ms) =
			pipeline.daemon.index_project(&project_str, false)?;
		if was_cached {
			eprintln!("[pipeline] Daemon cache hit ({} symbols)", count);
		} else {
			eprintln!(
				"[pipeline] Daemon indexed {} symbols in {}ms",
				count, time_ms
			);
		}
	}

	// We still need local graph for context expansion and hybrid search
	let manager = IndexManager::new()
		.with_semantic_analysis()
		.with_reference_extraction();
	let manager = if pipeline.config.enable_persistence {
		manager.with_persistence()
	} else {
		manager
	};

	let result = manager
		.index_project(&pipeline.config.project_path)
		.map_err(|e| RetrievalError::Embedding(e.to_string()))?;

	pipeline.symbols = Some(result.symbols.clone());
	pipeline.graph = result.semantic_graph;

	// Initialize hybrid search (needed for local search)
	if pipeline.config.semantic_search {
		initialize_hybrid_search(pipeline, &result.symbols)?;
	}

	Ok(())
}

/// Load documentation store for enhanced context
fn load_doc_store(
	pipeline: &mut RetrievalPipeline,
	project_path: &std::path::Path,
) {
	match DocStore::load(project_path) {
		Ok(store) => {
			if store.is_ready() {
				let stats = store.stats();
				eprintln!(
					"[pipeline] Loaded doc store ({} entries, {}% ready)",
					stats.total,
					(stats.completion_percent() as u32)
				);
				pipeline.doc_store = Some(store);
			} else {
				eprintln!(
					"[pipeline] Doc store not ready \
					(run 'ch-cli docs generate')"
				);
			}
		}
		Err(_) => {
			eprintln!(
				"[pipeline] No doc store found \
				(run 'ch-cli docs generate')"
			);
		}
	}
}
