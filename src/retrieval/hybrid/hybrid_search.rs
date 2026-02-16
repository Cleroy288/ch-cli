//! HybridSearch Search Operations
//!
//! Search methods for hybrid keyword+semantic search with
//! fixed, adaptive, and spec-based weights.

use crate::indexer::SearchIndex;
use crate::retrieval::daemon::protocol::SearchSpec;
use crate::retrieval::hybrid::search::SearchContext;
use crate::retrieval::hybrid::{
	HybridSearch, HybridSearchResult, VectorStore, search,
};
use crate::retrieval::RetrievalResult;

impl HybridSearch {
	/// Build a SearchContext from this instance
	fn search_ctx<'ctx>(
		&'ctx self,
	) -> SearchContext<'ctx> {
		SearchContext {
			keyword_index: &self.keyword_index,
			vector_store: &self.vector_store,
			daemon_client: &self.daemon_client,
			config: &self.config,
		}
	}

	/// Perform hybrid search with fixed weights
	pub fn search(
		&self,
		query: &str,
		limit: usize,
	) -> RetrievalResult<Vec<HybridSearchResult>> {
		let ctx = self.search_ctx();
		let mut results =
			search::search(&ctx, query, limit)?;
		self.apply_enriched_boost(
			&mut results, query,
		)?;
		Ok(results)
	}

	/// Perform hybrid search with adaptive weights.
	/// Favors BM25 for symbol lookups, semantic for
	/// concepts.
	pub fn search_adaptive(
		&self,
		query: &str,
		limit: usize,
	) -> RetrievalResult<Vec<HybridSearchResult>> {
		let ctx = self.search_ctx();
		let mut results = search::search_adaptive(
			&ctx, query, limit,
		)?;
		self.apply_enriched_boost(
			&mut results, query,
		)?;
		Ok(results)
	}

	/// Perform hybrid search using a SearchSpec.
	/// Uses intent to adjust symbol kind boosts.
	pub fn search_with_spec(
		&self,
		spec: &SearchSpec,
		limit: usize,
	) -> RetrievalResult<Vec<HybridSearchResult>> {
		let ctx = self.search_ctx();
		let mut results = search::search_with_spec(
			&ctx, spec, limit,
		)?;
		self.apply_enriched_boost(
			&mut results,
			&spec.original_query,
		)?;
		Ok(results)
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
