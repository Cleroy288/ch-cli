//! Triple Search Index
//!
//! Provides separate Tantivy indexes for Code, Doc, and Notes content types.
//! Enables parallel search across all three indexes without interference.

mod index;
mod search;
mod types;

// Re-export all public types for backward compatibility
pub use index::TripleSearchIndex;
pub use types::{
	TripleIndexStats, TripleLimits,
	TripleSearchResults,
};
