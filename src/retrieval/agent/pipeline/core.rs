//! Core RetrievalPipeline struct and basic methods.

use std::sync::Arc;

use crate::indexer::{SemanticGraph, Symbol, TrigramIndex};
use crate::retrieval::daemon::DaemonClient;
use crate::retrieval::docgen::DocStore;
use crate::retrieval::hybrid::{HybridSearch, TripleHybridSearch};
use crate::retrieval::RetrievalResult;

use super::config::PipelineConfig;

/// The main retrieval pipeline orchestrator
pub struct RetrievalPipeline {
	/// configuration
	#[doc(hidden)]
	pub config: PipelineConfig,
	/// daemon client for ML operations
	#[doc(hidden)]
	pub daemon: DaemonClient,
	/// indexed symbols (cached after first run)
	#[doc(hidden)]
	pub symbols: Option<Vec<Symbol>>,
	/// semantic graph (Arc-wrapped for zero-cost sharing)
	#[doc(hidden)]
	pub graph: Option<Arc<SemanticGraph>>,
	/// hybrid search instance (unified pipeline)
	#[doc(hidden)]
	pub hybrid: Option<HybridSearch>,
	/// triple hybrid search (code, doc, notes pipelines)
	#[doc(hidden)]
	pub triple_hybrid: Option<TripleHybridSearch>,
	/// trigram index for fast pre-filtering
	#[doc(hidden)]
	pub trigram_index: Option<TrigramIndex>,
	/// documentation store for enhanced context
	#[doc(hidden)]
	pub doc_store: Option<DocStore>,
}

impl RetrievalPipeline {
	/// Create a new pipeline with default config
	pub fn new() -> Self {
		Self {
			config: PipelineConfig::default(),
			daemon: DaemonClient::new(),
			symbols: None,
			graph: None,
			hybrid: None,
			triple_hybrid: None,
			trigram_index: None,
			doc_store: None,
		}
	}

	/// Create pipeline with custom config
	pub fn with_config(config: PipelineConfig) -> Self {
		Self {
			config,
			daemon: DaemonClient::new(),
			symbols: None,
			graph: None,
			hybrid: None,
			triple_hybrid: None,
			trigram_index: None,
			doc_store: None,
		}
	}
}

impl RetrievalPipeline {
	/// Initialize the pipeline.
	/// Uses persistent caching when enabled for faster starts.
	pub fn initialize(&mut self) -> RetrievalResult<()> {
		// Use local indexing - daemon caching adds complexity
		// without benefit for single-query CLI usage
		super::initialization::initialize_local(self)
	}

	/// Get documentation for a symbol if available
	pub fn get_doc_for_symbol(
		&self,
		symbol_name: &str,
	) -> Option<String> {
		let store = self.doc_store.as_ref()?;
		let entry = store.get_by_name(symbol_name)?;
		Some(entry.combined_doc())
	}

	/// Check if doc store is available and ready
	pub fn has_doc_context(&self) -> bool {
		self.doc_store
			.as_ref()
			.map(|store| store.is_ready())
			.unwrap_or(false)
	}
}

impl Default for RetrievalPipeline {
	fn default() -> Self {
		Self::new()
	}
}

