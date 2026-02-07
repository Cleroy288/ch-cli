//! Triple Hybrid Search — State and Persistence
//!
//! Persistence, stats, and state query operations for the
//! triple hybrid search system.

use crate::retrieval::hybrid::triple::{
	TripleHybridSearch, TripleHybridStats,
};
use crate::retrieval::RetrievalResult;

impl TripleHybridSearch {
	/// Persist all indexes and stores
	pub fn persist(&self) -> RetrievalResult<()> {
		self.vector_store.persist()
	}

	/// Get stats
	pub fn stats(&self) -> TripleHybridStats {
		let vector_stats = self.vector_store.stats();
		TripleHybridStats {
			code_keyword_count: 0,
			code_vector_count: vector_stats.code_count,
			doc_keyword_count: 0,
			doc_vector_count: vector_stats.doc_count,
			notes_keyword_count: 0,
			notes_vector_count: vector_stats.notes_count,
		}
	}

	/// Check if empty
	pub fn is_empty(&self) -> bool {
		self.symbols.is_empty()
	}
}
