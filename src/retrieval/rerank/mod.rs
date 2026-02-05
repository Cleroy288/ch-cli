//! Semantic Reranking Module
//!
//! Reranks search results using a cross-encoder model.
//! The cross-encoder scores (query, document) pairs directly,
//! providing more accurate relevance scores than bi-encoder similarity.

pub mod cross_encoder;

pub use cross_encoder::BgeReranker;

/// Result of reranking a single document
#[derive(Debug, Clone)]
pub struct RerankedItem<T> {
	/// the original item
	pub item: T,
	/// cross-encoder relevance score
	pub score: f32,
	/// original rank before reranking
	pub original_rank: usize,
}

/// Rerank a list of items by their scores
pub fn rerank_by_score<T>(items: Vec<RerankedItem<T>>) -> Vec<RerankedItem<T>> {
	let mut sorted = items;
	sorted.sort_by(|a, b| b.score.partial_cmp(&a.score).unwrap_or(std::cmp::Ordering::Equal));
	sorted
}
