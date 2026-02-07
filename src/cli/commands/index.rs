//! Index command implementation.
//!
//! Thin handler that delegates to IndexService.

use std::path::Path;
use std::time::Instant;

use crate::service::index::types::IndexOptions;
use crate::service::{DefaultIndexService, IndexService};

use super::error::CommandResult;

/// Execute the `index` command
pub fn index_command(
	path: &str,
	semantic: bool,
	_verbose: bool,
) -> CommandResult {
	let start = Instant::now(); // timing

	println!("Indexing project at: {}", path);
	if semantic {
		println!("  Semantic analysis: enabled");
	}

	let service = DefaultIndexService::new();
	let opts = IndexOptions {
		semantic,
		verbose: false,
		persistence: true,
	};
	let result = service.index_project(
		Path::new(path),
		&opts,
	)?;

	print_index_stats(&result.stats);
	print_semantic_stats(&result);
	println!(
		"\nDone in {} ms",
		start.elapsed().as_millis(),
	);

	Ok(())
}

/// Display basic index statistics
fn print_index_stats(
	stats: &crate::indexer::CrawlStats,
) {
	println!("\nIndex Statistics:");
	println!(
		"  Files discovered: {}",
		stats.files_found,
	);
	println!(
		"  Files parsed:     {}",
		stats.files_parsed,
	);
	println!(
		"  Files failed:     {}",
		stats.files_failed,
	);
	println!(
		"  Symbols found:    {}",
		stats.symbols_found,
	);
	println!(
		"  Duration:         {} ms",
		stats.duration_ms,
	);
}

/// Display semantic analysis stats if available
fn print_semantic_stats(
	result: &crate::indexer::IndexResult,
) {
	let graph = match result.semantic_graph {
		Some(ref g) => g,
		None => return,
	};
	let stats = graph.stats();

	println!("\nSemantic Analysis:");
	println!(
		"  Definitions:    {}",
		stats.total_definitions,
	);
	println!(
		"  Unique symbols: {}",
		stats.unique_symbols,
	);
}
