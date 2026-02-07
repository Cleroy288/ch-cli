use std::path::PathBuf;

use ch_cli::indexer::{CodeLocation, SearchHit, Symbol, SymbolKind};
use ch_cli::retrieval::hybrid::config::HybridSearchConfig;
use ch_cli::retrieval::hybrid::fusion::RankedItem;
use ch_cli::retrieval::hybrid::fusion_core::{
	fuse_search_results, fuse_with_weights,
};
use ch_cli::retrieval::hybrid::vector_store::{
	SearchResult as VectorSearchResult, VectorPoint,
};

/// Helper to create a test SearchHit
fn create_search_hit(
	name: &str,
	line: usize,
	score: f32,
) -> SearchHit {
	let location =
		CodeLocation::new(PathBuf::from("test.rs"), line, 1, 0, 0);
	let symbol = Symbol::new(
		name.to_string(),
		SymbolKind::Function,
		location,
	);
	SearchHit { symbol, score }
}

/// Helper to create a test VectorSearchResult
fn create_vector_result(
	name: &str,
	line: usize,
	distance: f32,
) -> VectorSearchResult {
	let point = VectorPoint {
		id: 0,
		vector: vec![0.0, 0.0, 0.0],
		file_path: PathBuf::from("test.rs"),
		line,
		symbol_name: name.to_string(),
		symbol_kind: "function".to_string(),
	};
	VectorSearchResult { point, distance }
}

/// Test fuse_search_results combines keyword and semantic results
#[test]
fn test_fuse_search_results() {
	let config = HybridSearchConfig::default();
	let keyword_results = vec![RankedItem {
		item: create_search_hit("func_a", 10, 1.0),
		rank: 1,
		score: 1.0,
	}];
	let semantic_results = vec![RankedItem {
		item: create_vector_result("func_b", 20, 0.1),
		rank: 1,
		score: 0.1,
	}];

	let results = fuse_search_results(
		&config, keyword_results, semantic_results, "test",
	);

	assert_eq!(results.len(), 2);
	assert!(results[0].rrf_score > 0.0);
}

/// Test fuse_with_weights applies custom weights to results
#[test]
fn test_fuse_with_weights() {
	let config = HybridSearchConfig::default();
	let keyword_results = vec![RankedItem {
		item: create_search_hit("func_a", 10, 1.0),
		rank: 1,
		score: 1.0,
	}];
	let semantic_results = vec![RankedItem {
		item: create_vector_result("func_b", 20, 0.1),
		rank: 1,
		score: 0.1,
	}];

	let results = fuse_with_weights(
		&config, keyword_results, semantic_results, 2.0, 1.0, "test",
	);

	assert_eq!(results.len(), 2);
	assert!(
		results[0].keyword_rank.is_some()
			|| results[0].semantic_rank.is_some()
	);
}

/// Test fuse_with_weights merges overlapping results
#[test]
fn test_fuse_with_weights_overlap() {
	let config = HybridSearchConfig::default();
	let keyword_results = vec![RankedItem {
		item: create_search_hit("func_a", 10, 1.0),
		rank: 1,
		score: 1.0,
	}];
	let semantic_results = vec![RankedItem {
		item: create_vector_result("func_a", 10, 0.1),
		rank: 1,
		score: 0.1,
	}];

	let results = fuse_with_weights(
		&config, keyword_results, semantic_results,
		1.0, 1.0, "test",
	);

	assert_eq!(results.len(), 1);
	assert!(results[0].keyword_rank.is_some());
	assert!(results[0].semantic_rank.is_some());
}

/// Test empty keyword results returns only semantic
#[test]
fn test_fuse_with_weights_empty_keyword() {
	let config = HybridSearchConfig::default();
	let keyword_results = vec![];
	let semantic_results = vec![RankedItem {
		item: create_vector_result("func_b", 20, 0.1),
		rank: 1,
		score: 0.1,
	}];

	let results = fuse_with_weights(
		&config, keyword_results, semantic_results, 1.0, 1.0, "test",
	);

	assert_eq!(results.len(), 1);
	assert!(results[0].semantic_rank.is_some());
	assert!(results[0].keyword_rank.is_none());
}

/// Test empty semantic results returns only keyword
#[test]
fn test_fuse_with_weights_empty_semantic() {
	let config = HybridSearchConfig::default();
	let keyword_results = vec![RankedItem {
		item: create_search_hit("func_a", 10, 1.0),
		rank: 1,
		score: 1.0,
	}];
	let semantic_results = vec![];

	let results = fuse_with_weights(
		&config, keyword_results, semantic_results, 1.0, 1.0, "test",
	);

	assert_eq!(results.len(), 1);
	assert!(results[0].keyword_rank.is_some());
	assert!(results[0].semantic_rank.is_none());
}
