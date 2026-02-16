//! Types for the daemon server
//!
//! Contains the CachedProject struct, shared type aliases
//! for doc generation, and caching project data.

use std::collections::HashMap;
use std::path::PathBuf;
use std::sync::{Arc, Mutex};

use crate::indexer::{SemanticGraph, Symbol};
use crate::retrieval::docgen::{DocEntry, DocGenerator, DocStore};
use crate::retrieval::hybrid::HybridSearch;

/// Shared doc stores across daemon threads
pub type SharedDocStores =
	Arc<Mutex<HashMap<PathBuf, DocStore>>>;

/// Shared doc generator across daemon threads
pub type SharedDocGenerator =
	Arc<Mutex<Option<DocGenerator>>>;

/// Graph + symbols pair for cross-references
pub type GraphAndSymbols =
	Option<(Arc<SemanticGraph>, Vec<Symbol>)>;

/// Prepared entry: (id, entry, prompt)
pub type PreparedEntry = (String, DocEntry, String);

/// Maximum number of projects to cache in daemon memory
pub const MAX_CACHED_PROJECTS: usize = 5;

/// A cached project with index data for fast retrieval
pub struct CachedProject {
	/// all indexed symbols
	pub symbols: Vec<Symbol>,
	/// semantic graph for context expansion (Arc-wrapped)
	pub graph: Option<Arc<SemanticGraph>>,
	/// hybrid search instance
	pub hybrid: HybridSearch,
	/// when the project was indexed (unix timestamp)
	pub last_indexed: u64,
}
