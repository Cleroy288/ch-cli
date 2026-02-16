//! Fusion types for hybrid search
//!
//! Contains RankedItem and FusedResult types used across
//! the fusion pipeline.

/// A scored item from a single search source
#[derive(Debug, Clone)]
pub struct RankedItem<T> {
	/// the item
	pub item: T,
	/// rank in the result list (1-indexed)
	pub rank: usize,
	/// original score from the search source
	pub score: f32,
}

/// Result of hybrid fusion
#[derive(Debug, Clone)]
pub struct FusedResult<T> {
	/// the fused item
	pub item: T,
	/// combined fusion score
	pub score: f32,
	/// keyword rank (None if not in keyword results)
	pub keyword_rank: Option<usize>,
	/// semantic rank (None if not in semantic results)
	pub semantic_rank: Option<usize>,
	/// original keyword score
	pub keyword_score: Option<f32>,
	/// original semantic score (distance, lower is better)
	pub semantic_score: Option<f32>,
}
