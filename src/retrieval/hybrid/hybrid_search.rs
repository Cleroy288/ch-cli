//! HybridSearch Search Operations
//!
//! Search methods for hybrid keyword+semantic search with
//! fixed, adaptive, and spec-based weights.

use crate::indexer::SearchIndex;
use crate::retrieval::daemon::protocol::SearchSpec;
use crate::retrieval::daemon::DaemonClient;
use crate::retrieval::hybrid::{
	HybridSearch, HybridSearchConfig, HybridSearchResult, VectorStore, search,
};
use crate::retrieval::RetrievalResult;

impl HybridSearch {
	/// Perform hybrid search with fixed weights
	pub fn search(
		&self,
		query: &str,
		limit: usize,
	) -> RetrievalResult<Vec<HybridSearchResult>> {
		search::search(
			&self.keyword_index,
			&self.vector_store,
			&self.daemon_client,
			&self.config,
			query,
			limit,
		)
	}

	/// Perform hybrid search with adaptive weights.
	/// Favors BM25 for symbol lookups, semantic for concepts.
	pub fn search_adaptive(
		&self,
		query: &str,
		limit: usize,
	) -> RetrievalResult<Vec<HybridSearchResult>> {
		search::search_adaptive(
			&self.keyword_index,
			&self.vector_store,
			&self.daemon_client,
			&self.config,
			query,
			limit,
		)
	}

	/// Perform hybrid search using a SearchSpec from query expansion.
	/// Uses intent to adjust symbol kind boosts
	/// (e.g., deprioritize fields for Understand).
	pub fn search_with_spec(
		&self,
		spec: &SearchSpec,
		limit: usize,
	) -> RetrievalResult<Vec<HybridSearchResult>> {
		search::search_with_spec(
			&self.keyword_index,
			&self.vector_store,
			&self.daemon_client,
			&self.config,
			spec,
			limit,
		)
	}

	/// Get reference to keyword index
	pub fn keyword_index(&self) -> &SearchIndex {
		&self.keyword_index
	}

	/// Get reference to vector store
	pub fn vector_store(&self) -> &VectorStore {
		&self.vector_store
	}
}
