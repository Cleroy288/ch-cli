//! Embedding and Reranking Methods
//!
//! Provides embedding and reranking operations
//! for the DaemonClient.

use std::path::Path;
use std::time::Duration;

use crate::retrieval::daemon::protocol::SearchSpec;
use crate::retrieval::RetrievalResult;

use super::core_requests;

/// Generate embeddings for texts
pub fn embed_texts(
	socket_path: &Path,
	timeout: Duration,
	texts: Vec<String>,
) -> RetrievalResult<Vec<Vec<f32>>> {
	core_requests::embed(socket_path, timeout, texts)
}

/// Rerank documents given a query
pub fn rerank_documents(
	socket_path: &Path,
	timeout: Duration,
	query: String,
	documents: Vec<String>,
) -> RetrievalResult<Vec<f32>> {
	core_requests::rerank(
		socket_path,
		timeout,
		query,
		documents,
	)
}

/// Expand query into structured SearchSpec
pub fn expand_query(
	socket_path: &Path,
	timeout: Duration,
	query: String,
) -> RetrievalResult<SearchSpec> {
	core_requests::expand(socket_path, timeout, query)
}
