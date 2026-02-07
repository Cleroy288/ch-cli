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
		.map_err(super::semantic_impl::map_retrieval_err)?;

	if scores.len() != results.len() {
		return Err(SearchError::Io(
			std::io::Error::new(
				std::io::ErrorKind::Other,
				format!(
					"Rerank mismatch: {} scores, \
					{} results",
					scores.len(),
					results.len()
				),
			),
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
		.map(|r| {
			format!(
				"{} {} {}",
				r.symbol.kind,
				r.symbol.name,
				r.symbol
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
	results.sort_by(|a, b| {
		b.rerank_score
			.partial_cmp(&a.rerank_score)
			.unwrap_or(std::cmp::Ordering::Equal)
	});
}

/// Build context XML if context expansion requested
pub(super) fn build_context_xml(
	query: &str,
	results: &[HybridSearchResult],
	graph: &Option<SemanticGraph>,
	context: bool,
) -> Option<String> {
	if !context {
		return None;
	}
	let graph = graph.as_ref()?;
	let symbols: Vec<_> =
		results.iter().map(|r| r.symbol.clone()).collect();
	let config = ContextConfig::default();
	let expander =
		ContextExpander::with_config(graph, config, 8000);
	Some(expander.expand_to_xml(&symbols))
}

/// Convert HybridSearchResults to SearchResultHits
pub(super) fn convert_to_hits(
	results: Vec<HybridSearchResult>,
) -> Vec<SearchResultHit> {
	results
		.into_iter()
		.map(|r| SearchResultHit {
			symbol: r.symbol,
			score: r.rrf_score as f64,
			keyword_rank: r.keyword_rank,
			semantic_rank: r.semantic_rank,
			rerank_score: r.rerank_score.map(|s| {
				s as f64
			}),
		})
		.collect()
}
