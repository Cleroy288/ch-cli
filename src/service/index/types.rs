//! DTOs for the index service.

#[allow(clippy::struct_excessive_bools)]
/// Boolean flags for project indexing
#[derive(Debug, Clone, Default)]
pub struct IndexFlags {
	/// Enable semantic analysis
	pub semantic: bool,
	/// Show verbose output
	pub verbose: bool,
	/// Enable persistence of index state
	pub persistence: bool,
}

/// Options for project indexing
#[derive(Debug, Clone, Default)]
pub struct IndexOptions {
	/// boolean flags
	pub flags: IndexFlags,
}
