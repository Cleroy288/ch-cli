//! Triple Hybrid Search — Relevance Filtering
//!
//! Filters fused results by RRF score threshold with min/max
//! guarantees to ensure quality while maintaining coverage.

use crate::retrieval::hybrid::triple::TripleHybridSearch;
use crate::retrieval::hybrid::HybridSearchResult;

impl TripleHybridSearch {
	/// Filter results by relevance with min/max guarantees
	/// - Always includes minimum results (if available)
	/// - Filters below threshold after minimum met
	/// - Stops at max_limit
	pub(crate) fn filter_by_relevance(
		&self,
		results: Vec<HybridSearchResult>,
		max_limit: usize,
		content_type: &str,
	) -> Vec<HybridSearchResult> {
		let threshold = self.config.rrf_score_threshold;
		let min_results = self.config.min_results_per_type;
		let total = results.len();

		let mut filtered: Vec<HybridSearchResult> =
			Vec::new();

		for result in results.into_iter() {
			if filtered.len() < min_results {
				filtered.push(result);
				continue;
			}
			if filtered.len() >= max_limit {
				break;
			}
			if result.rrf_score >= threshold {
				filtered.push(result);
			}
		}

		// Debug log if filtering occurred
		let below = total.saturating_sub(filtered.len());
		if below > 0 {
			eprintln!(
				"[pipeline] {}: {} results, {} filtered \
				 (score < {})",
				content_type,
				filtered.len(),
				below,
				threshold
			);
		}

		filtered
	}
}
