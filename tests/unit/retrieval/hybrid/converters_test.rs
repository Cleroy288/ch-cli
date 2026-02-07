use std::path::PathBuf;

use ch_cli::indexer::{CodeLocation, SearchHit, Symbol, SymbolKind};
use ch_cli::retrieval::hybrid::converters::{
	to_keyword_ranked, to_semantic_ranked,
};
use ch_cli::retrieval::hybrid::vector_store::{
	SearchResult as VectorSearchResult, VectorPoint,
};

/// Create a test SearchHit with given name and score
fn make_search_hit(name: &str, score: f32) -> SearchHit {
	let location =
		CodeLocation::new(PathBuf::from("test.rs"), 1, 0, 0, 0);
	let symbol =
		Symbol::new(name.to_string(), SymbolKind::Function, location);
	SearchHit { symbol, score }
}

/// Create a test VectorSearchResult with given id and distance
fn make_vector_result(
	id: u64,
	distance: f32,
) -> VectorSearchResult {
	let point = VectorPoint {
		id,
		vector: vec![1.0, 0.0, 0.0],
		file_path: PathBuf::from("test.rs"),
		line: 1,
		symbol_name: format!("sym_{}", id),
		symbol_kind: "function".to_string(),
	};
	VectorSearchResult { point, distance }
}

/// Test to_keyword_ranked converts SearchHits with correct ranks and scores
#[test]
fn test_to_keyword_ranked() {
	let hits = vec![
		make_search_hit("foo", 1.0),
		make_search_hit("bar", 0.5),
	]; // input hits with decreasing scores

	let ranked = to_keyword_ranked(hits); // convert to ranked items

	assert_eq!(ranked.len(), 2);
	// first item gets rank 1
	assert_eq!(ranked[0].rank, 1);
	assert!((ranked[0].score - 1.0).abs() < 1e-6);
	assert_eq!(ranked[0].item.symbol.name, "foo");
	// second item gets rank 2
	assert_eq!(ranked[1].rank, 2);
	assert!((ranked[1].score - 0.5).abs() < 1e-6);
	assert_eq!(ranked[1].item.symbol.name, "bar");
}

/// Test to_semantic_ranked converts VectorSearchResults
/// with correct ranks and distances
#[test]
fn test_to_semantic_ranked() {
	let hits = vec![
		make_vector_result(0, 0.1),
		make_vector_result(1, 0.3),
		make_vector_result(2, 0.7),
	]; // input vector results with increasing distance

	let ranked = to_semantic_ranked(hits); // convert to ranked items

	assert_eq!(ranked.len(), 3);
	// ranks are 1-indexed sequential
	assert_eq!(ranked[0].rank, 1);
	assert_eq!(ranked[1].rank, 2);
	assert_eq!(ranked[2].rank, 3);
	// scores come from distance field
	assert!((ranked[0].score - 0.1).abs() < 1e-6);
	assert!((ranked[1].score - 0.3).abs() < 1e-6);
	assert!((ranked[2].score - 0.7).abs() < 1e-6);
	// point IDs preserved
	assert_eq!(ranked[0].item.point.id, 0);
	assert_eq!(ranked[1].item.point.id, 1);
	assert_eq!(ranked[2].item.point.id, 2);
}
