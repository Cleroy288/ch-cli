//! Search and reranking logic for the pipeline.

use std::sync::mpsc;
use std::time::Duration;

use crate::indexer::DocumentType;
use crate::retrieval::context::{ContextConfig, ContextExpander};
use crate::retrieval::daemon::protocol::SearchSpec;
use crate::retrieval::daemon::DaemonClient;
use crate::retrieval::hybrid::HybridSearchResult;
use crate::retrieval::query::fallback_parse;
use crate::retrieval::{RetrievalError, RetrievalResult};

use super::core::RetrievalPipeline;

impl RetrievalPipeline {
	/// Perform hybrid search using SearchSpec with intent-aware boosting
	pub(super) fn search(
		&self,
		spec: &SearchSpec,
		limit: usize,
	) -> RetrievalResult<Vec<HybridSearchResult>> {
		if let Some(ref hybrid) = self.hybrid {
			// use search_with_spec for intent-aware boost
			hybrid.search_with_spec(spec, limit)
		} else {
			// Fallback: no results if hybrid not initialized
			Ok(Vec::new())
		}
	}

	/// Safe query expansion with timeout
	/// Returns fallback SearchSpec if daemon fails or times out
	pub(super) fn expand_query_safe(&self, query: &str) -> SearchSpec {
		let (tx, rx) = mpsc::channel(); // channel for result communication
		let query_clone = query.to_string(); // owned copy for thread

		std::thread::spawn(move || {
			// Create new daemon client in thread (avoids Clone requirement)
			let daemon = DaemonClient::new();
			let result = daemon.expand(query_clone);
			let _ = tx.send(result);
		});

		// Wait with 10 second timeout
		match rx.recv_timeout(Duration::from_secs(10)) {
			Ok(Ok(spec)) => spec,
			Ok(Err(e)) => {
				eprintln!("[pipeline] Query expansion failed: {}, using fallback", e);
				self.fallback_query_expansion(query)
			}
			Err(_) => {
				eprintln!("[pipeline] Query expansion timed out, using fallback");
				self.fallback_query_expansion(query)
			}
		}
	}

	/// Safe reranking with timeout
	/// Returns original results (sorted by RRF) if daemon fails or times out
	pub(super) fn rerank_safe(
		&self,
		query: &str,
		spec: &SearchSpec,
		results: Vec<HybridSearchResult>,
	) -> Vec<HybridSearchResult> {
		if results.is_empty() {
			return results;
		}

		let (tx, rx) = mpsc::channel(); // channel for result communication
		let query_clone = query.to_string(); // owned copy for thread

		// Create documents for reranking (before spawning thread)
		let documents: Vec<String> = results
			.iter()
			.map(|r| {
				format!(
					"{} {} {}",
					r.symbol.kind,
					r.symbol.name,
					r.symbol.signature.as_deref().unwrap_or("")
				)
			})
			.collect();

		std::thread::spawn(move || {
			// Create new daemon client in thread (avoids Clone requirement)
			let daemon = DaemonClient::new();
			let rerank_result = daemon.rerank(query_clone, documents);
			let _ = tx.send(rerank_result);
		});

		// Wait with 15 second timeout (reranking can be slower)
		match rx.recv_timeout(Duration::from_secs(15)) {
			Ok(Ok(scores)) => {
				// Apply scores and sort
				let mut reranked = results;
				for (result, score) in reranked.iter_mut().zip(scores.iter()) {
					// Apply query-aware and intent-aware boosts
					let doc_type = DocumentType::from_path(&result.symbol.location.file);
					let doc_query_boost = doc_type.boost_factor_for_query(query);
					let doc_intent_boost = doc_type.boost_factor_for_intent(&spec.intent);
					let kind_boost = result.symbol.kind.boost_factor_for_intent(&spec.intent);
					let combined_boost = doc_query_boost * doc_intent_boost * kind_boost;
					result.rerank_score = Some(*score * combined_boost);
				}
				reranked.sort_by(|a, b| {
					b.rerank_score
						.partial_cmp(&a.rerank_score)
						.unwrap_or(std::cmp::Ordering::Equal)
				});
				reranked
			}
			Ok(Err(e)) => {
				eprintln!("[pipeline] Reranking failed: {}, using RRF scores", e);
				results // Already sorted by RRF
			}
			Err(_) => {
				eprintln!("[pipeline] Reranking timed out, using RRF scores");
				results // Already sorted by RRF
			}
		}
	}

	/// Fallback query expansion when daemon is unavailable
	/// Uses fast-path parser for basic symbol extraction
	#[doc(hidden)]
	pub fn fallback_query_expansion(
		&self,
		query: &str,
	) -> SearchSpec {
		fallback_parse(query)
	}
}
