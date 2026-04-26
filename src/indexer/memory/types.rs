use serde::{Deserialize, Serialize};

use crate::domain::memory::Interaction;

/// A search hit with relevance score
#[derive(Debug, Clone)]
pub struct MemoryHit {
	/// Matched interaction record
	pub interaction: Interaction,
	/// Relevance score (higher = better)
	pub score: f32,
}

/// Aggregate statistics for the memory store
#[derive(
	Debug, Clone, Serialize, Deserialize,
)]
pub struct MemoryStats {
	/// Total stored interactions
	pub total: usize,
	/// Distinct session count
	pub sessions: usize,
	/// Combined JSONL file size
	pub size_bytes: u64,
	/// Unix epoch of last write
	pub last_updated: u64,
}
