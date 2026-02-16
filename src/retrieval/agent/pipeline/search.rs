//! Search and reranking logic for the pipeline.

use std::io::Write;
use std::sync::mpsc;
use std::time::Duration;

use crate::indexer::DocumentType;
use crate::retrieval::daemon::protocol::SearchSpec;
use crate::retrieval::daemon::DaemonClient;
use crate::retrieval::hybrid::HybridSearchResult;
use crate::retrieval::query::fallback_parse;
use crate::retrieval::RetrievalResult;

use super::core::RetrievalPipeline;

impl RetrievalPipeline {
	/// Perform hybrid search using SearchSpec
	pub(super) fn search(
		&self,
		spec: &SearchSpec,
		limit: usize,
	) -> RetrievalResult<Vec<HybridSearchResult>> {
		if let Some(ref hybrid) = self.hybrid {
			hybrid.search_with_spec(spec, limit)
		} else {
			Ok(Vec::new())
		}
	}

	/// Safe query expansion with timeout.
	/// Returns fallback SearchSpec if daemon fails.
	pub(super) fn expand_query_safe(
		&self,
		query: &str,
	) -> SearchSpec {
		let result =
			spawn_daemon_expand(query);
		let timeout = Duration::from_secs(10);

		match result.recv_timeout(timeout) {
			Ok(Ok(spec)) => spec,
			Ok(Err(err)) => {
				let _ = writeln!(
					std::io::stderr().lock(),
					"[pipeline] Expansion \
					failed: {}, fallback",
					err
				);
				self.fallback_query_expansion(query)
			}
			Err(_) => {
				let _ = writeln!(
					std::io::stderr().lock(),
					"[pipeline] Expansion timed out"
				);
				self.fallback_query_expansion(query)
			}
		}
	}

	/// Safe reranking with timeout.
	/// Returns original results if daemon fails.
	pub(super) fn rerank_safe(
		&self,
		query: &str,
		spec: &SearchSpec,
		results: Vec<HybridSearchResult>,
	) -> Vec<HybridSearchResult> {
		if results.is_empty() {
			return results;
		}

		let documents = build_rerank_documents(&results);
		let scores = fetch_rerank_scores(
			query, documents,
		);

		match scores {
			Some(scores) => {
				apply_rerank_scores(
					query, spec, results, &scores,
				)
			}
			None => results,
		}
	}

	/// Fallback query expansion when daemon unavailable
	#[doc(hidden)]
	pub fn fallback_query_expansion(
		&self,
		query: &str,
	) -> SearchSpec {
		fallback_parse(query)
	}
}

/// Spawn daemon expand query on background thread
fn spawn_daemon_expand(
	query: &str,
) -> mpsc::Receiver<RetrievalResult<SearchSpec>> {
	let (sender, receiver) = mpsc::channel();
	let query_clone = query.to_string();

	std::thread::spawn(move || {
		let daemon = DaemonClient::new();
		let result = daemon.expand(query_clone);
		let _ = sender.send(result);
	});
	receiver
}

/// Build document strings for reranking
fn build_rerank_documents(
	results: &[HybridSearchResult],
) -> Vec<String> {
	results
		.iter()
		.map(|res| {
			format!(
				"{} {} {}",
				res.symbol.kind,
				res.symbol.name,
				res.symbol
					.signature
					.as_deref()
					.unwrap_or("")
			)
		})
		.collect()
}

/// Fetch rerank scores from daemon with timeout
fn fetch_rerank_scores(
	query: &str,
	documents: Vec<String>,
) -> Option<Vec<f32>> {
	let receiver =
		spawn_daemon_rerank(query, documents);
	let timeout = Duration::from_secs(15);

	match receiver.recv_timeout(timeout) {
		Ok(Ok(scores)) => Some(scores),
		Ok(Err(err)) => {
			let _ = writeln!(
				std::io::stderr().lock(),
				"[pipeline] Reranking failed: {}",
				err
			);
			None
		}
		Err(_) => {
			let _ = writeln!(
				std::io::stderr().lock(),
				"[pipeline] Reranking timed out"
			);
			None
		}
	}
}

/// Spawn daemon rerank on background thread
fn spawn_daemon_rerank(
	query: &str,
	documents: Vec<String>,
) -> mpsc::Receiver<RetrievalResult<Vec<f32>>> {
	let (sender, receiver) = mpsc::channel();
	let query_clone = query.to_string();

	std::thread::spawn(move || {
		let daemon = DaemonClient::new();
		let result =
			daemon.rerank(query_clone, documents);
		let _ = sender.send(result);
	});
	receiver
}

/// Apply rerank scores with intent-aware boosting
fn apply_rerank_scores(
	query: &str,
	spec: &SearchSpec,
	mut results: Vec<HybridSearchResult>,
	scores: &[f32],
) -> Vec<HybridSearchResult> {
	for (result, score) in
		results.iter_mut().zip(scores.iter())
	{
		let doc_type = DocumentType::from_path(
			&result.symbol.location.file,
		);
		let query_boost =
			doc_type.boost_factor_for_query(query);
		let intent_boost =
			doc_type.boost_factor_for_intent(&spec.intent);
		let kind_boost = result
			.symbol
			.kind
			.boost_factor_for_intent(&spec.intent);
		let combined =
			query_boost * intent_boost * kind_boost;
		result.rerank_score = Some(*score * combined);
	}

	results.sort_by(|lhs, rhs| {
		rhs.rerank_score
			.partial_cmp(&lhs.rerank_score)
			.unwrap_or(std::cmp::Ordering::Equal)
	});
	results
}
