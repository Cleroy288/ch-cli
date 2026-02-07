//! Triple Vector Store — Search Operations
//!
//! Individual and parallel search operations across the
//! code, doc, and notes vector stores.

use crate::retrieval::hybrid::triple_vector_store::{
	TripleVectorResults, TripleVectorStore,
};
use crate::retrieval::hybrid::vector_store::SearchResult;

impl TripleVectorStore {
	/// Search code vectors only
	pub fn search_code(
		&self,
		query: &[f32],
		k: usize,
	) -> Vec<SearchResult> {
		self.code_store.search(query, k)
	}

	/// Search doc vectors only
	pub fn search_docs(
		&self,
		query: &[f32],
		k: usize,
	) -> Vec<SearchResult> {
		self.doc_store.search(query, k)
	}

	/// Search notes vectors only
	pub fn search_notes(
		&self,
		query: &[f32],
		k: usize,
	) -> Vec<SearchResult> {
		self.notes_store.search(query, k)
	}

	/// Parallel search all three stores using rayon
	pub fn search_parallel(
		&self,
		query: &[f32],
		code_k: usize,
		doc_k: usize,
		notes_k: usize,
	) -> TripleVectorResults {
		// Use rayon::join for parallel execution
		let ((code_results, doc_results), notes_results) =
			rayon::join(
				|| {
					rayon::join(
						|| self.search_code(query, code_k),
						|| self.search_docs(query, doc_k),
					)
				},
				|| self.search_notes(query, notes_k),
			);

		TripleVectorResults {
			code_results,
			doc_results,
			notes_results,
		}
	}

	/// Sequential search all three stores (fallback)
	pub fn search_sequential(
		&self,
		query: &[f32],
		code_k: usize,
		doc_k: usize,
		notes_k: usize,
	) -> TripleVectorResults {
		let code_results = self.search_code(query, code_k);
		let doc_results = self.search_docs(query, doc_k);
		let notes_results =
			self.search_notes(query, notes_k);

		TripleVectorResults {
			code_results,
			doc_results,
			notes_results,
		}
	}
}
