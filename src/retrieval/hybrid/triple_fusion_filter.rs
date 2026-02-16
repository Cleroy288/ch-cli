//! Triple Hybrid Search — Relevance Filtering
//!
//! Filters fused results by score threshold with min/max
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
		let threshold = self.config.score_threshold;
		let min_results =
			self.config.min_results_per_type;
		let total = results.len();

		let filtered = apply_threshold(
			results, min_results, max_limit, threshold,
		);

		log_filter_stats(
			content_type, filtered.len(), total,
			threshold,
		);
		filtered
	}
}

/// Apply threshold filtering with min/max guarantees
fn apply_threshold(
	results: Vec<HybridSearchResult>,
	min_results: usize,
	max_limit: usize,
	threshold: f32,
) -> Vec<HybridSearchResult> {
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
		if result.score >= threshold {
			filtered.push(result);
		}
	}

	filtered
}

/// Log filtering stats if any results were removed
#[allow(clippy::print_stderr)]
fn log_filter_stats(
	content_type: &str,
	kept: usize,
	total: usize,
	threshold: f32,
) {
	let below = total.saturating_sub(kept);
	if below > 0 {
		eprintln!(
			"[pipeline] {}: {} results, {} filtered \
			 (score < {})",
			content_type, kept, below, threshold
		);
	}
}
