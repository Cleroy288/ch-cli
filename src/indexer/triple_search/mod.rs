mod index;
mod search;
mod types;

// Re-export all public types for backward compatibility
pub use index::TripleSearchIndex;
pub use types::{
	TripleIndexStats, TripleLimits,
	TripleSearchResults,
};
