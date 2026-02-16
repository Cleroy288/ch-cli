//! Search functions for hybrid search
//!
//! Contains search, search_adaptive, search_with_spec,
//! and search_with_weights functions that perform
//! hybrid keyword+semantic search.

use std::collections::HashMap;

use crate::indexer::SearchIndex;
use crate::retrieval::daemon::protocol::SearchSpec;
use crate::retrieval::daemon::DaemonClient;
use crate::retrieval::hybrid::adaptive::{
	self, AdaptiveWeights,
};
use crate::retrieval::hybrid::config::HybridSearchConfig;
use crate::retrieval::hybrid::converters::{
	to_keyword_ranked, to_semantic_ranked,
};
use crate::retrieval::hybrid::fusion_core::{
	fuse_with_weights, FusionParams,
};
use crate::retrieval::hybrid::fusion_intent::{
	fuse_with_weights_and_intent, IntentContext,
};
use crate::retrieval::hybrid::result::HybridSearchResult;
use crate::retrieval::hybrid::vector_store::VectorStore;
use crate::retrieval::{RetrievalError, RetrievalResult};

use super::fusion::RankedItem;
use super::vector_store::SearchResult as VecResult;

/// Ranked keyword+semantic results pair
type RankedPair = (
	Vec<RankedItem<crate::indexer::SearchHit>>,
	Vec<RankedItem<VecResult>>,
);

/// Bundled references for hybrid search operations
pub struct SearchContext<'ctx> {
	/// keyword search index
	pub keyword_index: &'ctx SearchIndex,
	/// vector store for semantic search
	pub vector_store: &'ctx VectorStore,
	/// daemon client for embeddings
	pub daemon_client: &'ctx DaemonClient,
	/// hybrid search configuration
	pub config: &'ctx HybridSearchConfig,
}

/// Perform hybrid search with fixed weights
pub fn search<'ctx>(
	ctx: &SearchContext<'ctx>,
	query: &str,
	limit: usize,
) -> RetrievalResult<Vec<HybridSearchResult>> {
	let weights = AdaptiveWeights {
		keyword_weight: ctx.config.keyword_weight,
		semantic_weight: ctx.config.semantic_weight,
	};
	search_with_weights(ctx, query, limit, &weights)
}

/// Perform hybrid search with adaptive weights.
/// Favors BM25 for symbol lookups, semantic for concepts.
pub fn search_adaptive<'ctx>(
	ctx: &SearchContext<'ctx>,
	query: &str,
	limit: usize,
) -> RetrievalResult<Vec<HybridSearchResult>> {
	let weights = adaptive::compute_weights(query);
	search_with_weights(ctx, query, limit, &weights)
}

/// Perform hybrid search using a SearchSpec.
/// Uses intent to adjust symbol kind boosts.
pub fn search_with_spec<'ctx>(
	ctx: &SearchContext<'ctx>,
	spec: &SearchSpec,
	limit: usize,
) -> RetrievalResult<Vec<HybridSearchResult>> {
	let weights = adaptive::compute_weights(
		&spec.original_query,
	);
	let query = &spec.original_query;
	let (kw_ranked, sem_ranked) =
		run_both_searches(ctx, query)?;

	let empty_rc: HashMap<String, usize> =
		HashMap::new();
	let intent_ctx = IntentContext {
		intent: &spec.intent,
		ref_counts: &empty_rc,
	};
	let params = FusionParams {
		keyword_weight: weights.keyword_weight,
		semantic_weight: weights.semantic_weight,
		query,
	};
	let fused = fuse_with_weights_and_intent(
		kw_ranked,
		sem_ranked,
		&params,
		&intent_ctx,
	);

	Ok(fused.into_iter().take(limit).collect())
}

/// Perform hybrid search with custom weights
pub fn search_with_weights<'ctx>(
	ctx: &SearchContext<'ctx>,
	query: &str,
	limit: usize,
	weights: &AdaptiveWeights,
) -> RetrievalResult<Vec<HybridSearchResult>> {
	let (kw_ranked, sem_ranked) =
		run_both_searches(ctx, query)?;

	let params = FusionParams {
		keyword_weight: weights.keyword_weight,
		semantic_weight: weights.semantic_weight,
		query,
	};
	let fused = fuse_with_weights(
		kw_ranked, sem_ranked, &params,
	);

	Ok(fused.into_iter().take(limit).collect())
}

/// Run keyword and semantic searches, return ranked items
fn run_both_searches<'ctx>(
	ctx: &SearchContext<'ctx>,
	query: &str,
) -> RetrievalResult<RankedPair> {
	let candidates = ctx.config.candidates_per_source;
	let keyword_hits = ctx
		.keyword_index
		.search(query, candidates)
		.map_err(|err| {
			RetrievalError::Embedding(err.to_string())
		})?;

	let embeddings = ctx
		.daemon_client
		.embed(vec![query.to_string()])?;
	let query_emb =
		embeddings.first().ok_or_else(|| {
			RetrievalError::Embedding(
				"no embedding returned".to_string(),
			)
		})?;
	let semantic_hits =
		ctx.vector_store.search(query_emb, candidates);

	Ok((
		to_keyword_ranked(keyword_hits),
		to_semantic_ranked(semantic_hits),
	))
}
