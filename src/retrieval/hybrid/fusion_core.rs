//! Core fusion functions for hybrid search
//!
//! Combines keyword and semantic results using min-max
//! normalized Convex Combination (CC).

use std::collections::HashMap;

use crate::indexer::SearchHit;
use crate::retrieval::hybrid::config::HybridSearchConfig;
use crate::retrieval::hybrid::fusion::RankedItem;
use crate::retrieval::hybrid::fusion_processors::{
	process_keyword_results, process_semantic_results,
	ProcessorParams,
};
use crate::retrieval::hybrid::normalize::{
	normalize_keyword_scores, normalize_semantic_scores,
};
use crate::retrieval::hybrid::result::HybridSearchResult;
use crate::retrieval::hybrid::vector_store::{
	SearchResult as VectorSearchResult,
};

/// Bundled fusion weight and query parameters
pub struct FusionParams<'param> {
	/// weight for keyword (BM25) results
	pub keyword_weight: f32,
	/// weight for semantic results
	pub semantic_weight: f32,
	/// the search query string
	pub query: &'param str,
}

/// Fuse keyword and semantic results with default weights
pub fn fuse_search_results(
	config: &HybridSearchConfig,
	keyword_results: Vec<RankedItem<SearchHit>>,
	semantic_results: Vec<RankedItem<VectorSearchResult>>,
	query: &str,
) -> Vec<HybridSearchResult> {
	let params = FusionParams {
		keyword_weight: config.keyword_weight,
		semantic_weight: config.semantic_weight,
		query,
	};
	fuse_with_weights(
		keyword_results, semantic_results, &params,
	)
}

/// Fuse keyword and semantic results with custom weights
/// using min-max normalized Convex Combination
pub fn fuse_with_weights<'prm>(
	keyword_results: Vec<RankedItem<SearchHit>>,
	semantic_results: Vec<RankedItem<VectorSearchResult>>,
	params: &FusionParams<'prm>,
) -> Vec<HybridSearchResult> {
	let mut results_map: HashMap<String, HybridSearchResult> =
		HashMap::new();
	let empty_rc: HashMap<String, usize> = HashMap::new();

	merge_keyword_side(
		&mut results_map,
		keyword_results,
		params,
		&empty_rc,
	);
	merge_semantic_side(
		&mut results_map,
		semantic_results,
		params,
		&empty_rc,
	);
	sort_by_score(results_map)
}

/// Process and merge keyword results into the map
fn merge_keyword_side<'prm>(
	results_map: &mut HashMap<String, HybridSearchResult>,
	keyword_results: Vec<RankedItem<SearchHit>>,
	params: &FusionParams<'prm>,
	empty_rc: &HashMap<String, usize>,
) {
	let norms =
		normalize_keyword_scores(&keyword_results);
	let proc_params = ProcessorParams {
		norm_scores: &norms,
		weight: params.keyword_weight,
		query: params.query,
		ref_counts: empty_rc,
	};
	process_keyword_results(
		results_map, keyword_results, &proc_params,
	);
}

/// Process and merge semantic results into the map
fn merge_semantic_side<'prm>(
	results_map: &mut HashMap<String, HybridSearchResult>,
	semantic_results: Vec<RankedItem<VectorSearchResult>>,
	params: &FusionParams<'prm>,
	empty_rc: &HashMap<String, usize>,
) {
	let norms =
		normalize_semantic_scores(&semantic_results);
	let proc_params = ProcessorParams {
		norm_scores: &norms,
		weight: params.semantic_weight,
		query: params.query,
		ref_counts: empty_rc,
	};
	process_semantic_results(
		results_map, semantic_results, &proc_params,
	);
}

/// Collect results and sort by score descending
fn sort_by_score(
	results_map: HashMap<String, HybridSearchResult>,
) -> Vec<HybridSearchResult> {
	let mut results: Vec<_> =
		results_map.into_values().collect();
	results.sort_by(|left, right| {
		right.score.partial_cmp(&left.score).unwrap()
	});
	results
}
