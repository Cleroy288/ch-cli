//! Individual Request Handlers
//!
//! Extracted from client_handler.rs for norm compliance.

use super::ModelDaemon;
use crate::retrieval::daemon::protocol::DaemonResponse;

/// Handle embed request
pub fn handle_embed(
	daemon: &ModelDaemon,
	texts: Vec<String>,
) -> DaemonResponse {
	match &daemon.embedder {
		Some(embedder) => match embedder.embed_batch(&texts) {
			Ok(embeddings) => {
				DaemonResponse::Embeddings(embeddings)
			}
			Err(err) => DaemonResponse::Error(format!(
				"embedding error: {}",
				err
			)),
		},
		None => DaemonResponse::Error(
			"embedding model not loaded".to_string(),
		),
	}
}

/// Handle rerank request
pub fn handle_rerank(
	daemon: &ModelDaemon,
	query: &str,
	documents: Vec<String>,
) -> DaemonResponse {
	match &daemon.reranker {
		Some(reranker) => {
			match reranker.score_batch(query, &documents) {
				Ok(scores) => DaemonResponse::Scores(scores),
				Err(err) => DaemonResponse::Error(format!(
					"reranking error: {}",
					err
				)),
			}
		}
		None => {
			// fallback: return dummy scores based on position
			let scores: Vec<f32> = documents
				.iter()
				.enumerate()
				.map(|(idx, _)| 1.0 / (idx as f32 + 1.0))
				.collect();
			DaemonResponse::Scores(scores)
		}
	}
}

/// Handle expand request (query expansion via LLM)
///
/// Lazy-loads the Phi-3 model on first expand call.
pub fn handle_expand(
	daemon: &mut ModelDaemon,
	query: &str,
) -> DaemonResponse {
	let model_id =
		daemon.config.expansion_model.clone();
	daemon.interpreter.ensure_loaded(&model_id);
	let spec = daemon.interpreter.interpret(query);
	DaemonResponse::SearchSpec(spec)
}
