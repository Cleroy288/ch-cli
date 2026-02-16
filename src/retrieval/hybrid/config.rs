//! Configuration for hybrid search
//!
//! Contains HybridSearchConfig struct with all tunable parameters
//! for keyword/semantic search fusion.

/// Configuration for hybrid search
#[derive(Debug, Clone)]
pub struct HybridSearchConfig {
	/// weight for keyword search results (0.0-1.0)
	pub keyword_weight: f32,
	/// weight for semantic search results (0.0-1.0)
	pub semantic_weight: f32,
	/// number of candidates to fetch from each source
	pub candidates_per_source: usize,
	/// Minimum fusion score threshold - filters below this
	pub score_threshold: f32,
	/// Minimum results to keep per type regardless of threshold
	pub min_results_per_type: usize,
}

impl Default for HybridSearchConfig {
	fn default() -> Self {
		Self {
			keyword_weight: 0.5,
			semantic_weight: 0.5,
			candidates_per_source: 50,
			score_threshold: 0.05,
			min_results_per_type: 1,
		}
	}
}
