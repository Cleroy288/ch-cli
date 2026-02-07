//! Hybrid search result type
//!
//! Contains the HybridSearchResult struct representing
//! a combined result from keyword and semantic search.

use crate::indexer::Symbol;

/// Result from hybrid search
#[derive(Debug, Clone)]
pub struct HybridSearchResult {
	/// the symbol
	pub symbol: Symbol,
	/// combined RRF score
	pub rrf_score: f32,
	/// keyword rank (None if not in keyword results)
	pub keyword_rank: Option<usize>,
	/// semantic rank (None if not in semantic results)
	pub semantic_rank: Option<usize>,
	/// keyword BM25 score
	pub keyword_score: Option<f32>,
	/// semantic distance (lower is better)
	pub semantic_distance: Option<f32>,
	/// rerank score from cross-encoder (if reranking was applied)
	pub rerank_score: Option<f32>,
}
