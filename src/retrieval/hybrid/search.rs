//! Search functions for hybrid search
//!
//! Contains search, search_adaptive, search_with_spec, and search_with_weights
//! functions that perform hybrid keyword+semantic search.

use crate::indexer::SearchIndex;
use crate::retrieval::daemon::protocol::SearchSpec;
use crate::retrieval::daemon::DaemonClient;
use crate::retrieval::hybrid::adaptive;
use crate::retrieval::hybrid::config::HybridSearchConfig;
use crate::retrieval::hybrid::converters::{
	to_keyword_ranked, to_semantic_ranked,
};
use crate::retrieval::hybrid::fusion::RankedItem;
use crate::retrieval::hybrid::fusion_core::fuse_with_weights;
use crate::retrieval::hybrid::fusion_intent::fuse_with_weights_and_intent;
use crate::retrieval::hybrid::result::HybridSearchResult;
use crate::retrieval::hybrid::vector_store::VectorStore;
use crate::retrieval::{RetrievalError, RetrievalResult};

/// Perform hybrid search with fixed weights
pub fn search(
	keyword_index: &SearchIndex,
	vector_store: &VectorStore,
	daemon_client: &DaemonClient,
	config: &HybridSearchConfig,
	query: &str,
	limit: usize,
) -> RetrievalResult<Vec<HybridSearchResult>> {
	search_with_weights(
		keyword_index,
		vector_store,
		daemon_client,
		config,
		query,
		limit,
		config.keyword_weight,
		config.semantic_weight,
	)
}

/// Perform hybrid search with adaptive weights based on query type
/// Favors BM25 for symbol lookups, semantic for conceptual queries
pub fn search_adaptive(
	keyword_index: &SearchIndex,
	vector_store: &VectorStore,
	daemon_client: &DaemonClient,
	config: &HybridSearchConfig,
	query: &str,
	limit: usize,
) -> RetrievalResult<Vec<HybridSearchResult>> {
	let weights = adaptive::compute_weights(query);
	search_with_weights(
		keyword_index,
		vector_store,
		daemon_client,
		config,
		query,
		limit,
		weights.keyword_weight,
		weights.semantic_weight,
	)
}

/// Perform hybrid search using a SearchSpec from query expansion.
/// Uses intent to adjust symbol kind boosts (e.g., deprioritize
/// fields for Understand).
pub fn search_with_spec(
	keyword_index: &SearchIndex,
	vector_store: &VectorStore,
	daemon_client: &DaemonClient,
	config: &HybridSearchConfig,
	spec: &SearchSpec,
	limit: usize,
) -> RetrievalResult<Vec<HybridSearchResult>> {
	let weights = adaptive::compute_weights(&spec.original_query);
	let candidates = config.candidates_per_source;
	let query = &spec.original_query;

	// keyword search
	let keyword_hits = keyword_index
		.search(query, candidates)
		.map_err(|e| RetrievalError::Embedding(e.to_string()))?;

	// semantic search - get query embedding
	let query_embeddings = daemon_client.embed(vec![query.to_string()])?;
	let query_embedding = query_embeddings.first().ok_or_else(|| {
		RetrievalError::Embedding("no embedding returned".to_string())
	})?;

	let semantic_hits = vector_store.search(query_embedding, candidates);

	// convert to ranked items
	let keyword_ranked = to_keyword_ranked(keyword_hits);
	let semantic_ranked = to_semantic_ranked(semantic_hits);

	// fuse results with intent-aware weights
	let fused = fuse_with_weights_and_intent(
		config,
		keyword_ranked,
		semantic_ranked,
		weights.keyword_weight,
		weights.semantic_weight,
		query,
		&spec.intent,
	);

	Ok(fused.into_iter().take(limit).collect())
}

/// Perform hybrid search with custom weights
pub fn search_with_weights(
	keyword_index: &SearchIndex,
	vector_store: &VectorStore,
	daemon_client: &DaemonClient,
	config: &HybridSearchConfig,
	query: &str,
	limit: usize,
	keyword_weight: f32,
	semantic_weight: f32,
) -> RetrievalResult<Vec<HybridSearchResult>> {
	let candidates = config.candidates_per_source;

	// keyword search
	let keyword_hits = keyword_index
		.search(query, candidates)
		.map_err(|e| RetrievalError::Embedding(e.to_string()))?;

	// semantic search - get query embedding
	let query_embeddings = daemon_client.embed(vec![query.to_string()])?;
	let query_embedding = query_embeddings.first().ok_or_else(|| {
		RetrievalError::Embedding("no embedding returned".to_string())
	})?;

	let semantic_hits = vector_store.search(query_embedding, candidates);

	// convert to ranked items
	let keyword_ranked: Vec<RankedItem<_>> = to_keyword_ranked(keyword_hits);
	let semantic_ranked: Vec<RankedItem<_>> = to_semantic_ranked(semantic_hits);

	// fuse results with query-aware weights
	let fused = fuse_with_weights(
		config,
		keyword_ranked,
		semantic_ranked,
		keyword_weight,
		semantic_weight,
		query,
	);

	// take top results
	Ok(fused.into_iter().take(limit).collect())
}
