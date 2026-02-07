//! DTOs for the index service.

/// Options for project indexing
#[derive(Debug, Clone, Default)]
pub struct IndexOptions {
	/// Enable semantic analysis
	pub semantic: bool,
	/// Show verbose output
	pub verbose: bool,
	/// Enable persistence of index state
	pub persistence: bool,
}
