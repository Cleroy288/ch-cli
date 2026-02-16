use rustean::retrieval::hybrid::vector_store_csls::{
	local_scaling_distance, rerank_with_local_scaling,
};
use rustean::retrieval::hybrid::vector_store::{
	PointMeta, SearchResult,
};
use std::path::PathBuf;

/// Helper: create a search result with given id and distance
fn make_result(id: u64, distance: f32) -> SearchResult {
	SearchResult {
		point: PointMeta {
			id,
			file_path: PathBuf::from("test.rs"),
			line: 1,
			symbol_name: format!("sym_{}", id),
			symbol_kind: "function".to_string(),
		},
		distance,
	}
}

#[test]
fn local_scaling_distance_zero_dist_returns_zero() {
	// d=0 → LS = 1 - exp(0) = 0
	let ls = local_scaling_distance(0.0, 0.5, 0.5);
	assert!((ls - 0.0).abs() < 1e-6);
}

#[test]
fn local_scaling_distance_hub_penalized() {
	// Hub has small sigma → larger LS distance
	let hub = local_scaling_distance(0.3, 0.5, 0.1);
	// Non-hub has large sigma → smaller LS distance
	let non_hub = local_scaling_distance(0.3, 0.5, 0.8);
	assert!(hub > non_hub);
}

#[test]
fn local_scaling_distance_large_dist_near_one() {
	// Large distance → LS ≈ 1.0
	let ls = local_scaling_distance(2.0, 0.3, 0.3);
	assert!(ls > 0.99);
}

#[test]
fn rerank_with_local_scaling_reorders() {
	let results = vec![
		make_result(0, 0.2), // hub (small sigma)
		make_result(1, 0.25), // non-hub (large sigma)
		make_result(2, 0.3),
	];
	// id=0 is a hub (sigma=0.05), id=1 is not (sigma=0.8)
	let sigmas = vec![0.05, 0.8, 0.3];

	let reranked =
		rerank_with_local_scaling(results, &sigmas, 2);

	assert_eq!(reranked.len(), 2);
	// id=1 should rank higher (non-hub gets lower LS dist)
	assert_eq!(reranked[0].point.id, 1);
}

#[test]
fn rerank_with_local_scaling_empty_input() {
	let results: Vec<SearchResult> = vec![];
	let sigmas: Vec<f32> = vec![];

	let reranked =
		rerank_with_local_scaling(results, &sigmas, 5);
	assert!(reranked.is_empty());
}
