//! Async Operations for Daemon Client
//!
//! Provides async method implementations.

use std::path::Path;
use std::time::Duration;

use super::protocol::SearchSpec;
use crate::retrieval::RetrievalResult;

/// Ping daemon to check if alive (async)
pub async fn async_ping(
	socket_path: &Path,
	timeout: Duration,
) -> RetrievalResult<bool> {
	super::async_methods::async_ping(socket_path, timeout)
		.await
}

/// Generate embeddings async
pub async fn async_embed(
	socket_path: &Path,
	timeout: Duration,
	texts: Vec<String>,
) -> RetrievalResult<Vec<Vec<f32>>> {
	super::async_methods::async_embed(
		socket_path, timeout, texts,
	)
	.await
}

/// Rerank documents async
pub async fn async_rerank(
	socket_path: &Path,
	timeout: Duration,
	query: String,
	documents: Vec<String>,
) -> RetrievalResult<Vec<f32>> {
	super::async_methods::async_rerank(
		socket_path, timeout, query, documents,
	)
	.await
}

/// Expand query async
pub async fn async_expand(
	socket_path: &Path,
	timeout: Duration,
	query: String,
) -> RetrievalResult<SearchSpec> {
	super::async_methods::async_expand(
		socket_path, timeout, query,
	)
	.await
}
