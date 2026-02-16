//! Pipeline initialization logic (local path).

use std::io::Write;

use crate::indexer::IndexManager;
use crate::retrieval::{RetrievalError, RetrievalResult};

use super::core::RetrievalPipeline;
use super::hybrid_init::initialize_hybrid_search;
use super::triple_init::initialize_triple_hybrid;
use super::persistence::{
	GraphPersistInput, build_graph_with_persistence,
	build_trigram_index,
};

use super::initialization_helpers::{
	load_doc_store, log_index_stats,
};

/// Initialize with local indexing (primary path)
pub fn initialize_local(
	pipeline: &mut RetrievalPipeline,
) -> RetrievalResult<()> {
	let _ = writeln!(
		std::io::stderr().lock(),
		"[pipeline] Indexing project: {}",
		pipeline.config.project_path
	);

	let result = run_local_indexing(pipeline)?;
	store_index_results(pipeline, result);
	init_hybrid_if_enabled(pipeline)
}

/// Run the local index manager and return results
fn run_local_indexing(
	pipeline: &RetrievalPipeline,
) -> RetrievalResult<crate::indexer::IndexResult> {
	let manager = IndexManager::new()
		.with_semantic_analysis()
		.with_reference_extraction();
	let manager =
		if pipeline.config.flags.enable_persistence {
			let _ = writeln!(
				std::io::stderr().lock(),
				"[pipeline] Persistence enabled \
				(incremental indexing)"
			);
			manager.with_persistence()
		} else {
			manager
		};

	manager
		.index_project(&pipeline.config.project_path)
		.map_err(|err| {
			RetrievalError::Embedding(err.to_string())
		})
}

/// Store symbols, graph, trigram, and doc store.
/// Takes ownership of IndexResult to avoid cloning
/// the symbols vector (~10MB for large projects).
fn store_index_results(
	pipeline: &mut RetrievalPipeline,
	result: crate::indexer::IndexResult,
) {
	log_index_stats(&result);

	let project_path = std::path::PathBuf::from(
		&pipeline.config.project_path,
	);
	let path = project_path.as_path();

	pipeline.graph = build_graph_with_persistence(
		pipeline,
		GraphPersistInput {
			graph: result.semantic_graph.clone(),
			new_refs: &result.references,
			symbols: &result.symbols,
			project_path: path,
			incremental: result.incremental,
		},
	);

	pipeline.trigram_index = build_trigram_index(
		pipeline,
		&result.file_results,
		path,
		result.incremental,
	);

	if pipeline.config.flags.use_doc_context {
		load_doc_store(pipeline, path);
	}

	// Move symbols last (after all borrows are done)
	pipeline.symbols = Some(result.symbols);
}

/// Initialize hybrid search if semantic search enabled.
/// Temporarily takes symbols from pipeline to avoid
/// borrow conflict with &mut pipeline.
fn init_hybrid_if_enabled(
	pipeline: &mut RetrievalPipeline,
) -> RetrievalResult<()> {
	if !pipeline.config.flags.semantic_search {
		return Ok(());
	}

	// Take symbols out to avoid borrow conflict
	let symbols = pipeline.symbols.take()
		.unwrap_or_default();

	let result =
		if pipeline.config.flags.use_triple_pipeline {
			let doc_store = pipeline.doc_store.take();
			let res = initialize_triple_hybrid(
				pipeline,
				&symbols,
				doc_store.as_ref(),
			);
			pipeline.doc_store = doc_store;
			res
		} else {
			initialize_hybrid_search(
				pipeline, &symbols,
			)
		};

	pipeline.symbols = Some(symbols); // restore
	result
}
