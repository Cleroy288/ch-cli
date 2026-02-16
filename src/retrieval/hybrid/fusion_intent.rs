//! Intent-aware fusion functions for hybrid search
//!
//! Combines keyword and semantic results using min-max
//! normalized Convex Combination with intent-aware boosting.

use std::collections::HashMap;

use crate::indexer::SearchHit;
use crate::retrieval::daemon::protocol::QueryIntent;
use crate::retrieval::hybrid::fusion::RankedItem;
use crate::retrieval::hybrid::fusion_core::FusionParams;
use crate::retrieval::hybrid::fusion_intent_builders::{
	build_keyword_intent_result,
	build_semantic_intent_result,
	merge_into_map, IntentFusionArgs,
};
use crate::retrieval::hybrid::normalize::{
	normalize_keyword_scores, normalize_semantic_scores,
};
use crate::retrieval::hybrid::result::HybridSearchResult;
use crate::retrieval::hybrid::vector_store::SearchResult
	as VectorSearchResult;

/// Bundled intent and ref count context for fusion
pub struct IntentContext<'ctx> {
	/// query intent for symbol kind boosting
	pub intent: &'ctx QueryIntent,
	/// ref counts for hub penalty
	pub ref_counts: &'ctx HashMap<String, usize>,
}

/// Fuse results with intent-aware symbol kind boost
/// using Convex Combination with normalized scores
pub fn fuse_with_weights_and_intent<'prm>(
	keyword_results: Vec<RankedItem<SearchHit>>,
	semantic_results: Vec<RankedItem<VectorSearchResult>>,
	params: &FusionParams<'prm>,
	ctx: &IntentContext<'prm>,
) -> Vec<HybridSearchResult> {
	let mut results_map: HashMap<
		String,
		HybridSearchResult,
	> = HashMap::new();

	let kw_args = IntentFusionArgs {
		params,
		ctx,
		norms: normalize_keyword_scores(
			&keyword_results,
		),
	};
	let sem_args = IntentFusionArgs {
		params,
		ctx,
		norms: normalize_semantic_scores(
			&semantic_results,
		),
	};

	process_keyword_intent(
		&mut results_map, &keyword_results, &kw_args,
	);
	process_semantic_intent(
		&mut results_map, &semantic_results, &sem_args,
	);

	sort_fused_results(results_map)
}

/// Sort fused results by score descending
fn sort_fused_results(
	results_map: HashMap<String, HybridSearchResult>,
) -> Vec<HybridSearchResult> {
	let mut results: Vec<_> =
		results_map.into_values().collect();
	results.sort_by(|left, right| {
		right.score.partial_cmp(&left.score).unwrap()
	});
	results
}

/// Process keyword results with intent boost
fn process_keyword_intent<'prm>(
	results_map: &mut HashMap<
		String,
		HybridSearchResult,
	>,
	keyword_results: &[RankedItem<SearchHit>],
	args: &IntentFusionArgs<'prm>,
) {
	for (idx, ranked) in
		keyword_results.iter().enumerate()
	{
		let norm =
			args.norms.get(idx).copied().unwrap_or(0.0);
		let result = build_keyword_intent_result(
			ranked, norm, args,
		);
		let key = format!(
			"{}:{}",
			ranked.item.symbol.name,
			ranked.item.symbol.location.line,
		);
		merge_into_map(results_map, key, result);
	}
}

/// Process semantic results with intent boost
fn process_semantic_intent<'prm>(
	results_map: &mut HashMap<
		String,
		HybridSearchResult,
	>,
	semantic_results: &[RankedItem<VectorSearchResult>],
	args: &IntentFusionArgs<'prm>,
) {
	for (idx, ranked) in
		semantic_results.iter().enumerate()
	{
		let norm =
			args.norms.get(idx).copied().unwrap_or(0.0);
		let (key, result) =
			build_semantic_intent_result(
				ranked, norm, args,
			);
		merge_into_map(results_map, key, result);
	}
}
