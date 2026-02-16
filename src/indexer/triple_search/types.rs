//! Triple Search Types
//!
//! Contains data structures for triple index statistics and search results.

use crate::indexer::search::SearchHit;

/// Stats from triple indexing operation
#[derive(Debug, Clone, Default)]
pub struct TripleIndexStats {
	/// number of code symbols indexed
	pub code_count: usize,
	/// number of doc symbols indexed
	pub doc_count: usize,
	/// number of notes symbols indexed
	pub notes_count: usize,
}

impl TripleIndexStats {
	/// Get total symbols indexed
	pub fn total(&self) -> usize {
		self.code_count + self.doc_count + self.notes_count
	}
}

/// Per-index limits for triple search
#[derive(Debug, Clone, Copy)]
pub struct TripleLimits {
	/// max results from code index
	pub code: usize,
	/// max results from doc index
	pub doc: usize,
	/// max results from notes index
	pub notes: usize,
}

impl TripleLimits {
	/// All limits set to the same value
	pub fn uniform(limit: usize) -> Self {
		Self {
			code: limit,
			doc: limit,
			notes: limit,
		}
	}
}

/// Results from triple search operation
#[derive(Debug, Default)]
pub struct TripleSearchResults {
	/// search results from code index
	pub code_results: Vec<SearchHit>,
	/// search results from doc index
	pub doc_results: Vec<SearchHit>,
	/// search results from notes index
	pub notes_results: Vec<SearchHit>,
}
