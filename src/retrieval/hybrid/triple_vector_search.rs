//! Triple Vector Store — Search Operations
//!
//! Parallel semantic search across code, notes, and
//! enriched vector stores. Doc pipeline uses keyword-only
//! search (no vector store).

use crate::indexer::triple_search::TripleLimits;
use crate::retrieval::hybrid::triple_vector_store::{
	TripleVectorResults, TripleVectorStore,
};
use crate::retrieval::hybrid::vector_store::SearchResult;

/// Pair of search result vectors from two stores
type SearchPair =
	(Vec<SearchResult>, Vec<SearchResult>);

impl TripleVectorStore {
	/// Search code vectors only
	pub fn search_code(
		&self,
		query: &[f32],
		top_k: usize,
	) -> Vec<SearchResult> {
		self.code_store.search(query, top_k)
	}

	/// Doc pipeline uses keyword-only search.
	/// Returns empty — no doc vectors are stored.
	pub fn search_docs(
		&self,
		_query: &[f32],
		_top_k: usize,
	) -> Vec<SearchResult> {
		Vec::new()
	}

	/// Search notes vectors only
	pub fn search_notes(
		&self,
		query: &[f32],
		top_k: usize,
	) -> Vec<SearchResult> {
		self.notes_store.search(query, top_k)
	}

	/// Search enriched (LLM doc) vectors only
	pub fn search_enriched(
		&self,
		query: &[f32],
		top_k: usize,
	) -> Vec<SearchResult> {
		self.enriched_store.search(query, top_k)
	}
}

impl TripleVectorStore {
	/// Parallel search code, notes, enriched stores.
	/// Doc store is skipped (keyword-only pipeline).
	pub fn search_parallel(
		&self,
		query: &[f32],
		limits: &TripleLimits,
	) -> TripleVectorResults {
		let (code, right) = rayon::join(
			|| self.search_code(query, limits.code),
			|| self.search_right_pair(query, limits),
		);
		TripleVectorResults {
			code_results: code,
			doc_results: Vec::new(),
			notes_results: right.0,
			enriched_results: right.1,
		}
	}

	/// Search notes + enriched stores in parallel
	fn search_right_pair(
		&self,
		query: &[f32],
		limits: &TripleLimits,
	) -> SearchPair {
		rayon::join(
			|| self.search_notes(query, limits.notes),
			|| self.search_enriched(query, limits.code),
		)
	}
}

impl TripleVectorStore {
	/// Sequential search all stores (fallback).
	/// Doc store returns empty (keyword-only pipeline).
	pub fn search_sequential(
		&self,
		query: &[f32],
		limits: &TripleLimits,
	) -> TripleVectorResults {
		TripleVectorResults {
			code_results: self.search_code(
				query, limits.code,
			),
			doc_results: Vec::new(),
			notes_results: self.search_notes(
				query, limits.notes,
			),
			enriched_results: self.search_enriched(
				query, limits.code,
			),
		}
	}
}
