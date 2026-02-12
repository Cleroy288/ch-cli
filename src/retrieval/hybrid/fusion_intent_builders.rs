//! Intent-aware fusion builder functions
//!
//! Builds individual keyword and semantic results with
//! intent-aware boost factors for the intent fusion.

use std::collections::HashMap;
use std::sync::Arc;

use crate::indexer::{
	ByteSpan, CodeLocation, DocumentType, SearchHit,
	Symbol,
};
use crate::retrieval::hybrid::converters::parse_symbol_kind;
use crate::retrieval::hybrid::fusion::RankedItem;
use crate::retrieval::hybrid::fusion_core::FusionParams;
use crate::retrieval::hybrid::fusion_intent::IntentContext;
use crate::retrieval::hybrid::hub_penalty::ref_count_penalty;
use crate::retrieval::hybrid::hub_ref_counts::lookup_ref_count;
use crate::retrieval::hybrid::result::HybridSearchResult;
use crate::retrieval::hybrid::vector_store::SearchResult
	as VectorSearchResult;

/// Bundled fusion params and intent context
pub(crate) struct IntentFusionArgs<'prm> {
	/// fusion weights and query
	pub params: &'prm FusionParams<'prm>,
	/// intent and ref count context
	pub ctx: &'prm IntentContext<'prm>,
	/// normalized scores per result
	pub norms: Vec<f32>,
}

/// Build a keyword result with intent-aware boost
pub(crate) fn build_keyword_intent_result<'prm>(
	ranked: &RankedItem<SearchHit>,
	norm_score: f32,
	args: &IntentFusionArgs<'prm>,
) -> HybridSearchResult {
	let boost = compute_kw_intent_boost(
		ranked, args.params, args.ctx,
	);
	let score =
		norm_score * args.params.keyword_weight * boost;

	HybridSearchResult {
		symbol: Arc::new(ranked.item.symbol.clone()),
		score,
		keyword_rank: Some(ranked.rank),
		semantic_rank: None,
		keyword_score: Some(ranked.score),
		semantic_distance: None,
		rerank_score: None,
	}
}

/// Compute keyword intent boost factor
fn compute_kw_intent_boost<'prm>(
	ranked: &RankedItem<SearchHit>,
	params: &FusionParams<'prm>,
	ctx: &IntentContext<'prm>,
) -> f32 {
	let file = &ranked.item.symbol.location.file;
	let doc_type = DocumentType::from_path(file);
	let doc_qb =
		doc_type.boost_factor_for_query(params.query);
	let doc_ib =
		doc_type.boost_factor_for_intent(ctx.intent);
	let kind_boost = ranked
		.item
		.symbol
		.kind
		.boost_factor_for_intent(ctx.intent);
	let ref_cnt = lookup_ref_count(
		&ranked.item.symbol.name, ctx.ref_counts,
	);
	let penalty = ref_count_penalty(ref_cnt);
	doc_qb * doc_ib * kind_boost * penalty
}

/// Build a semantic result with intent-aware boost
pub(crate) fn build_semantic_intent_result<'prm>(
	ranked: &RankedItem<VectorSearchResult>,
	norm_score: f32,
	args: &IntentFusionArgs<'prm>,
) -> (String, HybridSearchResult) {
	let doc_type = DocumentType::from_path(
		&ranked.item.point.file_path,
	);
	let boost = compute_sem_intent_boost(
		&doc_type, args,  ranked,
	);
	let score =
		norm_score * args.params.semantic_weight * boost;
	let key = format!(
		"{}:{}",
		ranked.item.point.symbol_name,
		ranked.item.point.line,
	);
	let symbol = build_symbol_from_point(ranked);
	let result = HybridSearchResult {
		symbol: Arc::new(symbol),
		score,
		keyword_rank: None,
		semantic_rank: Some(ranked.rank),
		keyword_score: None,
		semantic_distance: Some(ranked.score),
		rerank_score: None,
	};
	(key, result)
}

/// Compute semantic intent boost factor
fn compute_sem_intent_boost<'prm>(
	doc_type: &DocumentType,
	args: &IntentFusionArgs<'prm>,
	ranked: &RankedItem<VectorSearchResult>,
) -> f32 {
	let doc_qb =
		doc_type.boost_factor_for_query(args.params.query);
	let doc_ib = doc_type
		.boost_factor_for_intent(args.ctx.intent);
	let kind = parse_symbol_kind(
		&ranked.item.point.symbol_kind,
	);
	let kind_boost =
		kind.boost_factor_for_intent(args.ctx.intent);
	let ref_cnt = lookup_ref_count(
		&ranked.item.point.symbol_name,
		args.ctx.ref_counts,
	);
	let penalty = ref_count_penalty(ref_cnt);
	doc_qb * doc_ib * kind_boost * penalty
}

/// Build Symbol from semantic vector point
fn build_symbol_from_point(
	ranked: &RankedItem<VectorSearchResult>,
) -> Symbol {
	let point = &ranked.item.point;
	let kind = parse_symbol_kind(&point.symbol_kind);
	Symbol::new(
		point.symbol_name.clone(),
		kind,
		CodeLocation::new(
			point.file_path.clone(),
			point.line, 1, ByteSpan::ZERO,
		),
	)
}

/// Merge a result into the map, summing scores
pub(crate) fn merge_into_map(
	map: &mut HashMap<String, HybridSearchResult>,
	key: String,
	result: HybridSearchResult,
) {
	map.entry(key)
		.and_modify(|existing| {
			existing.score += result.score;
			if result.keyword_rank.is_some() {
				existing.keyword_rank =
					result.keyword_rank;
				existing.keyword_score =
					result.keyword_score;
			}
			if result.semantic_rank.is_some() {
				existing.semantic_rank =
					result.semantic_rank;
				existing.semantic_distance =
					result.semantic_distance;
			}
		})
		.or_insert(result);
}
