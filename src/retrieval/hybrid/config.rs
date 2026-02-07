//! Configuration for hybrid search
//!
//! Contains HybridSearchConfig struct with all tunable parameters
//! for keyword/semantic search fusion.

/// Configuration for hybrid search
#[derive(Debug, Clone)]
pub struct HybridSearchConfig {
	/// weight for keyword search results
	pub keyword_weight: f32,
	/// weight for semantic search results
	pub semantic_weight: f32,
	/// number of candidates to fetch from each source
	pub candidates_per_source: usize,
	/// RRF k constant
	pub rrf_k: f32,
	/// Minimum RRF score threshold - filters below this
	pub rrf_score_threshold: f32,
	/// Minimum results to keep per type regardless of threshold
	pub min_results_per_type: usize,
}

impl Default for HybridSearchConfig {
	fn default() -> Self {
		Self {
			keyword_weight: 1.0,
			semantic_weight: 1.0,
			candidates_per_source: 50,
			rrf_k: 60.0,
			rrf_score_threshold: 0.015,
			min_results_per_type: 1,
		}
	}
}

