//! Intent-aware fusion functions for hybrid search
//!
//! Contains fuse_with_weights_and_intent and
//! process_*_with_intent functions that combine keyword
//! and semantic results using RRF with intent-aware boosting.

use std::collections::HashMap;

use crate::indexer::{CodeLocation, DocumentType, SearchHit, Symbol};
use crate::retrieval::daemon::protocol::QueryIntent;
use crate::retrieval::hybrid::config::HybridSearchConfig;
use crate::retrieval::hybrid::converters::parse_symbol_kind;
use crate::retrieval::hybrid::fusion::{rrf_score, RankedItem};
use crate::retrieval::hybrid::result::HybridSearchResult;
use crate::retrieval::hybrid::vector_store::SearchResult as VectorSearchResult;

/// Fuse results with intent-aware symbol kind boost.
/// Uses boost_factor_for_intent to adjust symbol
/// priority based on query intent.
pub fn fuse_with_weights_and_intent(
	config: &HybridSearchConfig,
	keyword_results: Vec<RankedItem<SearchHit>>,
	semantic_results: Vec<RankedItem<VectorSearchResult>>,
	keyword_weight: f32,
	semantic_weight: f32,
	query: &str,
	intent: &QueryIntent,
) -> Vec<HybridSearchResult> {
	let mut results_by_key: HashMap<String, HybridSearchResult> =
		HashMap::new();

	// process keyword results with intent-aware boost
	for ranked in keyword_results {
		let result = process_keyword_with_intent(
			config, &ranked, keyword_weight, query, intent,
		);
		let key = format!(
			"{}:{}",
			ranked.item.symbol.name, ranked.item.symbol.location.line
		);

		results_by_key
			.entry(key)
			.and_modify(|r| {
				r.rrf_score += result.rrf_score;
				r.keyword_rank = result.keyword_rank;
				r.keyword_score = result.keyword_score;
			})
			.or_insert(result);
	}

	// process semantic results with intent-aware boost
	for ranked in semantic_results {
		let (key, result) = process_semantic_with_intent(
			config, &ranked, semantic_weight, query, intent,
		);

		results_by_key
			.entry(key)
			.and_modify(|r| {
				r.rrf_score += result.rrf_score;
				r.semantic_rank = result.semantic_rank;
				r.semantic_distance = result.semantic_distance;
			})
			.or_insert(result);
	}

	// sort by RRF score descending
	let mut results: Vec<_> = results_by_key.into_values().collect();
	results.sort_by(|a, b| b.rrf_score.partial_cmp(&a.rrf_score).unwrap());

	results
}

/// Process a single keyword result with intent-aware boost
pub fn process_keyword_with_intent(
	config: &HybridSearchConfig,
	ranked: &RankedItem<SearchHit>,
	keyword_weight: f32,
	query: &str,
	intent: &QueryIntent,
) -> HybridSearchResult {
	let doc_type =
		DocumentType::from_path(&ranked.item.symbol.location.file);
	let doc_query_boost = doc_type.boost_factor_for_query(query);
	let doc_intent_boost = doc_type.boost_factor_for_intent(intent);
	let kind_boost =
		ranked.item.symbol.kind.boost_factor_for_intent(intent);
	let combined_boost =
		doc_query_boost * doc_intent_boost * kind_boost;

	let base = rrf_score(ranked.rank, config.rrf_k);
	let rrf = base * keyword_weight * combined_boost;

	HybridSearchResult {
		symbol: ranked.item.symbol.clone(),
		rrf_score: rrf,
		keyword_rank: Some(ranked.rank),
		semantic_rank: None,
		keyword_score: Some(ranked.score),
		semantic_distance: None,
		rerank_score: None,
	}
}

/// Process a single semantic result with intent-aware boost
pub fn process_semantic_with_intent(
	config: &HybridSearchConfig,
	ranked: &RankedItem<VectorSearchResult>,
	semantic_weight: f32,
	query: &str,
	intent: &QueryIntent,
) -> (String, HybridSearchResult) {
	let doc_type =
		DocumentType::from_path(&ranked.item.point.file_path);
	let doc_query_boost = doc_type.boost_factor_for_query(query);
	let doc_intent_boost = doc_type.boost_factor_for_intent(intent);
	let symbol_kind =
		parse_symbol_kind(&ranked.item.point.symbol_kind);
	let kind_boost = symbol_kind.boost_factor_for_intent(intent);
	let combined_boost =
		doc_query_boost * doc_intent_boost * kind_boost;

	let base = rrf_score(ranked.rank, config.rrf_k);
	let rrf = base * semantic_weight * combined_boost;

	let key = format!(
		"{}:{}",
		ranked.item.point.symbol_name,
		ranked.item.point.line,
	);

	let point = &ranked.item.point;
	let symbol = Symbol::new(
		point.symbol_name.clone(),
		symbol_kind,
		CodeLocation::new(point.file_path.clone(), point.line, 1, 0, 0),
	);

	let result = HybridSearchResult {
		symbol,
		rrf_score: rrf,
		keyword_rank: None,
		semantic_rank: Some(ranked.rank),
		keyword_score: None,
		semantic_distance: Some(ranked.score),
		rerank_score: None,
	};

	(key, result)
}

