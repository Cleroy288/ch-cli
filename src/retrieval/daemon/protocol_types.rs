//! Protocol data types for daemon IPC
//!
//! Contains structs used in DaemonRequest/DaemonResponse
//! messages: status, search results, doc entries.

use serde::{Deserialize, Serialize};

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
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
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

