use std::collections::HashMap;
use std::path::PathBuf;

use rustean::indexer::{
	ByteSpan, CodeLocation, SearchHit,
	Symbol, SymbolKind,
};
use rustean::retrieval::daemon::protocol::QueryIntent;
use rustean::retrieval::hybrid::fusion::RankedItem;
use rustean::retrieval::hybrid::fusion_core::FusionParams;
use rustean::retrieval::hybrid::fusion_intent::{
	fuse_with_weights_and_intent, IntentContext,
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

/// Test fuse_with_weights_and_intent produces results
#[test]
fn test_fuse_with_intent_produces_results() {
	let intent = QueryIntent::FindDefinition;
	let rc: HashMap<String, usize> = HashMap::new();
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

	let params = FusionParams {
		keyword_weight: 0.5,
		semantic_weight: 0.5,
		query: "test",
	};
	let ctx = IntentContext {
		intent: &intent, ref_counts: &rc,
	};
	let results = fuse_with_weights_and_intent(
		keyword_results, semantic_results,
		&params, &ctx,
	);

	assert_eq!(results.len(), 2);
	assert!(results[0].score > 0.0);
}

/// Test fuse_with_weights_and_intent merges overlapping
#[test]
fn test_fuse_with_intent_overlap() {
	let intent = QueryIntent::Understand;
	let rc: HashMap<String, usize> = HashMap::new();
	let keyword_results = vec![RankedItem {
		item: create_search_hit("func_a", 10, 5.0),
		rank: 1,
		score: 5.0,
	}];
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
	let ctx = IntentContext {
		intent: &intent, ref_counts: &rc,
	};
	let results = fuse_with_weights_and_intent(
		keyword_results, semantic_results,
		&params, &ctx,
	);

	assert_eq!(results.len(), 1);
	assert!(results[0].keyword_rank.is_some());
	assert!(results[0].semantic_rank.is_some());
}

/// Test different intents produce different boosts
#[test]
fn test_fuse_with_intent_modify() {
	let intent = QueryIntent::Modify;
	let rc: HashMap<String, usize> = HashMap::new();
	let keyword_results = vec![RankedItem {
		item: create_search_hit("func_c", 30, 3.0),
		rank: 1,
		score: 3.0,
	}];

	let params = FusionParams {
		keyword_weight: 0.5,
		semantic_weight: 0.5,
		query: "modify test",
	};
	let ctx = IntentContext {
		intent: &intent, ref_counts: &rc,
	};
	let results = fuse_with_weights_and_intent(
		keyword_results, vec![],
		&params, &ctx,
	);

	assert_eq!(results.len(), 1);
	assert!(results[0].score > 0.0);
	assert_eq!(results[0].keyword_rank, Some(1));
}
