//! Enriched Fusion for HybridSearch
//!
//! Boosts existing hybrid search results using NL-enriched
//! vector store scores from LLM-generated documentation.

use std::collections::HashMap;

use crate::retrieval::hybrid::normalize::{
	min_max_normalize, score_bounds,
};
use crate::retrieval::hybrid::vector_store::{
	PointMeta, SearchResult as VectorSearchResult,
};
use crate::retrieval::hybrid::HybridSearchResult;

/// Weight for enriched NL results (low to avoid hub effect)
const ENRICHED_WEIGHT: f32 = 0.25;

/// Boost existing results with enriched NL scores.
/// Only boosts results already found by keyword+semantic.
pub fn boost_with_enriched(
	results: &mut [HybridSearchResult],
	enriched_hits: &[VectorSearchResult],
) {
	if enriched_hits.is_empty() || results.is_empty() {
		return;
	}
	let scores = build_enriched_scores(enriched_hits);
	apply_boost(results, &scores);
	results.sort_by(|left, right| {
		right
			.score
			.partial_cmp(&left.score)
			.unwrap_or(std::cmp::Ordering::Equal)
	});
}

/// Build normalized score lookup from enriched hits
fn build_enriched_scores(
	hits: &[VectorSearchResult],
) -> HashMap<String, f32> {
	let dists: Vec<f32> =
		hits.iter().map(|hit| hit.distance).collect();
	let (dist_min, dist_max) = score_bounds(&dists);

	hits.iter()
		.map(|hit| {
			let key = build_point_key(&hit.point);
			let norm = 1.0
				- min_max_normalize(
					hit.distance, dist_min, dist_max,
				);
			(key, norm * ENRICHED_WEIGHT)
		})
		.collect()
}

/// Apply score boosts from enriched lookup
fn apply_boost(
	results: &mut [HybridSearchResult],
	scores: &HashMap<String, f32>,
) {
	for result in results.iter_mut() {
		let key = build_result_key(result);
		if let Some(&boost) = scores.get(&key) {
			result.score += boost;
		}
	}
}

/// Build lookup key from HybridSearchResult
pub fn build_result_key(
	result: &HybridSearchResult,
) -> String {
	format!(
		"{}:{}:{}",
		result.symbol.location.file.display(),
		result.symbol.location.line,
		result.symbol.name,
	)
}

/// Build lookup key from PointMeta
pub fn build_point_key(
	point: &PointMeta,
) -> String {
	format!(
		"{}:{}:{}",
		point.file_path.display(),
		point.line,
		point.symbol_name,
	)
}
