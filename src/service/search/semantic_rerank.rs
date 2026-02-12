//! Reranking and context expansion for semantic search.

use crate::domain::errors::search::SearchError;
use crate::indexer::SemanticGraph;
use crate::retrieval::daemon::DaemonClient;
use crate::retrieval::hybrid::HybridSearchResult;
use crate::retrieval::{
	ContextConfig, ContextExpander,
};

use super::types::SearchResultHit;

/// Rerank results using cross-encoder via daemon
pub(super) fn rerank_results(
	query: &str,
	mut results: Vec<HybridSearchResult>,
	limit: usize,
) -> Result<Vec<HybridSearchResult>, SearchError> {
	let documents = build_rerank_docs(&results);
	let client = DaemonClient::new();

	let scores = client
		.rerank(query.to_string(), documents)
		.map_err(
			super::semantic_impl::map_retrieval_err,
		)?;

	if scores.len() != results.len() {
		return Err(SearchError::IoError(
			std::io::Error::other(format!(
				"Rerank mismatch: {} scores, \
				{} results",
				scores.len(),
				results.len()
			)),
		));
	}

	assign_and_sort(&mut results, &scores);
	results.truncate(limit);
	Ok(results)
}

/// Build document strings for reranking
fn build_rerank_docs(
	results: &[HybridSearchResult],
) -> Vec<String> {
	results
		.iter()
		.map(|result| {
			format!(
				"{} {} {}",
				result.symbol.kind,
				result.symbol.name,
				result
					.symbol
					.signature
					.as_deref()
					.unwrap_or("")
			)
		})
		.collect()
}

/// Assign rerank scores and sort descending
fn assign_and_sort(
	results: &mut [HybridSearchResult],
	scores: &[f32],
) {
	for (result, score) in
		results.iter_mut().zip(scores.iter())
	{
		result.rerank_score = Some(*score);
	}
	results.sort_by(|lhs, rhs| {
		rhs.rerank_score
			.partial_cmp(&lhs.rerank_score)
			.unwrap_or(std::cmp::Ordering::Equal)
	});
}

/// Build context XML if context expansion requested
pub(super) fn build_context_xml(
	_query: &str,
	results: &[HybridSearchResult],
	graph: &Option<std::sync::Arc<SemanticGraph>>,
	context: bool,
) -> Option<String> {
	if !context {
		return None;
	}
	let graph = graph.as_deref()?;
	let symbols: Vec<_> = results
		.iter()
		.map(|result| (*result.symbol).clone())
		.collect();
	let config = ContextConfig::default();
	let expander = ContextExpander::with_config(
		graph, config, 8000,
	);
	Some(expander.expand_to_xml(&symbols))
}

/// Convert HybridSearchResults to SearchResultHits
pub(super) fn convert_to_hits(
	results: Vec<HybridSearchResult>,
) -> Vec<SearchResultHit> {
	results
		.into_iter()
		.map(|result| SearchResultHit {
			symbol: (*result.symbol).clone(),
			score: result.score as f64,
			keyword_rank: result.keyword_rank,
			semantic_rank: result.semantic_rank,
			rerank_score: result
				.rerank_score
				.map(|score| score as f64),
		})
		.collect()
}
