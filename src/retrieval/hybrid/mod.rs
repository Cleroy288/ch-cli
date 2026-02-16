//! Hybrid Search Module
//!
//! Combines keyword search (Tantivy) with semantic search (embeddings)
//! using min-max normalized Convex Combination for result merging.

pub mod adaptive;
pub mod config;
pub mod converters;
pub mod embedder_bge;
pub mod embedder_jina;
pub mod embedder_trait;
pub mod embedding;
pub mod embedding_keywords;
pub mod embedding_text;
pub mod embedding_version;
pub mod hub_penalty;
pub mod hub_ref_counts;
pub mod embedding_batch;
pub mod embedding_callers;
pub mod embedding_pooling;
pub mod fusion;
pub mod normalize;
pub mod jina_code_alibi;
pub mod jina_code_layers;
pub mod jina_code_model;
pub mod jina_embedding;
pub mod jina_embedding_batch;
pub mod mean_center;
pub mod fusion_core;
pub mod fusion_enriched;
pub mod fusion_intent;
pub mod fusion_intent_builders;
pub mod fusion_processors;
pub mod hybrid_default;
pub mod hybrid_enriched;
pub mod hybrid_impl;
pub mod hybrid_search;
pub mod indexing;
pub mod indexing_enriched;
pub mod result;
pub mod search;
pub mod triple;
pub mod triple_enriched;
pub mod triple_fusion;
pub mod triple_fusion_enriched;
pub mod triple_fusion_filter;
pub mod triple_fusion_helpers;
pub mod triple_fusion_impl;
pub mod triple_indexing;
pub mod triple_indexing_text;
pub mod triple_state;
pub mod triple_vector_helpers;
pub mod triple_vector_ops;
pub mod triple_vector_query;
pub mod triple_vector_search;
pub mod triple_vector_store;
pub mod vector_store;
pub mod vector_store_csls;
pub mod vector_store_load;
pub mod vector_store_ops;
pub mod vector_store_paths;
pub mod vector_store_persist;

pub use adaptive::{compute_weights, AdaptiveWeights, QueryType};
pub use config::HybridSearchConfig;
pub use converters::parse_symbol_kind;
pub use embedder_trait::Embedder;
pub use embedding::BgeEmbedder;
pub use jina_embedding::JinaEmbedder;
pub use fusion::{FusedResult, RankedItem};
pub use result::HybridSearchResult;
pub use triple::{PipelineSlice, TripleHybridResults, TripleHybridSearch, TripleHybridStats};
pub use triple_vector_store::{
	TripleVectorResults, TripleVectorStats, TripleVectorStore,
};
pub use vector_store::{
	PointMeta, SearchResult as VectorSearchResult,
	VectorPoint, VectorStore,
};

use crate::indexer::SearchIndex;
use crate::retrieval::daemon::DaemonClient;

/// Hybrid search combining keyword and semantic search
pub struct HybridSearch {
	/// Tantivy keyword search index
	pub(crate) keyword_index: SearchIndex,
	/// vector store for semantic search
	pub(crate) vector_store: VectorStore,
	/// daemon client for embeddings
	pub(crate) daemon_client: DaemonClient,
	/// configuration
	pub(crate) config: HybridSearchConfig,
	/// NL-enriched vector store from LLM docs
	pub(crate) enriched_store: Option<VectorStore>,
}
