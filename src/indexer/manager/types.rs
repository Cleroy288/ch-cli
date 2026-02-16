//! Types for index manager results and statistics.

use std::path::{Path, PathBuf};
use std::sync::Arc;

use crate::indexer::crawler::{CrawlStats, FileResult};
use crate::indexer::parser::ExtractedReference;
use crate::indexer::search::SearchIndex;
use crate::indexer::semantic::SemanticGraph;
use crate::indexer::state::ChangeSet;
use crate::indexer::Symbol;

/// Progress callback type for tracking indexing progress
pub type ProgressCallback = Box<dyn Fn(usize, usize, &Path) + Send + Sync>;

/// Callback for file change events during watch mode
pub type WatchCallback = Box<dyn Fn(&[PathBuf]) + Send + Sync>;

/// Result of indexing an entire project
pub struct IndexResult {
	/// Root directory that was indexed
	pub root: PathBuf,
	/// All symbols found across all files
	pub symbols: Vec<Symbol>,
	/// All references found across all files
	pub references: Vec<ExtractedReference>,
	/// Per-file results
	pub file_results: Vec<FileResult>,
	/// Statistics about the indexing operation
	pub stats: CrawlStats,
	/// Semantic graph for name resolution (wrapped in Arc for zero-cost sharing)
	pub semantic_graph: Option<Arc<SemanticGraph>>,
	/// Whether this was an incremental index
	pub incremental: bool,
	/// Change set if incremental indexing was used
	pub changes: Option<ChangeSet>,
	/// Search index (if persistence was enabled)
	pub search_index: Option<SearchIndex>,
}

impl std::fmt::Debug for IndexResult {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		f.debug_struct("IndexResult")
			.field("root", &self.root)
			.field("symbols", &self.symbols.len())
			.field("references", &self.references.len())
			.field("file_results", &self.file_results.len())
			.field("stats", &self.stats)
			.field("semantic_graph", &self.semantic_graph.is_some())
			.field("incremental", &self.incremental)
			.field(
				"changes",
				&self.changes.as_ref().map(
					|change_set| change_set.total_changes(),
				),
			)
			.field("search_index", &self.search_index.is_some())
			.finish()
	}
}

/// Statistics about a persisted index
#[derive(Debug, Clone)]
pub struct IndexStats {
	/// Project root
	pub root: PathBuf,
	/// Number of indexed files
	pub file_count: usize,
	/// Number of indexed symbols
	pub symbol_count: usize,
	/// Last update timestamp (seconds since UNIX epoch)
	pub last_updated: u64,
	/// Index format version
	pub version: u32,
}
