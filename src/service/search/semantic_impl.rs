//! Semantic (hybrid) search logic.
//!
//! Contains hybrid search, intent boosting,
//! and orchestration for context + reranking.

use crate::domain::errors::search::SearchError;
use crate::indexer::{DocumentType, Symbol};
use crate::retrieval::daemon::protocol::QueryIntent;
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
	let mut hybrid = create_hybrid()?;
	index_hybrid(
		&mut hybrid, &index_result.symbols,
	)?;

	let fetch = if opts.rerank {
		opts.limit * 3
	} else {
		opts.limit
	};
	let raw = hybrid
		.search(query, fetch)
		.map_err(map_retrieval_err)?;

	let spec = fallback_parse(query);
	let intent = spec.intent;
	let boosted = boost_hybrid(raw, &intent);

	let results = if opts.rerank {
		rerank_results(query, boosted, opts.limit)?
	} else {
		boosted
			.into_iter()
			.take(opts.limit)
			.collect()
	};

	let context_xml = build_context_xml(
		query,
		&results,
		&index_result.semantic_graph,
		opts.context,
	);
	let hits = convert_to_hits(results);

	Ok(SearchResult {
		hits,
		intent,
		context_xml,
	})
}

/// Map RetrievalError to SearchError
pub(super) fn map_retrieval_err(
	e: RetrievalError,
) -> SearchError {
	SearchError::Io(std::io::Error::new(
		std::io::ErrorKind::Other,
		e.to_string(),
	))
}

/// Create a new HybridSearch instance
fn create_hybrid(
) -> Result<HybridSearch, SearchError> {
	HybridSearch::new().map_err(map_retrieval_err)
}

/// Index symbols into the hybrid search
fn index_hybrid(
	hybrid: &mut HybridSearch,
	symbols: &[Symbol],
) -> Result<(), SearchError> {
	hybrid
		.index_symbols(symbols)
		.map_err(map_retrieval_err)
}

/// Apply intent-aware boosting to hybrid results
/// Computes per-result boost from doc type and kind
fn boost_hybrid(
	results: Vec<HybridSearchResult>,
	intent: &QueryIntent,
) -> Vec<HybridSearchResult> {
	let mut boosted: Vec<_> = results
		.into_iter()
		.map(|r| {
			let dt = DocumentType::from_path(
				&r.symbol.location.file,
			);
			let db =
				dt.boost_factor_for_intent(intent);
			let kb = r
				.symbol
				.kind
				.boost_factor_for_intent(intent);
			let score = r.rrf_score * db * kb;
			(r, score)
		})
		.collect();

	boosted.sort_by(|a, b| {
		b.1.partial_cmp(&a.1)
			.unwrap_or(std::cmp::Ordering::Equal)
	});
	boosted.into_iter().map(|(r, _)| r).collect()
}
