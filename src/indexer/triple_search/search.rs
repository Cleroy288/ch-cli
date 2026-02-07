//! Triple Search Operations
//!
//! Contains search methods for code, doc, notes, and parallel/sequential search.

use crate::indexer::search::{SearchHit, SearchResult};

use super::index::TripleSearchIndex;
use super::types::TripleSearchResults;

impl TripleSearchIndex {
	/// Search code index only
	pub fn search_code(&self, query: &str, limit: usize) -> SearchResult<Vec<SearchHit>> {
		self.code_index.search(query, limit)
	}

	/// Search doc index only
	pub fn search_docs(&self, query: &str, limit: usize) -> SearchResult<Vec<SearchHit>> {
		self.doc_index.search(query, limit)
	}

	/// Search notes index only
	pub fn search_notes(&self, query: &str, limit: usize) -> SearchResult<Vec<SearchHit>> {
		self.notes_index.search(query, limit)
	}

	/// Parallel search all three indexes using rayon
	pub fn search_parallel(
		&self,
		query: &str,
		code_limit: usize,
		doc_limit: usize,
		notes_limit: usize,
	) -> SearchResult<TripleSearchResults> {
		// Use rayon::join for parallel execution
		let ((code_result, doc_result), notes_result) = rayon::join(
			|| {
				rayon::join(
					|| self.search_code(query, code_limit),
					|| self.search_docs(query, doc_limit),
				)
			},
			|| self.search_notes(query, notes_limit),
		);

		// Propagate any errors
		let code_results = code_result?;
		let doc_results = doc_result?;
		let notes_results = notes_result?;

		Ok(TripleSearchResults {
			code_results,
			doc_results,
			notes_results,
		})
	}

	/// Sequential search all three indexes (fallback if rayon unavailable)
	pub fn search_sequential(
		&self,
		query: &str,
		code_limit: usize,
		doc_limit: usize,
		notes_limit: usize,
	) -> SearchResult<TripleSearchResults> {
		let code_results = self.search_code(query, code_limit)?;
		let doc_results = self.search_docs(query, doc_limit)?;
		let notes_results = self.search_notes(query, notes_limit)?;

		Ok(TripleSearchResults {
			code_results,
			doc_results,
			notes_results,
		})
	}
}
