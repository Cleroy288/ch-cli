//! Triple Hybrid Search — Fusion Implementation
//!
//! Convex Combination fusion for combining keyword and
//! semantic results within a single pipeline.

use std::collections::HashMap;

use crate::indexer::symbols::Symbol;
use crate::indexer::SearchHit;
use crate::retrieval::hybrid::normalize::{
	min_max_normalize, score_bounds,
};
use crate::retrieval::hybrid::triple::{
	PipelineSlice, TripleHybridSearch,
};
use crate::retrieval::hybrid::triple_fusion_helpers::{
	hit_penalty, insert_keyword_entry,
	insert_semantic_entry, ScoreBoundsRange, ScoreEntry,
};
use crate::retrieval::hybrid::vector_store::{
	PointMeta, SearchResult as VectorSearchResult,
};
use crate::retrieval::hybrid::HybridSearchResult;

impl TripleHybridSearch {
	/// Fuse keyword and semantic results for one pipeline
	pub(crate) fn fuse_pipeline_results<'slc>(
		&self,
		keyword_hits: &[SearchHit],
		semantic_results: &[VectorSearchResult],
		slice: &PipelineSlice<'slc>,
	) -> Vec<HybridSearchResult> {
		let mut scores = self.build_cc_scores(
			keyword_hits, semantic_results,
		);

		let mut results: Vec<HybridSearchResult> =
			scores.drain().map(|(_, val)| val).collect();
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

	/// Build CC scores from keyword and semantic results
	pub(crate) fn build_cc_scores(
		&self,
		keyword_hits: &[SearchHit],
		semantic_results: &[VectorSearchResult],
	) -> HashMap<String, HybridSearchResult> {
		let mut scores: HashMap<
			String,
			HybridSearchResult,
		> = HashMap::new();

		let kw_bounds = compute_kw_bounds(keyword_hits);
		let sem_bounds =
			compute_sem_bounds(semantic_results);

		self.merge_keyword_cc(
			&mut scores, keyword_hits, &kw_bounds,
		);
		self.merge_semantic_cc(
			&mut scores, semantic_results, &sem_bounds,
		);

		scores
	}

	/// Find symbol by point metadata index
	pub(crate) fn find_symbol_by_vector(
		&self,
		point: &PointMeta,
	) -> Option<&Symbol> {
		self.symbols.get(point.id as usize)
	}
}

impl TripleHybridSearch {
	/// Merge keyword CC scores into the results map
	fn merge_keyword_cc(
		&self,
		scores: &mut HashMap<String, HybridSearchResult>,
		keyword_hits: &[SearchHit],
		bounds: &ScoreBoundsRange,
	) {
		for (idx, hit) in
			keyword_hits.iter().enumerate()
		{
			let norm = min_max_normalize(
				hit.score, bounds.min, bounds.max,
			);
			let penalty = hit_penalty(
				&hit.symbol.name, &self.ref_counts,
			);
			let score =
				norm * self.config.keyword_weight * penalty;

			let entry = ScoreEntry {
				score, idx, raw: hit.score,
			};
			insert_keyword_entry(
				scores, &hit.symbol, &entry,
			);
		}
	}

	/// Merge semantic CC scores into the results map
	fn merge_semantic_cc(
		&self,
		scores: &mut HashMap<String, HybridSearchResult>,
		semantic_results: &[VectorSearchResult],
		bounds: &ScoreBoundsRange,
	) {
		for (idx, result) in
			semantic_results.iter().enumerate()
		{
			let Some(sym) = self
				.find_symbol_by_vector(&result.point)
			else {
				continue;
			};
			let norm = 1.0 - min_max_normalize(
				result.distance,
				bounds.min,
				bounds.max,
			);
			let penalty = hit_penalty(
				&sym.name, &self.ref_counts,
			);
			let score =
				norm * self.config.semantic_weight * penalty;

			let entry = ScoreEntry {
				score, idx, raw: result.distance,
			};
			insert_semantic_entry(
				scores, sym, &entry,
			);
		}
	}
}

/// Compute keyword score bounds from hits
fn compute_kw_bounds(
	keyword_hits: &[SearchHit],
) -> ScoreBoundsRange {
	let raw: Vec<f32> = keyword_hits
		.iter()
		.map(|hit| hit.score)
		.collect();
	let (min, max) = score_bounds(&raw);
	ScoreBoundsRange { min, max }
}

/// Compute semantic distance bounds from results
fn compute_sem_bounds(
	results: &[VectorSearchResult],
) -> ScoreBoundsRange {
	let raw: Vec<f32> = results
		.iter()
		.map(|res| res.distance)
		.collect();
	let (min, max) = score_bounds(&raw);
	ScoreBoundsRange { min, max }
}
