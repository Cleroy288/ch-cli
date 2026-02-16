//! HybridSearch — Enriched NL Store Methods
//!
//! Builder and search methods for the enriched vector store
//! that holds NL-focused embeddings from LLM-generated docs.

use std::path::Path;

use crate::indexer::Symbol;
use crate::retrieval::docgen::DocStore;
use crate::retrieval::hybrid::{
	HybridSearch, HybridSearchResult, VectorStore,
};
use crate::retrieval::hybrid::{
	fusion_enriched, indexing_enriched,
};
use crate::retrieval::RetrievalResult;

impl HybridSearch {
	/// Load enriched vectors from a persisted path.
	/// Returns empty store if file doesn't exist.
	pub fn with_enriched_path(
		mut self,
		path: impl AsRef<Path>,
	) -> RetrievalResult<Self> {
		let store = VectorStore::with_path(path)?;
		self.enriched_store = Some(store);
		Ok(self)
	}

	/// Index enriched NL embeddings from DocStore.
	/// Returns count of indexed symbols.
	pub fn index_enriched(
		&mut self,
		symbols: &[Symbol],
		doc_store: &DocStore,
	) -> RetrievalResult<usize> {
		let store = self
			.enriched_store
			.get_or_insert_with(VectorStore::new);
		indexing_enriched::index_enriched(
			store,
			&self.daemon_client,
			symbols,
			doc_store,
		)
	}

	/// Apply enriched boost to existing search results.
	/// Embeds query, searches enriched store, boosts matches.
	pub(crate) fn apply_enriched_boost(
		&self,
		results: &mut [HybridSearchResult],
		query: &str,
	) -> RetrievalResult<()> {
		let store = match &self.enriched_store {
			Some(enriched) if !enriched.is_empty() => {
				enriched
			}
			_ => return Ok(()),
		};

		// Embed query
		let embeddings = self
			.daemon_client
			.embed(vec![query.to_string()])?;
		let query_emb = match embeddings.first() {
			Some(emb) => emb,
			None => return Ok(()),
		};

		// Search enriched store
		let hits = store.search(query_emb, 50);
		fusion_enriched::boost_with_enriched(
			results, &hits,
		);
		Ok(())
	}
}
