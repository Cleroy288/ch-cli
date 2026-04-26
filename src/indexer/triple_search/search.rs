use crate::indexer::search::{SearchHit, SearchResult};

use super::index::TripleSearchIndex;
use super::types::{TripleLimits, TripleSearchResults};

impl TripleSearchIndex {
	/// Search code index only
	pub fn search_code(
		&self,
		query: &str,
		limit: usize,
	) -> SearchResult<Vec<SearchHit>> {
		self.code_index.search(query, limit)
	}

	/// Search doc index only
	pub fn search_docs(
		&self,
		query: &str,
		limit: usize,
	) -> SearchResult<Vec<SearchHit>> {
		self.doc_index.search(query, limit)
	}

	/// Search notes index only
	pub fn search_notes(
		&self,
		query: &str,
		limit: usize,
	) -> SearchResult<Vec<SearchHit>> {
		self.notes_index.search(query, limit)
	}

	/// Parallel search all three indexes
	pub fn search_parallel(
		&self,
		query: &str,
		limits: TripleLimits,
	) -> SearchResult<TripleSearchResults> {
		let ((code, doc), notes) = rayon::join(
			|| {
				rayon::join(
					|| self.search_code(query, limits.code),
					|| self.search_docs(query, limits.doc),
				)
			},
			|| self.search_notes(query, limits.notes),
		);

		Ok(TripleSearchResults {
			code_results: code?,
			doc_results: doc?,
			notes_results: notes?,
		})
	}

	/// Sequential search all three indexes
	pub fn search_sequential(
		&self,
		query: &str,
		limits: TripleLimits,
	) -> SearchResult<TripleSearchResults> {
		Ok(TripleSearchResults {
			code_results: self.search_code(
				query, limits.code,
			)?,
			doc_results: self.search_docs(
				query, limits.doc,
			)?,
			notes_results: self.search_notes(
				query, limits.notes,
			)?,
		})
	}
}
