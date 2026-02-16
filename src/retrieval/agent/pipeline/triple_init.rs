//! Triple hybrid search initialization.
//!
//! Initializes the code/doc/notes triple pipeline with
//! cache-first strategy: loads persisted vectors when
//! available, falls back to full GPU embedding otherwise.

use std::io::Write;
use std::path::Path;

use crate::indexer::Symbol;
use crate::retrieval::hybrid::embedding_callers::{
	precompute_caller_contexts,
};
use crate::retrieval::hybrid::hub_ref_counts::{
	precompute_ref_counts,
};
use crate::retrieval::hybrid::{
	HybridSearchConfig, TripleHybridSearch,
};
use crate::retrieval::RetrievalResult;

use super::core::RetrievalPipeline;
use super::hybrid_init::{
	hybrid_index_dir, invalidate_stale_cache,
};
use super::triple_init_cache::{
	check_dim_mismatch, try_load_triple_cache,
};

/// Initialize structured hybrid search (code/doc/notes)
pub fn initialize_triple_hybrid(
	pipeline: &mut RetrievalPipeline,
	symbols: &[Symbol],
	doc_store: Option<&crate::retrieval::docgen::DocStore>,
) -> RetrievalResult<()> {
	let index_dir = hybrid_index_dir(pipeline);
	invalidate_stale_cache(pipeline, &index_dir);

	if try_load_triple_cache(pipeline, symbols, &index_dir) {
		return Ok(());
	}

	build_and_persist_triple(
		pipeline, symbols, doc_store, &index_dir,
	)
}

/// Build fresh triple index and persist (expensive path)
fn build_and_persist_triple(
	pipeline: &mut RetrievalPipeline,
	symbols: &[Symbol],
	doc_store: Option<&crate::retrieval::docgen::DocStore>,
	index_dir: &Path,
) -> RetrievalResult<()> {
	let _ = writeln!(
		std::io::stderr().lock(),
		"[pipeline] Building triple hybrid index..."
	);

	let mut hybrid =
		build_triple_search(pipeline, index_dir)?;
	wire_graph_data(pipeline, &mut hybrid, symbols);

	let stats =
		hybrid.index_symbols_with_docs(symbols, doc_store)?;
	let _ = writeln!(
		std::io::stderr().lock(),
		"[pipeline] Indexed: {} code, {} doc, {} notes",
		stats.code_keyword_count,
		stats.doc_keyword_count,
		stats.notes_keyword_count
	);

	persist_triple(pipeline, &mut hybrid, index_dir);
	pipeline.triple_hybrid = Some(hybrid);
	Ok(())
}

/// Build the TripleHybridSearch instance
pub(super) fn build_triple_search(
	pipeline: &RetrievalPipeline,
	index_dir: &Path,
) -> RetrievalResult<TripleHybridSearch> {
	let hybrid_config = HybridSearchConfig {
		score_threshold: pipeline.config.score_threshold,
		min_results_per_type:
			pipeline.config.min_results_per_type,
		..HybridSearchConfig::default()
	};

	let mut hybrid =
		if pipeline.config.flags.enable_persistence {
			TripleHybridSearch::with_persistence(
				index_dir, pipeline.daemon.clone(),
			)?
		} else {
			TripleHybridSearch::new(pipeline.daemon.clone())?
		};
	hybrid = hybrid.with_config(hybrid_config);

	if pipeline.config.flags.enable_persistence {
		check_dim_mismatch(&mut hybrid, &pipeline.daemon)?;
	}
	Ok(hybrid)
}

/// Wire ref counts and caller contexts from graph
pub(super) fn wire_graph_data(
	pipeline: &RetrievalPipeline,
	hybrid: &mut TripleHybridSearch,
	symbols: &[Symbol],
) {
	if let Some(ref graph) = pipeline.graph {
		hybrid.ref_counts = precompute_ref_counts(graph);
		hybrid.caller_contexts =
			precompute_caller_contexts(symbols, graph);
	}
}

/// Persist triple hybrid index if enabled
fn persist_triple(
	pipeline: &RetrievalPipeline,
	hybrid: &mut TripleHybridSearch,
	index_dir: &Path,
) {
	if !pipeline.config.flags.enable_persistence {
		return;
	}
	if let Err(err) = hybrid.persist() {
		let _ = writeln!(
			std::io::stderr().lock(),
			"[pipeline] Warning: persist failed: {}",
			err
		);
	}
	crate::retrieval::hybrid::embedding_version::write_version(
		index_dir,
	);
}
