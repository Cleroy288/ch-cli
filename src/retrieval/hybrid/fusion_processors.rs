//! Fusion Result Processors
//!
//! Helper functions for processing keyword and semantic
//! results into the Convex Combination fusion map.

use std::collections::HashMap;
use std::sync::Arc;

use crate::indexer::{
	ByteSpan, CodeLocation, DocumentType, SearchHit,
	Symbol,
};
use crate::retrieval::hybrid::converters::parse_symbol_kind;
use crate::retrieval::hybrid::fusion::RankedItem;
use crate::retrieval::hybrid::hub_penalty::ref_count_penalty;
use crate::retrieval::hybrid::hub_ref_counts::lookup_ref_count;
use crate::retrieval::hybrid::result::HybridSearchResult;
use crate::retrieval::hybrid::vector_store::SearchResult
	as VectorSearchResult;

/// Bundled parameters for fusion processing
pub struct ProcessorParams<'param> {
	/// normalized scores per result
	pub norm_scores: &'param [f32],
	/// weight for this search channel
	pub weight: f32,
	/// the search query string
	pub query: &'param str,
	/// ref counts for hub penalty
	pub ref_counts: &'param HashMap<String, usize>,
}

/// Process keyword results into the fusion map
pub(crate) fn process_keyword_results<'prm>(
	results_by_key: &mut HashMap<
		String,
		HybridSearchResult,
	>,
	keyword_results: Vec<RankedItem<SearchHit>>,
	params: &ProcessorParams<'prm>,
) {
	for (idx, ranked) in
		keyword_results.iter().enumerate()
	{
		let key = format!(
			"{}:{}",
			ranked.item.symbol.name,
			ranked.item.symbol.location.line
		);
		let score =
			compute_keyword_cc(ranked, idx, params);
		merge_keyword_result(
			results_by_key, &key, ranked, score,
		);
	}
}

/// Compute CC score for a single keyword result
fn compute_keyword_cc<'prm>(
	ranked: &RankedItem<SearchHit>,
	index: usize,
	params: &ProcessorParams<'prm>,
) -> f32 {
	let file = &ranked.item.symbol.location.file;
	let doc_type = DocumentType::from_path(file);
	let doc_boost =
		doc_type.boost_factor_for_query(params.query);
	let kind_boost =
		ranked.item.symbol.kind.boost_factor();
	let ref_cnt = lookup_ref_count(
		&ranked.item.symbol.name, params.ref_counts,
	);
	let penalty = ref_count_penalty(ref_cnt);
	let combined = doc_boost * kind_boost * penalty;
	let norm = params
		.norm_scores
		.get(index)
		.copied()
		.unwrap_or(0.0);
	norm * params.weight * combined
}

/// Merge a keyword result into the fusion map
fn merge_keyword_result(
	map: &mut HashMap<String, HybridSearchResult>,
	key: &str,
	ranked: &RankedItem<SearchHit>,
	cc_score: f32,
) {
	map.entry(key.to_string())
		.and_modify(|existing| {
			existing.score += cc_score;
			existing.keyword_rank = Some(ranked.rank);
			existing.keyword_score = Some(ranked.score);
		})
		.or_insert(HybridSearchResult {
			symbol: Arc::new(ranked.item.symbol.clone()),
			score: cc_score,
			keyword_rank: Some(ranked.rank),
			semantic_rank: None,
			keyword_score: Some(ranked.score),
			semantic_distance: None,
			rerank_score: None,
		});
}

/// Process semantic results into the fusion map
pub(crate) fn process_semantic_results<'prm>(
	results_by_key: &mut HashMap<
		String,
		HybridSearchResult,
	>,
	semantic_results: Vec<RankedItem<VectorSearchResult>>,
	params: &ProcessorParams<'prm>,
) {
	for (idx, ranked) in
		semantic_results.iter().enumerate()
	{
		let key = format!(
			"{}:{}",
			ranked.item.point.symbol_name,
			ranked.item.point.line,
		);
		let kind = parse_symbol_kind(
			&ranked.item.point.symbol_kind,
		);
		let score = compute_semantic_cc(
			ranked, idx, kind, params,
		);
		let result =
			build_semantic_result(ranked, score, kind);
		merge_semantic_entry(
			results_by_key, &key, result,
		);
	}
}

/// Compute CC score for a single semantic result
fn compute_semantic_cc<'prm>(
	ranked: &RankedItem<VectorSearchResult>,
	index: usize,
	kind: crate::indexer::SymbolKind,
	params: &ProcessorParams<'prm>,
) -> f32 {
	let doc_type = DocumentType::from_path(
		&ranked.item.point.file_path,
	);
	let doc_boost =
		doc_type.boost_factor_for_query(params.query);
	let kind_boost = kind.boost_factor();
	let ref_cnt = lookup_ref_count(
		&ranked.item.point.symbol_name,
		params.ref_counts,
	);
	let penalty = ref_count_penalty(ref_cnt);
	let combined = doc_boost * kind_boost * penalty;
	let norm = params
		.norm_scores
		.get(index)
		.copied()
		.unwrap_or(0.0);
	norm * params.weight * combined
}

/// Build a HybridSearchResult for a semantic item
fn build_semantic_result(
	ranked: &RankedItem<VectorSearchResult>,
	cc_score: f32,
	kind: crate::indexer::SymbolKind,
) -> HybridSearchResult {
	let point = &ranked.item.point;
	let symbol = Symbol::new(
		point.symbol_name.clone(),
		kind,
		CodeLocation::new(
			point.file_path.clone(),
			point.line,
			1,
			ByteSpan::ZERO,
		),
	);
	HybridSearchResult {
		symbol: Arc::new(symbol),
		score: cc_score,
		keyword_rank: None,
		semantic_rank: Some(ranked.rank),
		keyword_score: None,
		semantic_distance: Some(ranked.score),
		rerank_score: None,
	}
}

/// Merge a semantic result into the fusion map
fn merge_semantic_entry(
	map: &mut HashMap<String, HybridSearchResult>,
	key: &str,
	result: HybridSearchResult,
) {
	map.entry(key.to_string())
		.and_modify(|existing| {
			existing.score += result.score;
			existing.semantic_rank =
				result.semantic_rank;
			existing.semantic_distance =
				result.semantic_distance;
		})
		.or_insert(result);
}
