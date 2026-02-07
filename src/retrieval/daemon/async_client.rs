//! Async Daemon Client using Tokio
//!
//! Provides async versions of daemon operations.

use std::path::PathBuf;
use std::time::Duration;

use super::async_ops;
use super::protocol::SearchSpec;
use crate::retrieval::{RetrievalConfig, RetrievalResult};

/// Async daemon client for non-blocking ML operations
pub struct AsyncDaemonClient {
	/// path to the Unix socket
	socket_path: PathBuf,
	/// request timeout duration
	timeout: Duration,
}

impl AsyncDaemonClient {
	/// Create new async client with defaults
	pub fn new() -> Self {
		let config = RetrievalConfig::default();
		Self {
			socket_path: config.socket_path,
			timeout: Duration::from_secs(30),
		}
	}

	/// Create with custom socket path
	pub fn with_socket_path(
		socket_path: PathBuf,
	) -> Self {
		Self {
			socket_path,
			timeout: Duration::from_secs(30),
		}
	}

	/// Set custom timeout (builder pattern)
	pub fn with_timeout(
		mut self,
		timeout: Duration,
	) -> Self {
		self.timeout = timeout;
		self
	}

	/// Get the socket path
	#[doc(hidden)]
	pub fn socket_path(&self) -> &PathBuf {
		&self.socket_path
	}

	/// Get the timeout
	#[doc(hidden)]
	pub fn timeout(&self) -> Duration {
		self.timeout
	}

	/// Ping daemon to check if alive
	pub async fn ping(&self) -> RetrievalResult<bool> {
		async_ops::async_ping(&self.socket_path, self.timeout)
			.await
	}

	/// Generate embeddings async
	pub async fn embed(
		&self,
		texts: Vec<String>,
	) -> RetrievalResult<Vec<Vec<f32>>> {
		async_ops::async_embed(
			&self.socket_path,
			self.timeout,
			texts,
		)
		.await
	}

	/// Rerank documents async
	pub async fn rerank(
		&self,
		query: String,
		documents: Vec<String>,
	) -> RetrievalResult<Vec<f32>> {
		async_ops::async_rerank(
			&self.socket_path,
			self.timeout,
			query,
			documents,
		)
		.await
	}

	/// Expand query async
	pub async fn expand(
		&self,
		query: String,
	) -> RetrievalResult<SearchSpec> {
		async_ops::async_expand(
			&self.socket_path,
			self.timeout,
			query,
		)
		.await
	}
}

impl Default for AsyncDaemonClient {
	fn default() -> Self {
		Self::new()
	}
}

