//! Types for the daemon server
//!
//! Contains the CachedProject struct for caching project data.

use crate::indexer::{SemanticGraph, Symbol};
use crate::retrieval::hybrid::HybridSearch;

/// Maximum number of projects to cache in daemon memory
pub const MAX_CACHED_PROJECTS: usize = 5;

/// A cached project with index data for fast retrieval
pub struct CachedProject {
	/// all indexed symbols
	pub symbols: Vec<Symbol>,
	/// semantic graph for context expansion
	pub graph: Option<SemanticGraph>,
	/// hybrid search instance
	pub hybrid: HybridSearch,
	/// when the project was indexed (unix timestamp)
	pub last_indexed: u64,
}
