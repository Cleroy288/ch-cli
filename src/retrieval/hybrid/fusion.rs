//! Reciprocal Rank Fusion (RRF)
//!
//! Combines results from multiple search sources using the RRF algorithm.
//! RRF score = sum(1/(k+rank)) where k=60 is the standard constant.

use std::collections::HashMap;

/// Default k constant for RRF (from original paper)
pub const DEFAULT_K: f32 = 60.0;

/// A scored item from a single search source
#[derive(Debug, Clone)]
pub struct RankedItem<T> {
	/// the item
	pub item: T,
	/// rank in the result list (1-indexed)
	pub rank: usize,
	/// original score from the search source
	pub score: f32,
}

/// Result of RRF fusion
#[derive(Debug, Clone)]
pub struct FusedResult<T> {
	/// the fused item
	pub item: T,
	/// combined RRF score
	pub rrf_score: f32,
	/// keyword rank (None if not in keyword results)
	pub keyword_rank: Option<usize>,
	/// semantic rank (None if not in semantic results)
	pub semantic_rank: Option<usize>,
	/// original keyword score
	pub keyword_score: Option<f32>,
	/// original semantic score (distance, lower is better)
	pub semantic_score: Option<f32>,
}

/// Compute RRF score for a single rank
pub fn rrf_score(rank: usize, k: f32) -> f32 {
	1.0 / (k + rank as f32)
}

/// Fuse two result lists using Reciprocal Rank Fusion
///
/// # Arguments
/// * `keyword_results` - Results from keyword search (Tantivy)
/// * `semantic_results` - Results from semantic search (embeddings)
/// * `get_id` - Function to extract unique ID from item
/// * `k` - RRF constant (default 60)
///
/// # Returns
/// Fused results sorted by RRF score (descending)
pub fn fuse_results<T, F, I>(
	keyword_results: Vec<RankedItem<T>>,
	semantic_results: Vec<RankedItem<T>>,
	get_id: F,
	k: f32,
) -> Vec<FusedResult<T>>
where
	T: Clone,
	F: Fn(&T) -> I,
	I: std::hash::Hash + Eq,
{
	// track items by ID
	let mut items_by_id: HashMap<I, FusedResult<T>> = HashMap::new();

	// process keyword results
	for ranked in keyword_results {
		let id = get_id(&ranked.item);
		let score = rrf_score(ranked.rank, k);

		items_by_id
			.entry(id)
			.and_modify(|existing| {
				existing.rrf_score += score;
				existing.keyword_rank = Some(ranked.rank);
				existing.keyword_score = Some(ranked.score);
			})
			.or_insert(FusedResult {
				item: ranked.item,
				rrf_score: score,
				keyword_rank: Some(ranked.rank),
				semantic_rank: None,
				keyword_score: Some(ranked.score),
				semantic_score: None,
			});
	}

	// process semantic results
	for ranked in semantic_results {
		let id = get_id(&ranked.item);
		let score = rrf_score(ranked.rank, k);

		items_by_id
			.entry(id)
			.and_modify(|existing| {
				existing.rrf_score += score;
				existing.semantic_rank = Some(ranked.rank);
				existing.semantic_score = Some(ranked.score);
			})
			.or_insert(FusedResult {
				item: ranked.item,
				rrf_score: score,
				keyword_rank: None,
				semantic_rank: Some(ranked.rank),
				keyword_score: None,
				semantic_score: Some(ranked.score),
			});
	}

	// collect and sort by RRF score (descending)
	let mut results: Vec<_> = items_by_id.into_values().collect();
	results.sort_by(|a, b| {
		b.rrf_score.partial_cmp(&a.rrf_score).unwrap()
	});

	results
}

/// Fuse with default k=60
pub fn fuse_results_default<T, F, I>(
	keyword_results: Vec<RankedItem<T>>,
	semantic_results: Vec<RankedItem<T>>,
	get_id: F,
) -> Vec<FusedResult<T>>
where
	T: Clone,
	F: Fn(&T) -> I,
	I: std::hash::Hash + Eq,
{
	fuse_results(keyword_results, semantic_results, get_id, DEFAULT_K)
}

/// Fuse with weighted contributions
///
/// # Arguments
/// * `keyword_weight` - Weight for keyword results (default 1.0)
/// * `semantic_weight` - Weight for semantic results (default 1.0)
pub fn fuse_results_weighted<T, F, I>(
	keyword_results: Vec<RankedItem<T>>,
	semantic_results: Vec<RankedItem<T>>,
	get_id: F,
	k: f32,
	keyword_weight: f32,
	semantic_weight: f32,
) -> Vec<FusedResult<T>>
where
	T: Clone,
	F: Fn(&T) -> I,
	I: std::hash::Hash + Eq,
{
	let mut items_by_id: HashMap<I, FusedResult<T>> = HashMap::new();

	// process keyword results with weight
	for ranked in keyword_results {
		let id = get_id(&ranked.item);
		let score = rrf_score(ranked.rank, k) * keyword_weight;

		items_by_id
			.entry(id)
			.and_modify(|existing| {
				existing.rrf_score += score;
				existing.keyword_rank = Some(ranked.rank);
				existing.keyword_score = Some(ranked.score);
			})
			.or_insert(FusedResult {
				item: ranked.item,
				rrf_score: score,
				keyword_rank: Some(ranked.rank),
				semantic_rank: None,
				keyword_score: Some(ranked.score),
				semantic_score: None,
			});
	}

	// process semantic results with weight
	for ranked in semantic_results {
		let id = get_id(&ranked.item);
		let score = rrf_score(ranked.rank, k) * semantic_weight;

		items_by_id
			.entry(id)
			.and_modify(|existing| {
				existing.rrf_score += score;
				existing.semantic_rank = Some(ranked.rank);
				existing.semantic_score = Some(ranked.score);
			})
			.or_insert(FusedResult {
				item: ranked.item,
				rrf_score: score,
				keyword_rank: None,
				semantic_rank: Some(ranked.rank),
				keyword_score: None,
				semantic_score: Some(ranked.score),
			});
	}

	let mut results: Vec<_> = items_by_id.into_values().collect();
	results.sort_by(|a, b| b.rrf_score.partial_cmp(&a.rrf_score).unwrap());

	results
}

