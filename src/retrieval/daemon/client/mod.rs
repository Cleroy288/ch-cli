//! Daemon Client Module
//!
//! Provides a client to communicate with model daemon.

mod connection;
#[doc(hidden)]
pub mod pool_ops;
mod core_requests;
mod doc_gen_requests;
mod doc_requests;
mod embedding_methods;
pub mod pool;
mod pool_getters;
mod project_requests;
pub mod retry;
mod status_methods;
mod status_requests;

use std::path::Path;
use std::time::Duration;

use crate::retrieval::daemon::protocol::{
	CachedSearchResult, DaemonStatus,
	DocEntryResponse, SearchSpec,
};
use crate::retrieval::{RetrievalConfig, RetrievalResult};

/// Index result: (symbol_count, cached, index_time_ms)
type IndexProjectResult =
	RetrievalResult<(usize, bool, u64)>;

/// Status result: (cached, symbol_count, last_indexed)
type ProjectStatusResult =
	RetrievalResult<(bool, usize, u64)>;

// Re-export public types for backward compatibility
pub use doc_requests::DocGenStatus;
pub use pool::{ConnectionPool, PooledConnection};
pub use retry::{is_retryable_error, RetryConfig};

/// Client for communicating with the model daemon
#[derive(Clone)]
pub struct DaemonClient {
	/// path to the Unix socket
	socket_path: std::path::PathBuf,
	/// connection timeout
	timeout: Duration,
}

impl DaemonClient {
	/// Create a new client with default config
	pub fn new() -> Self {
		let config = RetrievalConfig::default();
		Self {
			socket_path: config.socket_path,
			timeout: Duration::from_secs(30),
		}
	}

	/// Create client with custom socket path
	pub fn with_socket_path(
		socket_path: impl AsRef<Path>,
	) -> Self {
		Self {
			socket_path:
				socket_path.as_ref().to_path_buf(),
			timeout: Duration::from_secs(30),
		}
	}

	/// Generate embeddings for texts
	pub fn embed(
		&self,
		texts: Vec<String>,
	) -> RetrievalResult<Vec<Vec<f32>>> {
		embedding_methods::embed_texts(
			&self.socket_path,
			self.timeout,
			texts,
		)
	}

	/// Rerank documents given a query
	pub fn rerank(
		&self,
		query: String,
		documents: Vec<String>,
	) -> RetrievalResult<Vec<f32>> {
		embedding_methods::rerank_documents(
			&self.socket_path,
			self.timeout,
			query,
			documents,
		)
	}

	/// Expand query into SearchSpec
	pub fn expand(
		&self,
		query: String,
	) -> RetrievalResult<SearchSpec> {
		embedding_methods::expand_query(
			&self.socket_path,
			self.timeout,
			query,
		)
	}

	/// Get daemon status
	pub fn status(
		&self,
	) -> RetrievalResult<DaemonStatus> {
		status_methods::get_status(
			&self.socket_path,
			self.timeout,
		)
	}

	/// Ping daemon to check if alive
	pub fn ping(&self) -> RetrievalResult<bool> {
		status_methods::ping_daemon(
			&self.socket_path,
			self.timeout,
		)
	}

	/// Shutdown the daemon
	pub fn shutdown(&self) -> RetrievalResult<()> {
		status_methods::shutdown_daemon(
			&self.socket_path,
			self.timeout,
		)
	}

	/// Index project and cache in daemon
	/// Returns (symbol_count, cached, index_ms)
	pub fn index_project(
		&self,
		project_path: &str,
		force: bool,
	) -> IndexProjectResult {
		project_requests::index_project(
			&self.socket_path,
			self.timeout,
			project_path,
			force,
		)
	}

	/// Search a cached project
	pub fn search_project(
		&self,
		project_path: &str,
		query: &str,
		limit: usize,
	) -> RetrievalResult<Vec<CachedSearchResult>> {
		project_requests::search_project(
			&self.socket_path,
			self.timeout,
			project_requests::ProjectSearchQuery {
				project_path,
				query,
				limit,
			},
		)
	}

	/// Get project cache status
	/// Returns (cached, symbol_count, last_indexed)
	pub fn project_status(
		&self,
		project_path: &str,
	) -> ProjectStatusResult {
		project_requests::project_status(
			&self.socket_path,
			self.timeout,
			project_path,
		)
	}

	/// Evict project from daemon cache
	pub fn evict_project(
		&self,
		project_path: &str,
	) -> RetrievalResult<()> {
		project_requests::evict_project(
			&self.socket_path,
			self.timeout,
			project_path,
		)
	}

	/// Start doc gen for project (async)
	pub fn start_doc_gen(
		&self,
		project_path: String,
		force: bool,
	) -> RetrievalResult<DocGenStatus> {
		doc_requests::start_doc_gen(
			&self.socket_path,
			self.timeout,
			project_path,
			force,
		)
	}

	/// Get doc generation status
	pub fn doc_gen_status(
		&self,
		project_path: String,
	) -> RetrievalResult<DocGenStatus> {
		doc_requests::doc_gen_status(
			&self.socket_path,
			self.timeout,
			project_path,
		)
	}

	/// Get documentation for symbol
	pub fn get_doc(
		&self,
		project_path: String,
		symbol_name: String,
	) -> RetrievalResult<Option<DocEntryResponse>> {
		doc_requests::get_doc(
			&self.socket_path,
			self.timeout,
			project_path,
			symbol_name,
		)
	}

	/// Get doc for a symbol in a specific file
	pub fn get_doc_by_file(
		&self,
		project_path: String,
		file_path: String,
		symbol_name: String,
	) -> RetrievalResult<Option<DocEntryResponse>> {
		doc_requests::get_doc_by_file(
			&self.socket_path,
			self.timeout,
			doc_requests::DocByFileQuery {
				project_path,
				file_path,
				symbol_name,
			},
		)
	}

	/// Search documentation
	pub fn search_docs(
		&self,
		project_path: String,
		query: String,
		limit: usize,
	) -> RetrievalResult<Vec<DocEntryResponse>> {
		doc_requests::search_docs(
			&self.socket_path,
			self.timeout,
			doc_requests::DocSearchQuery {
				project_path,
				query,
				limit,
			},
		)
	}
}

impl Default for DaemonClient {
	fn default() -> Self {
		Self::new()
	}
}
