//! Min-max normalization for Convex Combination fusion
//!
//! Normalizes keyword (BM25) and semantic (cosine distance)
//! scores into [0, 1] range for weighted combination.

use crate::indexer::SearchHit;
use crate::retrieval::hybrid::fusion::RankedItem;
use crate::retrieval::hybrid::vector_store::SearchResult
	as VectorSearchResult;

/// Min-max normalize a score into [0, 1].
/// Returns 0.5 when max == min (single result).
pub fn min_max_normalize(
	score: f32, min: f32, max: f32,
) -> f32 {
	if (max - min).abs() < f32::EPSILON {
		return 0.5;
	}
	(score - min) / (max - min)
}

/// Compute (min, max) from a slice of scores
pub fn score_bounds(scores: &[f32]) -> (f32, f32) {
	if scores.is_empty() {
		return (0.0, 0.0);
	}
	let min = scores
		.iter()
		.copied()
		.fold(f32::INFINITY, f32::min);
	let max = scores
		.iter()
		.copied()
		.fold(f32::NEG_INFINITY, f32::max);
	(min, max)
}

/// Normalize BM25 scores (higher = better) to [0,1].
/// Highest BM25 score maps to 1.0, lowest to 0.0.
pub fn normalize_keyword_scores(
	results: &[RankedItem<SearchHit>],
) -> Vec<f32> {
	if results.is_empty() {
		return Vec::new();
	}
	let scores: Vec<f32> =
		results.iter().map(|item| item.score).collect();
	let (min, max) = score_bounds(&scores);
	scores
		.iter()
		.map(|val| min_max_normalize(*val, min, max))
		.collect()
}

/// Normalize semantic distances (lower = better) to [0,1].
/// Lowest distance maps to 1.0, highest to 0.0.
pub fn normalize_semantic_scores(
	results: &[RankedItem<VectorSearchResult>],
) -> Vec<f32> {
	if results.is_empty() {
		return Vec::new();
	}
	let dists: Vec<f32> =
		results.iter().map(|item| item.score).collect();
	let (min, max) = score_bounds(&dists);
	// Invert: lowest distance = highest normalized score
	dists
		.iter()
		.map(|dist| {
			1.0 - min_max_normalize(*dist, min, max)
		})
		.collect()
}
