//! Semantic (hybrid) search logic.
//!
//! Contains hybrid search, intent boosting,
//! and orchestration for context + reranking.

use std::path::Path;

use crate::domain::errors::search::SearchError;
use crate::indexer::{DocumentType, IndexState};
use crate::retrieval::daemon::protocol::QueryIntent;
use crate::retrieval::docgen::DocStore;
use crate::retrieval::hybrid::embedding_version::{
	invalidate_vectors, is_cache_current,
	write_version,
};
use crate::retrieval::hybrid::{
	HybridSearch, HybridSearchResult,
};
use crate::retrieval::query::fallback_parse;
use crate::retrieval::RetrievalError;

use super::semantic_rerank::{
	build_context_xml, convert_to_hits,
	rerank_results,
};
use super::types::{SearchOptions, SearchResult};

/// Run semantic search with hybrid index
pub(super) fn run_semantic_search(
	query: &str,
	opts: &SearchOptions,
	index_result: &crate::indexer::IndexResult,
) -> Result<SearchResult, SearchError> {
	let raw = fetch_hybrid_results(
		query, opts, index_result,
	)?;
	let spec = fallback_parse(query);
	let boosted = boost_hybrid(raw, &spec.intent);
	let results =
		apply_rerank(query, opts, boosted)?;
	let context_xml = build_context_xml(
		query,
		&results,
		&index_result.semantic_graph,
		opts.flags.context,
	);
	let hits = convert_to_hits(results);
	Ok(SearchResult {
		hits,
		intent: spec.intent,
		context_xml,
	})
}

/// Fetch raw results from hybrid index
fn fetch_hybrid_results(
	query: &str,
	opts: &SearchOptions,
	index_result: &crate::indexer::IndexResult,
) -> Result<Vec<HybridSearchResult>, SearchError> {
	let hybrid =
		load_or_create_hybrid(index_result)?;
	let fetch_limit = if opts.flags.rerank {
		opts.limit * 3
	} else {
		opts.limit
	};
	hybrid
		.search(query, fetch_limit)
		.map_err(map_retrieval_err)
}

/// Apply reranking or truncate to limit
fn apply_rerank(
	query: &str,
	opts: &SearchOptions,
	boosted: Vec<HybridSearchResult>,
) -> Result<Vec<HybridSearchResult>, SearchError> {
	if opts.flags.rerank {
		rerank_results(query, boosted, opts.limit)
	} else {
		Ok(boosted
			.into_iter()
			.take(opts.limit)
			.collect())
	}
}

/// Map RetrievalError to SearchError
pub(super) fn map_retrieval_err(
	err: RetrievalError,
) -> SearchError {
	SearchError::IoError(std::io::Error::other(
		err.to_string(),
	))
}

/// Load hybrid search from cache or build fresh.
/// Invalidates vectors when embedding format changes.
fn load_or_create_hybrid(
	result: &crate::indexer::IndexResult,
) -> Result<HybridSearch, SearchError> {
	let index_dir =
		IndexState::index_dir(&result.root);
	invalidate_stale_cache(&index_dir);
	let mut hybrid =
		open_hybrid_index(&index_dir)?;

	// Skip indexing if vectors already loaded
	if !hybrid.vector_store().is_empty() {
		return Ok(hybrid);
	}

	hybrid
		.index_symbols(&result.symbols)
		.map_err(map_retrieval_err)?;
	index_enriched_if_available(
		&mut hybrid, &result.symbols, &result.root,
	)?;
	let _ = hybrid.persist();
	write_version(&index_dir);
	Ok(hybrid)
}

/// Invalidate stale vector cache if needed
fn invalidate_stale_cache(
	index_dir: &Path,
) {
	if !is_cache_current(index_dir) {
		invalidate_vectors(index_dir);
	}
}

/// Open hybrid index from disk paths
fn open_hybrid_index(
	index_dir: &Path,
) -> Result<HybridSearch, SearchError> {
	let tantivy = index_dir.join("tantivy");
	let vectors = index_dir.join("vectors.bin");
	let enriched =
		index_dir.join("enriched_vectors.bin");
	HybridSearch::with_paths(&tantivy, &vectors)
		.map_err(map_retrieval_err)?
		.with_enriched_path(&enriched)
		.map_err(map_retrieval_err)
}

/// Index enriched NL embeddings if DocStore exists
fn index_enriched_if_available(
	hybrid: &mut HybridSearch,
	symbols: &[crate::indexer::Symbol],
	root: &Path,
) -> Result<(), SearchError> {
	if !DocStore::exists(root) {
		return Ok(());
	}
	let store = DocStore::load(root)
		.map_err(SearchError::IoError)?;
	let _ = hybrid.index_enriched(symbols, &store);
	Ok(())
}

/// Apply intent-aware boosting to hybrid results
fn boost_hybrid(
	results: Vec<HybridSearchResult>,
	intent: &QueryIntent,
) -> Vec<HybridSearchResult> {
	let mut boosted: Vec<_> = results
		.into_iter()
		.map(|res| {
			let score =
				intent_boosted_score(&res, intent);
			(res, score)
		})
		.collect();
	boosted.sort_by(|lhs, rhs| {
		rhs.1
			.partial_cmp(&lhs.1)
			.unwrap_or(std::cmp::Ordering::Equal)
	});
	boosted
		.into_iter()
		.map(|(result, _)| result)
		.collect()
}

/// Compute intent-boosted score for a result
fn intent_boosted_score(
	result: &HybridSearchResult,
	intent: &QueryIntent,
) -> f32 {
	let doc_type = DocumentType::from_path(
		&result.symbol.location.file,
	);
	let doc_boost =
		doc_type.boost_factor_for_intent(intent);
	let kind_boost = result
		.symbol
		.kind
		.boost_factor_for_intent(intent);
	result.score * doc_boost * kind_boost
}
