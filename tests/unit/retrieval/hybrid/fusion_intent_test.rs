use std::path::PathBuf;

use ch_cli::indexer::{CodeLocation, SearchHit, Symbol, SymbolKind};
use ch_cli::retrieval::daemon::protocol::QueryIntent;
use ch_cli::retrieval::hybrid::config::HybridSearchConfig;
use ch_cli::retrieval::hybrid::fusion::RankedItem;
use ch_cli::retrieval::hybrid::fusion_intent::{
	fuse_with_weights_and_intent,
	process_keyword_with_intent,
	process_semantic_with_intent,
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

/// Test fuse_with_weights_and_intent combines results with intent
#[test]
fn test_fuse_with_weights_and_intent() {
	let config = HybridSearchConfig::default();
	let intent = QueryIntent::FindDefinition;
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

	let results = fuse_with_weights_and_intent(
		&config,
		keyword_results,
		semantic_results,
		1.0,
		1.0,
		"test",
		&intent,
	);

	assert_eq!(results.len(), 2);
	assert!(results[0].rrf_score > 0.0);
}

/// Test fuse_with_weights_and_intent merges overlapping results
#[test]
fn test_fuse_with_weights_and_intent_overlap() {
	let config = HybridSearchConfig::default();
	let intent = QueryIntent::Understand;
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

	let results = fuse_with_weights_and_intent(
		&config,
		keyword_results,
		semantic_results,
		1.0,
		1.0,
		"test",
		&intent,
	);

	assert_eq!(results.len(), 1);
	assert!(results[0].keyword_rank.is_some());
	assert!(results[0].semantic_rank.is_some());
}

/// Test process_keyword_with_intent applies intent-aware boost
#[test]
fn test_process_keyword_with_intent() {
	let config = HybridSearchConfig::default();
	let intent = QueryIntent::FindUsages;
	let ranked = RankedItem {
		item: create_search_hit("func_a", 10, 1.0),
		rank: 1,
		score: 1.0,
	};

	let result = process_keyword_with_intent(
		&config, &ranked, 1.0, "test", &intent,
	);

	assert_eq!(result.symbol.name, "func_a");
	assert!(result.rrf_score > 0.0);
	assert_eq!(result.keyword_rank, Some(1));
	assert_eq!(result.semantic_rank, None);
}

/// Test process_semantic_with_intent applies intent-aware boost
#[test]
fn test_process_semantic_with_intent() {
	let config = HybridSearchConfig::default();
	let intent = QueryIntent::Debug;
	let ranked = RankedItem {
		item: create_vector_result("func_b", 20, 0.1),
		rank: 1,
		score: 0.1,
	};

	let (key, result) = process_semantic_with_intent(
		&config, &ranked, 1.0, "test", &intent,
	);

	assert_eq!(key, "func_b:20");
	assert_eq!(result.symbol.name, "func_b");
	assert!(result.rrf_score > 0.0);
	assert_eq!(result.keyword_rank, None);
	assert_eq!(result.semantic_rank, Some(1));
}

/// Test process_keyword_with_intent with different intents
#[test]
fn test_process_keyword_with_intent_modify() {
	let config = HybridSearchConfig::default();
	let intent = QueryIntent::Modify;
	let ranked = RankedItem {
		item: create_search_hit("func_c", 30, 0.8),
		rank: 2,
		score: 0.8,
	};

	let result = process_keyword_with_intent(
		&config, &ranked, 1.5, "modify test", &intent,
	);

	assert_eq!(result.symbol.name, "func_c");
	assert!(result.rrf_score > 0.0);
	assert_eq!(result.keyword_rank, Some(2));
	assert_eq!(result.keyword_score, Some(0.8));
}

/// Test process_semantic_with_intent with different intents
#[test]
fn test_process_semantic_with_intent_search() {
	let config = HybridSearchConfig::default();
	let intent = QueryIntent::Search;
	let ranked = RankedItem {
		item: create_vector_result("func_d", 40, 0.2),
		rank: 3,
		score: 0.2,
	};

	let (key, result) = process_semantic_with_intent(
		&config, &ranked, 0.5, "search test", &intent,
	);

	assert_eq!(key, "func_d:40");
	assert_eq!(result.symbol.name, "func_d");
	assert_eq!(result.semantic_rank, Some(3));
	assert_eq!(result.semantic_distance, Some(0.2));
}
