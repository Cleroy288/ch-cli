//! Daemon IPC Protocol
//!
//! Defines the message types for communication between the CLI client
//! and the model daemon over Unix sockets.

use serde::{Deserialize, Serialize};

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
	},
	/// Single documentation entry
	Doc(Option<DocEntryResponse>),
	/// Documentation search results
	DocResults(Vec<DocEntryResponse>),
}

/// Cached search result (serializable for IPC)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CachedSearchResult {
	/// symbol name
	pub symbol_name: String,
	/// symbol kind (function, struct, etc.)
	pub symbol_kind: String,
	/// file path
	pub file_path: String,
	/// line number
	pub line: usize,
	/// combined RRF score
	pub score: f32,
	/// rerank score if available
	pub rerank_score: Option<f32>,
}

/// Documentation entry for IPC (serializable)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DocEntryResponse {
	/// symbol name
	pub name: String,
	/// symbol kind (function, struct, etc.)
	pub kind: String,
	/// file path
	pub file_path: String,
	/// line number
	pub line: usize,
	/// user-written doc comment
	pub user_comment: Option<String>,
	/// LLM-generated documentation
	pub llm_doc: Option<String>,
	/// function signature (if applicable)
	pub signature: Option<String>,
	/// symbols this one depends on
	pub depends_on: Vec<String>,
	/// symbols that depend on this one
	pub depended_by: Vec<String>,
	/// external crate dependencies
	pub external_deps: Vec<String>,
	/// generation status
	pub status: String,
}

/// Expanded search specification from query expansion
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SearchSpec {
	/// original user query
	pub original_query: String,
	/// extracted symbol names to search for
	pub symbol_names: Vec<String>,
	/// what the user wants to do
	pub intent: QueryIntent,
	/// file filters to apply
	pub file_filters: Vec<FileFilter>,
	/// additional context hints
	pub context_hints: Vec<String>,
}

/// User intent for the query
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum QueryIntent {
	/// find where a symbol is defined
	FindDefinition,
	/// find where a symbol is used
	FindUsages,
	/// understand how something works
	Understand,
	/// modify existing code
	Modify,
	/// debug an issue
	Debug,
	/// general search
	Search,
}

/// File filter for search results
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FileFilter {
	/// glob pattern to match
	pub pattern: String,
	/// true to include, false to exclude
	pub include: bool,
}

/// Information about the compute device (CPU/GPU)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeviceStatus {
	/// device type ("CPU", "Metal", "CUDA")
	pub device_type: String,
	/// human-readable device name
	pub device_name: String,
	/// GPU memory in megabytes (None for CPU)
	pub memory_mb: Option<u64>,
}

impl Default for DeviceStatus {
	fn default() -> Self {
		Self {
			device_type: "CPU".to_string(),
			device_name: "CPU".to_string(),
			memory_mb: None,
		}
	}
}

/// Status information about the daemon
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DaemonStatus {
	/// is the daemon running
	pub running: bool,
	/// daemon process ID
	pub pid: Option<u32>,
	/// which models are loaded
	pub loaded_models: Vec<String>,
	/// compute device information
	pub device: DeviceStatus,
	/// uptime in seconds
	pub uptime_secs: u64,
}

impl Default for DaemonStatus {
	fn default() -> Self {
		Self {
			running: false,
			pid: None,
			loaded_models: Vec::new(),
			device: DeviceStatus::default(),
			uptime_secs: 0,
		}
	}
}

/// Serialize a request to JSON bytes with newline delimiter
pub fn serialize_request(request: &DaemonRequest) -> Result<Vec<u8>, serde_json::Error> {
	let mut bytes = serde_json::to_vec(request)?;
	bytes.push(b'\n'); // delimiter for line-based protocol
	Ok(bytes)
}

/// Deserialize a request from JSON bytes
pub fn deserialize_request(bytes: &[u8]) -> Result<DaemonRequest, serde_json::Error> {
	serde_json::from_slice(bytes)
}

