use std::path::PathBuf;

use ch_cli::indexer::{CodeLocation, Symbol, SymbolKind};
use ch_cli::retrieval::hybrid::result::HybridSearchResult;
use ch_cli::retrieval::hybrid::triple::{
	TripleHybridResults, TripleHybridStats,
};

/// Test TripleHybridStats total calculation
#[test]
fn test_triple_hybrid_stats_total() {
	let stats = TripleHybridStats {
		code_keyword_count: 10,
		code_vector_count: 10,
		doc_keyword_count: 5,
		doc_vector_count: 5,
		notes_keyword_count: 3,
		notes_vector_count: 3,
	};

	// Total = keyword counts (10 + 5 + 3 = 18)
	assert_eq!(stats.total(), 18);
}

/// Test TripleHybridResults default
#[test]
fn test_triple_hybrid_results_default() {
	let results = TripleHybridResults::default();

	assert!(results.code_results.is_empty());
	assert!(results.doc_results.is_empty());
	assert!(results.notes_results.is_empty());
}

/// Test default stats is zero
#[test]
fn test_default_stats_zero() {
	let stats = TripleHybridStats::default();
	assert_eq!(stats.total(), 0);
}

/// Helper for mock HybridSearchResult with RRF score
fn mock_result(rrf_score: f32) -> HybridSearchResult {
	HybridSearchResult {
		symbol: Symbol::new(
			format!(
				"test_{}",
				(rrf_score * 1000.0) as i32
			),
			SymbolKind::Function,
			CodeLocation::new(
				PathBuf::from("test.rs"),
				1, 0, 0, 10,
			),
		),
		rrf_score,
		keyword_rank: None,
		semantic_rank: None,
		keyword_score: None,
		semantic_distance: None,
		rerank_score: None,
	}
}

/// Test: Results above threshold are kept
#[test]
fn test_filter_keeps_high_scores() {
	let results = vec![
		mock_result(0.025),
		mock_result(0.020),
		mock_result(0.010),
		mock_result(0.005),
	];

	let threshold = 0.015_f32;
	let min_results = 1_usize;

	let mut filtered: Vec<HybridSearchResult> =
		Vec::new();
	for result in results.into_iter() {
		if filtered.len() < min_results {
			filtered.push(result);
			continue;
		}
		if filtered.len() >= 10 {
			break;
		}
		if result.rrf_score >= threshold {
			filtered.push(result);
		}
	}

	// Should keep 2 (scores 0.025 and 0.020)
	assert_eq!(filtered.len(), 2);
}
