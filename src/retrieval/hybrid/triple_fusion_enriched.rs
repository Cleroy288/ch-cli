//! Triple Hybrid Search — Enriched Fusion
//!
//! Fuses keyword + semantic + enriched results for the code
//! pipeline using Convex Combination. Enriched results provide
//! NL-focused boost from LLM-generated documentation embeddings.

use crate::indexer::SearchHit;
use crate::retrieval::hybrid::normalize::{
	min_max_normalize, score_bounds,
};
use crate::retrieval::hybrid::triple::{
	PipelineSlice, TripleHybridSearch,
};
use crate::retrieval::hybrid::triple_fusion_helpers::{
	empty_result, symbol_key, ScoreBoundsRange,
};
use crate::retrieval::hybrid::vector_store::SearchResult
	as VectorSearchResult;
use crate::retrieval::hybrid::HybridSearchResult;

/// Weight for enriched results (NL doc boost)
const ENRICHED_WEIGHT: f32 = 0.7;

/// Input for enriched fusion: keyword + semantic + enriched
pub(crate) struct EnrichedFusionInput<'input> {
	/// keyword search hits
	pub keyword_hits: &'input [SearchHit],
	/// semantic vector results
	pub semantic_results: &'input [VectorSearchResult],
	/// enriched NL-doc vector results
	pub enriched_results: &'input [VectorSearchResult],
}

impl TripleHybridSearch {
	/// Fuse keyword + semantic + enriched for code pipeline
	pub(crate) fn fuse_with_enriched<'inp>(
		&self,
		input: EnrichedFusionInput<'inp>,
		slice: &PipelineSlice<'inp>,
	) -> Vec<HybridSearchResult> {
		let mut scores = self.build_cc_scores(
			input.keyword_hits,
			input.semantic_results,
		);
		self.apply_enriched_scores(
			&mut scores, input.enriched_results,
		);
		let mut results: Vec<HybridSearchResult> =
			scores
				.drain()
				.map(|(_, val)| val)
				.collect();
		results.sort_by(|left, right| {
			right
				.score
				.partial_cmp(&left.score)
				.unwrap_or(std::cmp::Ordering::Equal)
		});
		self.filter_by_relevance(
			results,
			slice.limit,
			slice.content_type,
		)
	}

	/// Add enriched NL-doc scores as third signal
	fn apply_enriched_scores(
		&self,
		scores: &mut std::collections::HashMap<
			String,
			HybridSearchResult,
		>,
		enriched_results: &[VectorSearchResult],
	) {
		let dists: Vec<f32> = enriched_results
			.iter()
			.map(|hit| hit.distance)
			.collect();
		let (min, max) = score_bounds(&dists);
		let bounds = ScoreBoundsRange { min, max };

		for result in enriched_results.iter() {
			self.boost_enriched_entry(
				scores, result, &bounds,
			);
		}
	}

	/// Boost a single enriched entry in the scores map
	fn boost_enriched_entry(
		&self,
		scores: &mut std::collections::HashMap<
			String,
			HybridSearchResult,
		>,
		result: &VectorSearchResult,
		bounds: &ScoreBoundsRange,
	) {
		let Some(sym) = self
			.find_symbol_by_vector(&result.point)
		else {
			return;
		};
		let key = symbol_key(sym);
		let norm = 1.0 - min_max_normalize(
			result.distance, bounds.min, bounds.max,
		);
		let entry = scores
			.entry(key)
			.or_insert_with(|| empty_result(sym));
		entry.score += norm * ENRICHED_WEIGHT;
	}
}