/// Serialize a response to JSON bytes with newline delimiter
pub fn serialize_response(response: &DaemonResponse) -> Result<Vec<u8>, serde_json::Error> {
	let mut bytes = serde_json::to_vec(response)?;
	bytes.push(b'\n'); // delimiter for line-based protocol
	Ok(bytes)
}

/// Deserialize a response from JSON bytes
pub fn deserialize_response(bytes: &[u8]) -> Result<DaemonResponse, serde_json::Error> {
	serde_json::from_slice(bytes)
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test_request_serialization() {
		let req = DaemonRequest::Embed {
			texts: vec!["hello".to_string()],
		};
		let bytes = serialize_request(&req).unwrap();
		let parsed: DaemonRequest = deserialize_request(&bytes[..bytes.len() - 1]).unwrap();

		match parsed {
			DaemonRequest::Embed { texts } => assert_eq!(texts, vec!["hello"]),
			_ => panic!("wrong variant"),
		}
	}

	#[test]
	fn test_response_serialization() {
		let resp = DaemonResponse::Embeddings(vec![vec![0.1, 0.2, 0.3]]);
		let bytes = serialize_response(&resp).unwrap();
		let parsed: DaemonResponse = deserialize_response(&bytes[..bytes.len() - 1]).unwrap();

		match parsed {
			DaemonResponse::Embeddings(vecs) => {
				assert_eq!(vecs.len(), 1);
				assert_eq!(vecs[0], vec![0.1, 0.2, 0.3]);
			}
			_ => panic!("wrong variant"),
		}
	}

	#[test]
	fn test_index_project_request_serialization() {
		let req = DaemonRequest::IndexProject {
			project_path: "/home/user/project".to_string(),
			force: false,
		};
		let bytes = serialize_request(&req).unwrap();
		let parsed: DaemonRequest = deserialize_request(&bytes[..bytes.len() - 1]).unwrap();

		match parsed {
			DaemonRequest::IndexProject { project_path, force } => {
				assert_eq!(project_path, "/home/user/project");
				assert!(!force);
			}
			_ => panic!("wrong variant"),
		}
	}

	#[test]
	fn test_project_indexed_response_serialization() {
		let resp = DaemonResponse::ProjectIndexed {
			symbol_count: 100,
			cached: true,
			index_time_ms: 0,
		};
		let bytes = serialize_response(&resp).unwrap();
		let parsed: DaemonResponse = deserialize_response(&bytes[..bytes.len() - 1]).unwrap();

		match parsed {
			DaemonResponse::ProjectIndexed { symbol_count, cached, index_time_ms } => {
				assert_eq!(symbol_count, 100);
				assert!(cached);
				assert_eq!(index_time_ms, 0);
			}
			_ => panic!("wrong variant"),
		}
	}

	#[test]
	fn test_cached_search_result_serialization() {
		let result = CachedSearchResult {
			symbol_name: "test_function".to_string(),
			symbol_kind: "function".to_string(),
			file_path: "src/main.rs".to_string(),
			line: 42,
			score: 0.95,
			rerank_score: Some(0.87),
		};
		let resp = DaemonResponse::SearchResults(vec![result]);
		let bytes = serialize_response(&resp).unwrap();
		let parsed: DaemonResponse = deserialize_response(&bytes[..bytes.len() - 1]).unwrap();

		match parsed {
			DaemonResponse::SearchResults(results) => {
				assert_eq!(results.len(), 1);
				assert_eq!(results[0].symbol_name, "test_function");
				assert_eq!(results[0].line, 42);
			}
			_ => panic!("wrong variant"),
		}
	}

	#[test]
	fn test_project_cache_status_serialization() {
		let resp = DaemonResponse::ProjectCacheStatus {
			cached: true,
			symbol_count: 500,
			last_indexed: 1706745600,
		};
		let bytes = serialize_response(&resp).unwrap();
		let parsed: DaemonResponse = deserialize_response(&bytes[..bytes.len() - 1]).unwrap();

		match parsed {
			DaemonResponse::ProjectCacheStatus { cached, symbol_count, last_indexed } => {
				assert!(cached);
				assert_eq!(symbol_count, 500);
				assert_eq!(last_indexed, 1706745600);
			}
			_ => panic!("wrong variant"),
		}
	}
}
