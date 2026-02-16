//! HybridSearch Implementation
//!
//! Core methods for HybridSearch: construction, configuration,
//! and search operations.

use std::path::Path;

use crate::indexer::{SearchIndex, Symbol};
use crate::retrieval::daemon::DaemonClient;
use crate::retrieval::hybrid::{
	HybridSearch, HybridSearchConfig,
};
use crate::retrieval::{RetrievalError, RetrievalResult};
use crate::retrieval::hybrid::{indexing, VectorStore};

impl HybridSearch {
	/// Create a new hybrid search with in-memory indices
	pub fn new() -> RetrievalResult<Self> {
		let keyword_index = SearchIndex::in_memory()
			.map_err(|err| {
				RetrievalError::Embedding(err.to_string())
			})?;
		let vector_store = VectorStore::new();
		let daemon_client = DaemonClient::new();

		Ok(Self {
			keyword_index,
			vector_store,
			daemon_client,
			config: HybridSearchConfig::default(),
			enriched_store: None,
		})
	}

	/// Create hybrid search with persistence
	pub fn with_paths(
		tantivy_path: impl AsRef<Path>,
		vector_path: impl AsRef<Path>,
	) -> RetrievalResult<Self> {
		let keyword_index = SearchIndex::open_or_create(
			tantivy_path.as_ref(),
		)
		.map_err(|err| {
			RetrievalError::Embedding(err.to_string())
		})?;
		let vector_store = VectorStore::with_path(vector_path)?;
		let daemon_client = DaemonClient::new();

		Ok(Self {
			keyword_index,
			vector_store,
			daemon_client,
			config: HybridSearchConfig::default(),
			enriched_store: None,
		})
	}

	/// Set configuration
	pub fn with_config(mut self, config: HybridSearchConfig) -> Self {
		self.config = config;
		self
	}

	/// Index symbols for both keyword and semantic search
	pub fn index_symbols(&mut self, symbols: &[Symbol]) -> RetrievalResult<()> {
		indexing::index_symbols(
			&mut self.keyword_index,
			&mut self.vector_store,
			&self.daemon_client,
			symbols,
		)
	}

	/// Persist indices to disk
	pub fn persist(&self) -> RetrievalResult<()> {
		indexing::persist(&self.vector_store)?;
		if let Some(ref enriched) = self.enriched_store {
			enriched.persist()?;
		}
		Ok(())
	}
}
