use std::path::PathBuf;

use rustean::indexer::{
	ByteSpan, CodeLocation, SearchHit,
	Symbol, SymbolKind,
};
use rustean::retrieval::hybrid::fusion::RankedItem;
use rustean::retrieval::hybrid::normalize::{
	min_max_normalize, normalize_keyword_scores,
	normalize_semantic_scores, score_bounds,
};
use rustean::retrieval::hybrid::vector_store::{
	PointMeta, SearchResult as VectorSearchResult,
};

/// Test min_max_normalize returns 0.5 when max == min
#[test]
fn test_normalize_single_score_returns_half() {
	let result = min_max_normalize(5.0, 5.0, 5.0);
	assert!((result - 0.5).abs() < 1e-6);
}

/// Test min_max_normalize returns boundary values
#[test]
fn test_normalize_boundary_values() {
	let min_val = min_max_normalize(0.0, 0.0, 10.0);
	let max_val = min_max_normalize(10.0, 0.0, 10.0);
	let mid_val = min_max_normalize(5.0, 0.0, 10.0);

	assert!((min_val - 0.0).abs() < 1e-6);
	assert!((max_val - 1.0).abs() < 1e-6);
	assert!((mid_val - 0.5).abs() < 1e-6);
}

/// Test score_bounds with empty input
#[test]
fn test_score_bounds_empty() {
	let (min, max) = score_bounds(&[]);
	assert!((min - 0.0).abs() < 1e-6);
	assert!((max - 0.0).abs() < 1e-6);
}

/// Test score_bounds with multiple scores
#[test]
fn test_score_bounds_multiple() {
	let scores = vec![3.0, 1.0, 7.0, 2.0, 5.0];
	let (min, max) = score_bounds(&scores);

	assert!((min - 1.0).abs() < 1e-6);
	assert!((max - 7.0).abs() < 1e-6);
}

/// Helper: create keyword ranked item
fn kw_ranked(
	score: f32, rank: usize,
) -> RankedItem<SearchHit> {
	let sym = Symbol::new(
		format!("sym_{}", rank),
		SymbolKind::Function,
		CodeLocation::new(
			PathBuf::from("t.rs"),
			rank, 1, ByteSpan::ZERO,
		),
	);
	RankedItem {
		item: SearchHit { symbol: sym, score },
		rank,
		score,
	}
}

/// Test normalize_keyword_scores empty returns empty
#[test]
fn test_normalize_keyword_empty() {
	let result = normalize_keyword_scores(&[]);
	assert!(result.is_empty());
}

/// Test normalize_keyword_scores preserves order
#[test]
fn test_normalize_keyword_preserves_order() {
	let items = vec![
		kw_ranked(10.0, 1),
		kw_ranked(5.0, 2),
		kw_ranked(1.0, 3),
	];
	let norms = normalize_keyword_scores(&items);

	assert_eq!(norms.len(), 3);
	// Highest BM25 → 1.0, lowest → 0.0
	assert!((norms[0] - 1.0).abs() < 1e-6);
	assert!((norms[2] - 0.0).abs() < 1e-6);
	assert!(norms[0] > norms[1]);
	assert!(norms[1] > norms[2]);
}

/// Helper: create semantic ranked item
fn sem_ranked(
	dist: f32, rank: usize,
) -> RankedItem<VectorSearchResult> {
	let point = PointMeta {
		id: 0,
		file_path: PathBuf::from("t.rs"),
		line: rank,
		symbol_name: format!("sym_{}", rank),
		symbol_kind: "function".to_string(),
	};
	RankedItem {
		item: VectorSearchResult {
			point,
			distance: dist,
		},
		rank,
		score: dist,
	}
}

/// Test normalize_semantic_scores empty returns empty
#[test]
fn test_normalize_semantic_empty() {
	let result = normalize_semantic_scores(&[]);
	assert!(result.is_empty());
}

/// Test normalize_semantic_scores inverts distances
#[test]
fn test_normalize_semantic_inverts_distance() {
	let items = vec![
		sem_ranked(0.1, 1), // closest = best
		sem_ranked(0.5, 2),
		sem_ranked(0.9, 3), // farthest = worst
	];
	let norms = normalize_semantic_scores(&items);

	assert_eq!(norms.len(), 3);
	// Lowest distance → 1.0, highest → 0.0
	assert!((norms[0] - 1.0).abs() < 1e-6);
	assert!((norms[2] - 0.0).abs() < 1e-6);
	assert!(norms[0] > norms[1]);
	assert!(norms[1] > norms[2]);
}
