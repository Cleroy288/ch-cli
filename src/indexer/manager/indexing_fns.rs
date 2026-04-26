//! Pure helper functions for the indexing pipeline.

use std::path::{Path, PathBuf};
use std::time::Instant;

use crate::indexer::crawler::{CrawlStats, FileResult};
use crate::indexer::parser::ExtractedReference;
use crate::indexer::search::SearchIndex;
use crate::indexer::semantic::SymbolReference;
use crate::indexer::Symbol;

/// Canonicalize a root path, falling back to the original
pub fn canonicalize_root(root: &Path) -> PathBuf {
	root.canonicalize()
		.unwrap_or_else(|_| root.to_path_buf())
}

/// Collect symbols from all file results
pub fn collect_symbols(
	results: &[FileResult],
) -> Vec<Symbol> {
	results
		.iter()
		.flat_map(|res| res.symbols.clone())
		.collect()
}

/// Convert extracted references to symbol references.
/// Takes ownership to avoid cloning name + location.
pub fn to_sym_refs(
	extracted: Vec<ExtractedReference>,
) -> Vec<SymbolReference> {
	extracted
		.into_iter()
		.map(|ext| SymbolReference {
			name: ext.name,
			location: ext.location,
			context: ext.context,
		})
		.collect()
}

/// Build crawl stats from file results
pub fn build_stats(
	found: usize,
	processed: usize,
	results: &[FileResult],
	start: Instant,
) -> CrawlStats {
	let failed = results
		.iter()
		.filter(|res| res.error.is_some())
		.count();
	let sym_count: usize = results
		.iter()
		.map(|res| res.symbols.len())
		.sum();

	CrawlStats {
		files_found: found,
		files_parsed: processed - failed,
		files_failed: failed,
		symbols_found: sym_count,
		duration_ms: start.elapsed().as_millis() as u64,
	}
}

/// Reload symbols from Tantivy if incremental had none
pub fn reload_if_empty(
	syms: Vec<Symbol>,
	idx: &Option<SearchIndex>,
	incremental: bool,
) -> Vec<Symbol> {
	if !syms.is_empty() || !incremental {
		return syms;
	}
	idx.as_ref()
		.and_then(|search| search.load_all_symbols().ok())
		.unwrap_or(syms)
}
