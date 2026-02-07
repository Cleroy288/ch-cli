//! Keyword search execution logic.
//!
//! Contains business logic for keyword search
//! with intent-aware boosting, moved from handlers.

use std::path::Path;

use crate::domain::errors::search::SearchError;
use crate::indexer::{
	DocumentType, IndexManager, SearchIndex, Symbol,
};
use crate::retrieval::daemon::protocol::QueryIntent;
use crate::retrieval::query::fallback_parse;

use super::semantic_impl::run_semantic_search;
use super::types::{
	SearchOptions, SearchResult, SearchResultHit,
};

/// Execute a full search (keyword or hybrid)
pub(super) fn execute_search(
	query: &str,
	path: &Path,
	opts: &SearchOptions,
) -> Result<SearchResult, SearchError> {
	let manager = if opts.context {
		IndexManager::new().with_semantic_analysis()
	} else {
		IndexManager::new()
	};
	let result =
		manager.index_project(path).map_err(|e| {
			SearchError::Io(std::io::Error::new(
				std::io::ErrorKind::Other,
				e.to_string(),
			))
		})?;

	if opts.semantic {
		return run_semantic_search(
			query, opts, &result,
		);
	}
	run_keyword_search(query, opts, &result)
}

/// Run keyword search with intent-aware boosting
fn run_keyword_search(
	query: &str,
	opts: &SearchOptions,
	result: &crate::indexer::IndexResult,
) -> Result<SearchResult, SearchError> {
	let index = SearchIndex::in_memory()?;
	index.index_symbols(&result.symbols)?;

	let spec = fallback_parse(query);
	let intent = spec.intent;
	let raw = fetch_raw_hits(
		&index, query, opts, &intent,
	)?;
	let hits =
		boost_and_rank(raw, &intent, opts.limit);

	Ok(SearchResult {
		hits,
		intent,
		context_xml: None,
	})
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
	if opts.fuzzy {
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
) -> Vec<SearchResultHit> {
	let mut scored: Vec<_> = raw
		.into_iter()
		.map(|h| {
			let score = compute_boost(
				&h.symbol, h.score, intent,
			);
			(h, score)
		})
		.collect();

	scored.sort_by(|a, b| {
		b.1.partial_cmp(&a.1)
			.unwrap_or(std::cmp::Ordering::Equal)
	});

	scored
		.into_iter()
		.take(limit)
		.map(|(h, s)| SearchResultHit {
			symbol: h.symbol,
			score: s,
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
) -> f64 {
	let dt =
		DocumentType::from_path(&symbol.location.file);
	let db = dt.boost_factor_for_intent(intent);
	let kb =
		symbol.kind.boost_factor_for_intent(intent);

	// Penalize test functions for Understand queries
	let tp: f32 = if matches!(
		intent,
		QueryIntent::Understand
	) && symbol.name.starts_with("test_")
	{
		0.05
	} else {
		1.0
	};
	(base * db * kb * tp) as f64
}
