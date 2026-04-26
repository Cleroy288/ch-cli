use std::collections::HashMap;

use crate::domain::errors::search::SearchError;
use crate::indexer::{IndexResult, SearchIndex};

use super::boost::{
	precompute_ref_counts, QueryIntent,
};
use super::intent::parse_query_intent;
use super::search_impl_rank::boost_and_rank;
use super::types::{SearchOptions, SearchResult};

/// Uses a pre-built IndexResult from the cache.
pub(super) fn execute_search(
	query: &str,
	opts: &SearchOptions,
	result: &IndexResult,
) -> Result<SearchResult, SearchError> {
	let fallback; // holds in-memory index
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
	let parsed = parse_query_intent(query);
	let intent = parsed.intent;
	let raw = fetch_raw_hits(
		index, query, opts, &intent,
	)?;
	let hits = boost_and_rank(
		raw, &intent, opts.limit, &ref_counts,
	);
	Ok(SearchResult { hits, intent })
}

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
	let limit = opts
		.limit
		.max(1)
		.saturating_mul(multiplier);

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
