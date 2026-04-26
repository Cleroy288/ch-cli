use std::path::{Path, PathBuf};
use std::sync::Arc;

use crate::indexer::crawler::{CrawlStats, FileResult};
use crate::indexer::parser::ExtractedReference;
use crate::indexer::search::SearchIndex;
use crate::indexer::semantic::SemanticGraph;
use crate::indexer::state::ChangeSet;
use crate::indexer::Symbol;

pub type ProgressCallback = Box<dyn Fn(usize, usize, &Path) + Send + Sync>;

pub type WatchCallback = Box<dyn Fn(&[PathBuf]) + Send + Sync>;

pub struct IndexResult {
	pub root: PathBuf,
	pub symbols: Vec<Symbol>,
	pub references: Vec<ExtractedReference>,
	pub file_results: Vec<FileResult>,
	pub stats: CrawlStats,
	pub semantic_graph: Option<Arc<SemanticGraph>>,
	pub incremental: bool,
	pub changes: Option<ChangeSet>,
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

#[derive(Debug, Clone)]
pub struct IndexStats {
	pub root: PathBuf,
	pub file_count: usize,
	pub symbol_count: usize,
	pub last_updated: u64,
	pub version: u32,
}
