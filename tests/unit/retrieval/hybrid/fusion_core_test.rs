use std::path::PathBuf;

use rustean::indexer::{
	ByteSpan, CodeLocation, SearchHit,
	Symbol, SymbolKind,
};
use rustean::retrieval::hybrid::config::HybridSearchConfig;
use rustean::retrieval::hybrid::fusion::RankedItem;
use rustean::retrieval::hybrid::fusion_core::{
	fuse_search_results, fuse_with_weights, FusionParams,
};
use rustean::retrieval::hybrid::vector_store::{
	PointMeta, SearchResult as VectorSearchResult,
};

/// Helper to create a test SearchHit
fn create_search_hit(
	name: &str,
	line: usize,
	score: f32,
) -> SearchHit {
	let location =
		CodeLocation::new(PathBuf::from("test.rs"), line, 1, ByteSpan::ZERO);
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
	let point = PointMeta {
		id: 0,
		file_path: PathBuf::from("test.rs"),
		line,
		symbol_name: name.to_string(),
		symbol_kind: "function".to_string(),
	};
	VectorSearchResult { point, distance }
}

/// Test fuse_search_results produces non-zero scores
#[test]
fn test_fuse_search_results() {
	let config = HybridSearchConfig::default();
	let keyword_results = vec![RankedItem {
		item: create_search_hit("func_a", 10, 5.0),
		rank: 1,
		score: 5.0,
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
	// Both should have positive scores (CC normalized)
	assert!(results[0].score > 0.0);
	assert!(results[1].score > 0.0);
}

/// Test CC overlap: item in both channels gets higher score
#[test]
fn test_fuse_with_weights_overlap() {
	let keyword_results = vec![
		RankedItem {
			item: create_search_hit("func_a", 10, 5.0),
			rank: 1,
			score: 5.0,
		},
		RankedItem {
			item: create_search_hit("func_b", 20, 3.0),
			rank: 2,
			score: 3.0,
		},
	];
	// func_a also appears in semantic with low distance
	let semantic_results = vec![RankedItem {
		item: create_vector_result("func_a", 10, 0.1),
		rank: 1,
		score: 0.1,
	}];

	let params = FusionParams {
		keyword_weight: 0.5,
		semantic_weight: 0.5,
		query: "test",
	};
	let results = fuse_with_weights(
		keyword_results, semantic_results, &params,
	);

	// func_a appears in both — should be ranked first
	assert_eq!(results[0].symbol.name, "func_a");
	assert!(results[0].keyword_rank.is_some());
	assert!(results[0].semantic_rank.is_some());
}

/// Test empty keyword results returns only semantic
#[test]
fn test_fuse_empty_keyword() {
	let semantic_results = vec![RankedItem {
		item: create_vector_result("func_b", 20, 0.1),
		rank: 1,
		score: 0.1,
	}];

	let params = FusionParams {
		keyword_weight: 0.5,
		semantic_weight: 0.5,
		query: "test",
	};
	let results = fuse_with_weights(
		vec![], semantic_results, &params,
	);

	assert_eq!(results.len(), 1);
	assert!(results[0].semantic_rank.is_some());
	assert!(results[0].keyword_rank.is_none());
}

/// Test empty semantic results returns only keyword
#[test]
fn test_fuse_empty_semantic() {
	let keyword_results = vec![RankedItem {
		item: create_search_hit("func_a", 10, 5.0),
		rank: 1,
		score: 5.0,
	}];

	let params = FusionParams {
		keyword_weight: 0.5,
		semantic_weight: 0.5,
		query: "test",
	};
	let results = fuse_with_weights(
		keyword_results, vec![], &params,
	);

	assert_eq!(results.len(), 1);
	assert!(results[0].keyword_rank.is_some());
	assert!(results[0].semantic_rank.is_none());
}
