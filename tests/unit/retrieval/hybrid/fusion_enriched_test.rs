use std::path::PathBuf;
use std::sync::Arc;

use rustean::indexer::{
	ByteSpan, CodeLocation, Symbol, SymbolKind,
};
use rustean::retrieval::hybrid::fusion_enriched::{
	boost_with_enriched, build_point_key,
	build_result_key,
};
use rustean::retrieval::hybrid::result::HybridSearchResult;
use rustean::retrieval::hybrid::vector_store::{
	PointMeta, SearchResult as VectorSearchResult,
};

/// Helper: create a HybridSearchResult for testing
fn make_result(
	name: &str, line: usize, score: f32,
) -> HybridSearchResult {
	let loc = CodeLocation::new(
		PathBuf::from("src/test.rs"),
		line, 1, ByteSpan::ZERO,
	);
	let sym = Symbol::new(
		name.to_string(),
		SymbolKind::Function,
		loc,
	);
	HybridSearchResult {
		symbol: Arc::new(sym),
		score,
		keyword_rank: None,
		semantic_rank: None,
		keyword_score: None,
		semantic_distance: None,
		rerank_score: None,
	}
}

/// Helper: create a VectorSearchResult for testing
fn make_vector_hit(
	name: &str, line: usize, distance: f32,
) -> VectorSearchResult {
	let point = PointMeta {
		id: 0,
		file_path: PathBuf::from("src/test.rs"),
		line,
		symbol_name: name.to_string(),
		symbol_kind: "function".to_string(),
	};
	VectorSearchResult { point, distance }
}

#[test]
fn boost_empty_hits_no_change() {
	// Arrange
	let mut results =
		vec![make_result("foo", 10, 1.0)];
	let original_score = results[0].score;

	// Act
	boost_with_enriched(&mut results, &[]);

	// Assert
	assert_eq!(results[0].score, original_score);
}

#[test]
fn boost_matching_key_increases_score() {
	// Arrange
	let mut results =
		vec![make_result("foo", 10, 1.0)];
	let enriched =
		vec![make_vector_hit("foo", 10, 0.1)];

	// Act
	boost_with_enriched(&mut results, &enriched);

	// Assert — score should increase from enriched boost
	assert!(results[0].score > 1.0);
}

#[test]
fn boost_non_matching_key_unchanged() {
	// Arrange
	let mut results =
		vec![make_result("foo", 10, 1.0)];
	let enriched =
		vec![make_vector_hit("bar", 20, 0.1)];

	// Act
	boost_with_enriched(&mut results, &enriched);

	// Assert — no matching key, score unchanged
	assert_eq!(results[0].score, 1.0);
}

#[test]
fn build_result_key_format() {
	// Arrange
	let result = make_result("my_func", 42, 1.0);

	// Act
	let key = build_result_key(&result);

	// Assert
	assert_eq!(key, "src/test.rs:42:my_func");
}

#[test]
fn build_point_key_format() {
	// Arrange
	let point = PointMeta {
		id: 0,
		file_path: PathBuf::from("src/test.rs"),
		line: 42,
		symbol_name: "my_func".to_string(),
		symbol_kind: "function".to_string(),
	};

	// Act
	let key = build_point_key(&point);

	// Assert
	assert_eq!(key, "src/test.rs:42:my_func");
}
