//! Daemon IPC Protocol
//!
//! Defines the message types for communication between
//! the CLI client and the model daemon over Unix sockets.

use serde::{Deserialize, Serialize};

// Re-export types from protocol_types for backward compat
pub use super::protocol_types::{
	CachedSearchResult, DaemonStatus, DeviceStatus,
	DocEntryResponse, FileFilter, QueryIntent, SearchSpec,
};

/// Request types sent from client to daemon
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum DaemonRequest {
	/// Generate embeddings for a list of texts
	Embed {
		/// text chunks to embed
		texts: Vec<String>,
	},
	/// Rerank documents given a query
	Rerank {
		/// the search query
		query: String,
		/// documents to rerank
		documents: Vec<String>,
	},
	/// Expand a natural language query
	Expand {
		/// natural language query
		query: String,
	},
	/// Get daemon status
	Status,
	/// Shutdown the daemon
	Shutdown,
	/// Ping to check if daemon is alive
	Ping,
	/// Index a project and cache it in daemon memory
	IndexProject {
		/// project root path
		project_path: String,
		/// force re-index even if cached
		force: bool,
	},
	/// Search a cached project
	SearchProject {
		/// project root path
		project_path: String,
		/// search query
		query: String,
		/// max results
		limit: usize,
	},
	/// Get project cache status
	ProjectStatus {
		/// project root path
		project_path: String,
	},
	/// Evict a project from cache
	EvictProject {
		/// project root path
		project_path: String,
	},
	/// Start documentation generation for a project
	StartDocGen {
		/// project root path
		project_path: String,
		/// force regeneration even if docs exist
		force: bool,
	},
	/// Get documentation generation status
	DocGenStatus {
		/// project root path
		project_path: String,
	},
	/// Get documentation for a specific symbol
	GetDoc {
		/// project root path
		project_path: String,
		/// symbol name to look up
		symbol_name: String,
	},
	/// Get doc for a symbol in a specific file
	GetDocByFile {
		/// project root path
		project_path: String,
		/// file path relative to project root
		file_path: String,
		/// symbol name (leaf only, e.g. "add")
		symbol_name: String,
	},
	/// Search documentation
	SearchDocs {
		/// project root path
		project_path: String,
		/// search query
		query: String,
		/// max results
		limit: usize,
	},
}

/// Response types sent from daemon to client
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum DaemonResponse {
	/// Embedding vectors (384 dims for bge-small)
	Embeddings(Vec<Vec<f32>>),
	/// Relevance scores for reranking
	Scores(Vec<f32>),
	/// Expanded search specification
	SearchSpec(SearchSpec),
	/// Daemon status information
	Status(DaemonStatus),
	/// Operation succeeded with no data
	Ok,
	/// Pong response to ping
	Pong,
	/// Error message
	Error(String),
	/// Project indexed successfully
	ProjectIndexed {
		/// number of symbols indexed
		symbol_count: usize,
		/// whether result came from cache
		cached: bool,
		/// indexing time in milliseconds
		index_time_ms: u64,
	},
	/// Search results from cached project
	SearchResults(Vec<CachedSearchResult>),
	/// Project cache status
	ProjectCacheStatus {
		/// whether project is cached
		cached: bool,
		/// number of symbols if cached
		symbol_count: usize,
		/// last indexed time (unix timestamp)
		last_indexed: u64,
	},
	/// Documentation generation status
	DocGenStatus {
		/// total number of entries
		total: usize,
		/// completed entries
		completed: usize,
		/// pending entries
		pending: usize,
		/// whether generation is complete
		is_ready: bool,
		/// whether generation is currently running
		in_progress: bool,
	},
	/// Single documentation entry
	Doc(Option<DocEntryResponse>),
	/// Documentation search results
	DocResults(Vec<DocEntryResponse>),
}

/// Serialize a request to JSON bytes with newline delimiter
pub fn serialize_request(
	request: &DaemonRequest,
) -> Result<Vec<u8>, serde_json::Error> {
	super::protocol_serde::serialize_request(request)
}

/// Deserialize a request from JSON bytes
pub fn deserialize_request(
	bytes: &[u8],
) -> Result<DaemonRequest, serde_json::Error> {
	super::protocol_serde::deserialize_request(bytes)
}

/// Serialize a response to JSON bytes with newline
pub fn serialize_response(
	response: &DaemonResponse,
) -> Result<Vec<u8>, serde_json::Error> {
	super::protocol_serde::serialize_response(response)
}

/// Deserialize a response from JSON bytes
pub fn deserialize_response(
	bytes: &[u8],
) -> Result<DaemonResponse, serde_json::Error> {
	super::protocol_serde::deserialize_response(bytes)
}

