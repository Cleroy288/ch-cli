//! Triple Hybrid Search
//!
//! Combines keyword search (Tantivy) with semantic search
//! (embeddings) across three separate pipelines: Code, Doc,
//! and Notes. Uses parallel execution for fast retrieval
//! without content type interference.

use std::collections::HashMap;
use std::path::Path;

use crate::indexer::symbols::Symbol;
use crate::indexer::triple_search::TripleSearchIndex;
use crate::retrieval::daemon::DaemonClient;
use crate::retrieval::hybrid::triple_vector_store::{
	TripleVectorStore,
};
use crate::retrieval::hybrid::{
	HybridSearchConfig, HybridSearchResult,
};
use crate::retrieval::{RetrievalError, RetrievalResult};

/// Stats from triple hybrid indexing
#[derive(Debug, Clone, Default)]
pub struct TripleHybridStats {
	/// code keyword symbols indexed
	pub code_keyword_count: usize,
	/// code vectors indexed
	pub code_vector_count: usize,
	/// doc keyword symbols indexed
	pub doc_keyword_count: usize,
	/// doc vectors indexed
	pub doc_vector_count: usize,
	/// notes keyword symbols indexed
	pub notes_keyword_count: usize,
	/// notes vectors indexed
	pub notes_vector_count: usize,
}

impl TripleHybridStats {
	/// Get total symbols indexed
	pub fn total(&self) -> usize {
		self.code_keyword_count
			+ self.doc_keyword_count
			+ self.notes_keyword_count
	}
}

/// Results from triple hybrid search
#[derive(Debug, Default)]
pub struct TripleHybridResults {
	/// hybrid search results from code pipeline
	pub code_results: Vec<HybridSearchResult>,
	/// hybrid search results from doc pipeline
	pub doc_results: Vec<HybridSearchResult>,
	/// hybrid search results from notes pipeline
	pub notes_results: Vec<HybridSearchResult>,
}

/// Pipeline-level context for fusion and filtering
pub struct PipelineSlice<'slice> {
	/// max results to return
	pub limit: usize,
	/// content type label (e.g. "code", "doc", "notes")
	pub content_type: &'slice str,
}

/// Triple hybrid search with separate code, doc, and notes
/// pipelines. Combines keyword and semantic search with RRF
/// fusion for each pipeline.
pub struct TripleHybridSearch {
	/// triple keyword index (code, doc, notes)
	pub(crate) keyword_index: TripleSearchIndex,
	/// triple vector store (code, doc, notes)
	pub(crate) vector_store: TripleVectorStore,
	/// daemon client for embedding generation
	pub(crate) daemon_client: DaemonClient,
	/// configuration for search
	pub(crate) config: HybridSearchConfig,
	/// symbol lookup for vector result conversion
	pub(crate) symbols: Vec<Symbol>,
	/// pre-computed ref counts for hub penalty
	pub(crate) ref_counts: HashMap<String, usize>,
	/// pre-computed caller context strings
	pub(crate) caller_contexts: HashMap<String, String>,
}

impl TripleHybridSearch {
	/// Create new triple hybrid search (in-memory)
	pub fn new(
		daemon_client: DaemonClient,
	) -> RetrievalResult<Self> {
		let keyword_index = TripleSearchIndex::in_memory()
			.map_err(|err| {
				RetrievalError::Embedding(err.to_string())
			})?;
		let vector_store = TripleVectorStore::new();

		Ok(Self {
			keyword_index,
			vector_store,
			daemon_client,
			config: HybridSearchConfig::default(),
			symbols: Vec::new(),
			ref_counts: HashMap::new(),
			caller_contexts: HashMap::new(),
		})
	}

	/// Create triple hybrid search with persistence
	pub fn with_persistence(
		base_path: &Path,
		daemon_client: DaemonClient,
	) -> RetrievalResult<Self> {
		let keyword_index =
			TripleSearchIndex::open_or_create(base_path)
				.map_err(|err| {
					RetrievalError::Embedding(
						err.to_string(),
					)
				})?;
		let vector_store =
			TripleVectorStore::with_path(base_path)?;

		Ok(Self {
			keyword_index,
			vector_store,
			daemon_client,
			config: HybridSearchConfig::default(),
			symbols: Vec::new(),
			ref_counts: HashMap::new(),
			caller_contexts: HashMap::new(),
		})
	}

	/// Set custom configuration for search
	pub fn with_config(
		mut self,
		config: HybridSearchConfig,
	) -> Self {
		self.config = config;
		self
	}

	/// Set pre-computed ref counts for hub penalty
	pub fn with_ref_counts(
		mut self,
		counts: HashMap<String, usize>,
	) -> Self {
		self.ref_counts = counts;
		self
	}
}

