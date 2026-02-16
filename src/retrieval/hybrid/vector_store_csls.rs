//! Local Scaling for hubness reduction
//!
//! Replaces CSLS with the Local Scaling algorithm (Zelnik-Manor
//! & Perona 2004). Hub points have small sigma (close to many
//! neighbors), so LS automatically pushes them away without
//! any hardcoded lists.

use hnsw_rs::prelude::*;

use crate::retrieval::hybrid::vector_store::{
	SearchResult, VectorStore,
};

/// K for sigma: distance to k-th nearest neighbor
const LOCAL_SCALING_K: usize = 7;

/// Near-zero threshold to avoid division by zero
const EPSILON: f32 = 1e-10;

/// Default sigma when no neighbors available.
/// 1.0 = max cosine distance (opposite vectors).
const DEFAULT_SIGMA: f32 = 1.0;

/// Precompute sigma (k-th NN distance) for all points.
/// Builds the HNSW index once, then searches all points.
pub fn compute_all_sigma(store: &VectorStore) -> Vec<f32> {
	if store.points.is_empty() {
		return Vec::new();
	}

	let hnsw = store.build_hnsw_index();
	let neighbor_k = LOCAL_SCALING_K + 1;
	let ef_search = neighbor_k.max(24);

	store
		.points
		.iter()
		.enumerate()
		.map(|(idx, point)| {
			kth_distance_from_index(
				&hnsw,
				&point.vector,
				idx,
				ef_search,
			)
		})
		.collect()
}

/// k-th NN distance using a pre-built HNSW index.
/// Excludes the point itself (self-match).
fn kth_distance_from_index(
	hnsw: &hnsw_rs::prelude::Hnsw<f32, DistCosine>,
	query: &[f32],
	self_idx: usize,
	ef_search: usize,
) -> f32 {
	let neighbor_k = LOCAL_SCALING_K + 1;
	let neighbours =
		hnsw.search(query, neighbor_k, ef_search);

	let dists: Vec<f32> = neighbours
		.into_iter()
		.filter(|nbr| nbr.d_id != self_idx)
		.take(LOCAL_SCALING_K)
		.map(|nbr| nbr.distance)
		.collect();

	dists.last().copied().unwrap_or(DEFAULT_SIGMA)
}

/// Local Scaling adjusted distance.
/// LS(d) = 1 - exp(-d^2 / (sigma_x * sigma_y))
pub fn local_scaling_distance(
	raw_dist: f32,
	sigma_query: f32,
	sigma_candidate: f32,
) -> f32 {
	let denom = sigma_query * sigma_candidate;
	if denom < EPSILON {
		return DEFAULT_SIGMA;
	}
	1.0 - (-raw_dist * raw_dist / denom).exp()
}

/// Re-rank results using Local Scaling.
/// Computes sigma_query from the k-th raw result distance.
pub fn rerank_with_local_scaling(
	results: Vec<SearchResult>,
	sigma_values: &[f32],
	top_k: usize,
) -> Vec<SearchResult> {
	let sigma_qry = query_sigma(&results);

	let mut adjusted: Vec<(SearchResult, f32)> = results
		.into_iter()
		.map(|res| {
			let score = compute_ls_score(
				&res, sigma_qry, sigma_values,
			);
			(res, score)
		})
		.collect();

	adjusted.sort_by(|left, right| {
		left.1
			.partial_cmp(&right.1)
			.unwrap_or(std::cmp::Ordering::Equal)
	});

	adjusted
		.into_iter()
		.take(top_k)
		.map(|(mut res, score)| {
			res.distance = score;
			res
		})
		.collect()
}

/// Compute LS score for a single result
fn compute_ls_score(
	result: &SearchResult,
	sigma_qry: f32,
	sigma_values: &[f32],
) -> f32 {
	let idx = result.point.id as usize;
	let sigma_cand = sigma_values
		.get(idx)
		.copied()
		.unwrap_or(DEFAULT_SIGMA);
	local_scaling_distance(
		result.distance, sigma_qry, sigma_cand,
	)
}

/// Compute sigma for the query from raw result distances.
/// Uses the k-th closest result distance as sigma_query.
fn query_sigma(results: &[SearchResult]) -> f32 {
	let k_idx = LOCAL_SCALING_K.saturating_sub(1);
	results
		.get(k_idx)
		.map(|res| res.distance)
		.unwrap_or(DEFAULT_SIGMA)
}
