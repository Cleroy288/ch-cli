use std::collections::HashMap;

use crate::indexer::{DocumentType, Symbol};

use super::boost::{
	lookup_ref_count, ref_count_penalty,
	QueryIntent,
};
use super::types::SearchResultHit;

pub(super) fn boost_and_rank(
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
		})
		.collect()
}

/// All boost factors are f32 (matching Tantivy),
/// then widened to f64 at the DTO boundary.
/// Returns 0.0 for NaN/infinite base scores.
fn compute_boost(
	symbol: &Symbol,
	base: f32,
	intent: &QueryIntent,
	ref_counts: &HashMap<String, usize>,
) -> f64 {
	if !base.is_finite() {
		return 0.0;
	}
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
	let test_penalty = test_penalty(intent, symbol);

	let combined = base
		* doc_boost
		* kind_boost
		* test_penalty
		* ref_penalty;
	f64::from(combined)
}

/// Penalize test functions for Understand queries
fn test_penalty(
	intent: &QueryIntent,
	symbol: &Symbol,
) -> f32 {
	if matches!(intent, QueryIntent::Understand)
		&& symbol.name.starts_with("test_")
	{
		return 0.05;
	}
	1.0
}
