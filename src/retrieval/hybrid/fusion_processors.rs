//! Fusion Result Processors
//!
//! Helper functions for processing keyword and semantic results
//! into the RRF fusion map.

use std::collections::HashMap;

use crate::indexer::{DocumentType, SearchHit, Symbol};
use crate::retrieval::hybrid::config::HybridSearchConfig;
use crate::retrieval::hybrid::converters::parse_symbol_kind;
use crate::retrieval::hybrid::fusion::{rrf_score, RankedItem};
use crate::retrieval::hybrid::result::HybridSearchResult;
use crate::retrieval::hybrid::vector_store::{
	SearchResult as VectorSearchResult,
};
use crate::indexer::CodeLocation;

/// Process keyword results and add them to the results map
pub(crate) fn process_keyword_results(
	config: &HybridSearchConfig,
	results_by_key: &mut HashMap<String, HybridSearchResult>,
	keyword_results: Vec<RankedItem<SearchHit>>,
	keyword_weight: f32,
	query: &str,
) {
	for ranked in keyword_results {
		let key = format!(
			"{}:{}",
			ranked.item.symbol.name,
			ranked.item.symbol.location.line
		);

		// Calculate combined boost
		let file = &ranked.item.symbol.location.file;
		let doc_type = DocumentType::from_path(file);
		let doc_boost = doc_type.boost_factor_for_query(query);
		let kind_boost = ranked.item.symbol.kind.boost_factor();
		let combined_boost = doc_boost * kind_boost;

		// Apply boost to RRF score
		let base = rrf_score(ranked.rank, config.rrf_k);
		let rrf = base * keyword_weight * combined_boost;

		results_by_key
			.entry(key.clone())
			.and_modify(|r| {
				r.rrf_score += rrf;
				r.keyword_rank = Some(ranked.rank);
				r.keyword_score = Some(ranked.score);
			})
			.or_insert(HybridSearchResult {
				symbol: ranked.item.symbol,
				rrf_score: rrf,
				keyword_rank: Some(ranked.rank),
				semantic_rank: None,
				keyword_score: Some(ranked.score),
				semantic_distance: None,
				rerank_score: None,
			});
	}
}

/// Process semantic results and add them to the results map
pub(crate) fn process_semantic_results(
	config: &HybridSearchConfig,
	results_by_key: &mut HashMap<String, HybridSearchResult>,
	semantic_results: Vec<RankedItem<VectorSearchResult>>,
	semantic_weight: f32,
	query: &str,
) {
	for ranked in semantic_results {
		let key = format!(
			"{}:{}",
			ranked.item.point.symbol_name,
			ranked.item.point.line,
		);

		// Calculate combined boost
		let doc_type =
			DocumentType::from_path(&ranked.item.point.file_path);
		let doc_boost = doc_type.boost_factor_for_query(query);
		let symbol_kind =
			parse_symbol_kind(&ranked.item.point.symbol_kind);
		let kind_boost = symbol_kind.boost_factor();
		let combined_boost = doc_boost * kind_boost;

		// Apply boost to RRF score
		let base = rrf_score(ranked.rank, config.rrf_k);
		let rrf = base * semantic_weight * combined_boost;

		results_by_key
			.entry(key.clone())
			.and_modify(|r| {
				r.rrf_score += rrf;
				r.semantic_rank = Some(ranked.rank);
				r.semantic_distance = Some(ranked.score);
			})
			.or_insert_with(|| {
				// create symbol from vector point
				let point = &ranked.item.point;
				let symbol = Symbol::new(
					point.symbol_name.clone(),
					symbol_kind,
					CodeLocation::new(
						point.file_path.clone(),
						point.line,
						1,
						0,
						0,
					),
				);

				HybridSearchResult {
					symbol,
					rrf_score: rrf,
					keyword_rank: None,
					semantic_rank: Some(ranked.rank),
					keyword_score: None,
					semantic_distance: Some(ranked.score),
					rerank_score: None,
				}
			});
	}
}
