//! Triple Hybrid Search — Fusion Implementation
//!
//! RRF fusion logic for combining keyword and semantic results
//! within a single pipeline (code, doc, or notes).

use std::collections::HashMap;

use crate::indexer::symbols::Symbol;
use crate::indexer::SearchHit;
use crate::retrieval::hybrid::fusion::rrf_score;
use crate::retrieval::hybrid::triple::TripleHybridSearch;
use crate::retrieval::hybrid::vector_store::{
	SearchResult as VectorSearchResult, VectorPoint,
};
use crate::retrieval::hybrid::HybridSearchResult;

impl TripleHybridSearch {
	/// Fuse keyword and semantic results for one pipeline
	pub(crate) fn fuse_pipeline_results(
		&self,
		keyword_hits: &[SearchHit],
		semantic_results: &[VectorSearchResult],
		limit: usize,
		content_type: &str,
	) -> Vec<HybridSearchResult> {
		let mut scores = self
			.build_rrf_scores(keyword_hits, semantic_results);

		// Sort by RRF score descending
		let mut results: Vec<HybridSearchResult> =
			scores.drain().map(|(_, v)| v).collect();
		results.sort_by(|a, b| {
			b.rrf_score
				.partial_cmp(&a.rrf_score)
				.unwrap_or(std::cmp::Ordering::Equal)
		});

		self.filter_by_relevance(
			results,
			limit,
			content_type,
		)
	}

	/// Build RRF scores from keyword and semantic results
	fn build_rrf_scores(
		&self,
		keyword_hits: &[SearchHit],
		semantic_results: &[VectorSearchResult],
	) -> HashMap<String, HybridSearchResult> {
		let mut scores: HashMap<String, HybridSearchResult> =
			HashMap::new();
		let k = self.config.rrf_k;

		// Process keyword results
		for (i, hit) in keyword_hits.iter().enumerate() {
			let key = symbol_key(&hit.symbol);
			let rrf = rrf_score(i + 1, k)
				* self.config.keyword_weight;

			let entry = scores
				.entry(key)
				.or_insert_with(|| empty_result(&hit.symbol));
			entry.rrf_score += rrf;
			entry.keyword_rank = Some(i + 1);
			entry.keyword_score = Some(hit.score);
		}

		// Process semantic results
		for (i, result) in
			semantic_results.iter().enumerate()
		{
			if let Some(sym) =
				self.find_symbol_by_vector(&result.point)
			{
				let key = symbol_key(sym);
				let rrf = rrf_score(i + 1, k)
					* self.config.semantic_weight;

				let entry = scores
					.entry(key)
					.or_insert_with(|| empty_result(sym));
				entry.rrf_score += rrf;
				entry.semantic_rank = Some(i + 1);
				entry.semantic_distance =
					Some(result.distance);
			}
		}

		scores
	}

	/// Find symbol by vector point index
	fn find_symbol_by_vector(
		&self,
		point: &VectorPoint,
	) -> Option<&Symbol> {
		self.symbols.get(point.id as usize)
	}
}

/// Create unique key for a symbol (file:line:name)
fn symbol_key(symbol: &Symbol) -> String {
	format!(
		"{}:{}:{}",
		symbol.location.file.display(),
		symbol.location.line,
		symbol.name
	)
}

/// Create empty HybridSearchResult for a symbol
fn empty_result(symbol: &Symbol) -> HybridSearchResult {
	HybridSearchResult {
		symbol: symbol.clone(),
		rrf_score: 0.0,
		keyword_rank: None,
		semantic_rank: None,
		keyword_score: None,
		semantic_distance: None,
		rerank_score: None,
	}
}
