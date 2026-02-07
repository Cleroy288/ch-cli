//! Core fusion functions for hybrid search
//!
//! Contains fuse_search_results and fuse_with_weights that
//! combine keyword and semantic results using RRF.

use std::collections::HashMap;

use crate::indexer::SearchHit;
use crate::retrieval::hybrid::config::HybridSearchConfig;
use crate::retrieval::hybrid::fusion::RankedItem;
use crate::retrieval::hybrid::fusion_processors::{
	process_keyword_results, process_semantic_results,
};
use crate::retrieval::hybrid::result::HybridSearchResult;
use crate::retrieval::hybrid::vector_store::{
	SearchResult as VectorSearchResult,
};

/// Fuse keyword and semantic results with default config weights
pub fn fuse_search_results(
	config: &HybridSearchConfig,
	keyword_results: Vec<RankedItem<SearchHit>>,
	semantic_results: Vec<RankedItem<VectorSearchResult>>,
	query: &str,
) -> Vec<HybridSearchResult> {
	fuse_with_weights(
		config,
		keyword_results,
		semantic_results,
		config.keyword_weight,
		config.semantic_weight,
		query,
	)
}

/// Fuse keyword and semantic results with custom weights
/// Uses query-aware boost to reduce doc priority for code queries
pub fn fuse_with_weights(
	config: &HybridSearchConfig,
	keyword_results: Vec<RankedItem<SearchHit>>,
	semantic_results: Vec<RankedItem<VectorSearchResult>>,
	keyword_weight: f32,
	semantic_weight: f32,
	query: &str,
) -> Vec<HybridSearchResult> {
	let mut results_by_key: HashMap<String, HybridSearchResult> =
		HashMap::new();

	// process keyword results
	process_keyword_results(
		config,
		&mut results_by_key,
		keyword_results,
		keyword_weight,
		query,
	);

	// process semantic results
	process_semantic_results(
		config,
		&mut results_by_key,
		semantic_results,
		semantic_weight,
		query,
	);

	// sort by RRF score
	let mut results: Vec<_> = results_by_key.into_values().collect();
	results.sort_by(|a, b| {
		b.rrf_score.partial_cmp(&a.rrf_score).unwrap()
	});

	results
}

