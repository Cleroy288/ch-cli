//! Result types for memory search and stats.

use serde::{Deserialize, Serialize};

use crate::domain::memory::Interaction;

/// A search hit with relevance score
#[derive(Debug, Clone)]
pub struct MemoryHit {
	pub interaction: Interaction, // matched record
	pub score: f32,               // relevance score
}

/// Aggregate statistics for the memory store
#[derive(
	Debug, Clone, Serialize, Deserialize,
)]
pub struct MemoryStats {
	pub total: usize,       // total interactions
	pub sessions: usize,    // number of sessions
	pub size_bytes: u64,    // total JSONL size
	pub last_updated: u64,  // last write timestamp
}
