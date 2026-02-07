//! Tiered expansion result types

use crate::retrieval::daemon::protocol::SearchSpec;

/// Which tier was used for query expansion
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TierUsed {
	/// Fast path with validated symbols (no LLM)
	FastPathValidated,
	/// Fast path without validation (no graph)
	FastPathUnvalidated,
	/// LLM expansion (Phi-3)
	LlmExpansion,
}

/// Result of tiered expansion
#[derive(Debug, Clone)]
pub struct TieredResult {
	/// the search specification
	pub spec: SearchSpec,
	/// which tier was used
	pub tier_used: TierUsed,
	/// time taken in milliseconds
	pub time_ms: u64,
}
