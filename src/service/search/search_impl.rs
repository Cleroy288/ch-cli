//! Keyword search execution logic.
//!
//! Contains business logic for keyword search
//! with intent-aware boosting, moved from handlers.

use std::collections::HashMap;

use crate::domain::errors::search::SearchError;
use crate::indexer::{
	DocumentType, IndexResult, SearchIndex, Symbol,
};
use crate::retrieval::daemon::protocol::QueryIntent;
use crate::retrieval::hybrid::hub_penalty::ref_count_penalty;
use crate::retrieval::hybrid::hub_ref_counts::{
	lookup_ref_count, precompute_ref_counts,
};
use crate::retrieval::query::fallback_parse;

use super::semantic_impl::run_semantic_search;
use super::types::{
	SearchOptions, SearchResult, SearchResultHit,
};

/// Execute a full search (keyword or hybrid).
/// Uses a pre-built IndexResult from the cache.
pub(super) fn execute_search(
	query: &str,
	opts: &SearchOptions,
	result: &IndexResult,
) -> Result<SearchResult, SearchError> {
	if opts.flags.semantic {
		return run_semantic_search(
			query, opts, result,
		);
	}
	run_keyword_search(query, opts, result)
}

/// Run keyword search with intent-aware boosting
fn run_keyword_search(
	query: &str,
	opts: &SearchOptions,
	result: &IndexResult,
) -> Result<SearchResult, SearchError> {
	let fallback; // holds in-memory index if needed
	let index = match result.search_index {
		Some(ref idx) => idx,
		None => {
			fallback = SearchIndex::in_memory()?;
			fallback
				.index_symbols(&result.symbols)?;
			&fallback
		}
	};
	search_with_index(query, opts, result, index)
}

/// Search with a resolved index reference
fn search_with_index(
	query: &str,
	opts: &SearchOptions,
	result: &IndexResult,
	index: &SearchIndex,
) -> Result<SearchResult, SearchError> {
	let ref_counts = build_ref_counts(result);
	let spec = fallback_parse(query);
	let intent = spec.intent;
	let raw = fetch_raw_hits(
		index, query, opts, &intent,
	)?;
	let hits = boost_and_rank(
		raw, &intent, opts.limit, &ref_counts,
	);
	Ok(SearchResult {
		hits,
		intent,
		context_xml: None,
	})
}

/// Build reference counts from semantic graph
fn build_ref_counts(
	result: &IndexResult,
) -> HashMap<String, usize> {
	result
		.semantic_graph
		.as_deref()
		.map(precompute_ref_counts)
		.unwrap_or_default()
}

/// Fetch raw hits from search index
fn fetch_raw_hits(
	index: &SearchIndex,
	query: &str,
	opts: &SearchOptions,
	intent: &QueryIntent,
) -> Result<
	Vec<crate::indexer::SearchHit>,
	SearchError,
> {
	let multiplier = match intent {
		QueryIntent::Understand => 10,
		_ => 3,
	};
	let limit = opts.limit * multiplier;

	if let Some(ref kind) = opts.kind {
		return index.search_by_kind(*kind, limit);
	}
	if opts.flags.fuzzy {
		return index.fuzzy_search(
			query, 2, limit,
		);
	}
	index.search_with_boost(query, limit)
}

/// Apply intent-aware boosts and sort by score
fn boost_and_rank(
	raw: Vec<crate::indexer::SearchHit>,
	intent: &QueryIntent,
	limit: usize,
	ref_counts: &HashMap<String, usize>,
) -> Vec<SearchResultHit> {
	let mut scored: Vec<_> = raw
		.into_iter()
		.map(|hit| {
			let score = compute_boost(
				&hit.symbol, hit.score,
				intent, ref_counts,
			);
			(hit, score)
		})
		.collect();
	sort_by_score_desc(&mut scored);
	scored_to_hits(scored, limit)
}

/// Sort (hit, score) pairs by score descending
fn sort_by_score_desc<T>(
	items: &mut [(T, f64)],
) {
	items.sort_by(|lhs, rhs| {
		rhs.1
			.partial_cmp(&lhs.1)
			.unwrap_or(std::cmp::Ordering::Equal)
	});
}

/// Convert scored hits into SearchResultHit vec
fn scored_to_hits(
	scored: Vec<(crate::indexer::SearchHit, f64)>,
	limit: usize,
) -> Vec<SearchResultHit> {
	scored
		.into_iter()
		.take(limit)
		.map(|(hit, score)| SearchResultHit {
			symbol: hit.symbol,
			score,
			keyword_rank: None,
			semantic_rank: None,
			rerank_score: None,
		})
		.collect()
}

/// Compute boosted score for a single hit
fn compute_boost(
	symbol: &Symbol,
	base: f32,
	intent: &QueryIntent,
	ref_counts: &HashMap<String, usize>,
) -> f64 {
	let doc_type =
		DocumentType::from_path(&symbol.location.file);
	let doc_boost =
		doc_type.boost_factor_for_intent(intent);
	let kind_boost =
		symbol.kind.boost_factor_for_intent(intent);
	let ref_count = lookup_ref_count(
		&symbol.name, ref_counts,
	);
	let ref_penalty = ref_count_penalty(ref_count);

	// Penalize test functions for Understand queries
	let test_penalty: f32 = if matches!(
		intent,
		QueryIntent::Understand
	) && symbol.name.starts_with("test_")
	{
		0.05
	} else {
		1.0
	};
	let combined = base
		* doc_boost
		* kind_boost
		* test_penalty
		* ref_penalty;
	combined as f64
}
