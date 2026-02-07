//! Hybrid Search Module
//!
//! Combines keyword search (Tantivy) with semantic search (embeddings)
//! using Reciprocal Rank Fusion for result merging.

pub mod adaptive;
pub mod config;
pub mod converters;
pub mod embedding;
pub mod embedding_batch;
pub mod embedding_pooling;
pub mod fusion;
pub mod fusion_core;
pub mod fusion_intent;
pub mod fusion_processors;
pub mod hybrid_default;
pub mod hybrid_impl;
pub mod hybrid_search;
pub mod indexing;
pub mod result;
pub mod search;
pub mod triple;
pub mod triple_fusion;
pub mod triple_fusion_filter;
pub mod triple_fusion_impl;
pub mod triple_indexing;
pub mod triple_state;
pub mod triple_vector_helpers;
pub mod triple_vector_ops;
pub mod triple_vector_query;
pub mod triple_vector_search;
pub mod triple_vector_store;
pub mod vector_store;
pub mod vector_store_load;
pub mod vector_store_ops;
pub mod vector_store_persist;

pub use adaptive::{compute_weights, AdaptiveWeights, QueryType};
pub use config::HybridSearchConfig;
pub use converters::parse_symbol_kind;
pub use embedding::BgeEmbedder;
pub use fusion::{FusedResult, RankedItem};
pub use result::HybridSearchResult;
pub use triple::{TripleHybridResults, TripleHybridSearch, TripleHybridStats};
pub use triple_vector_store::{
	TripleVectorResults, TripleVectorStats, TripleVectorStore,
};
pub use vector_store::{
	SearchResult as VectorSearchResult, VectorPoint, VectorStore,
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
}
