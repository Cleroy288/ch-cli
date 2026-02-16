//! Index command implementation.
//!
//! Thin handler that delegates to IndexService.

use std::io::Write;
use std::path::Path;
use std::time::Instant;

use crate::service::index::types::IndexOptions;
use crate::service::{
	DefaultIndexService, IndexService,
};

use super::error::CommandResult;

/// Execute the `index` command
pub fn index_command(
	path: &str,
	semantic: bool,
	_verbose: bool,
) -> CommandResult {
	let start = Instant::now(); // timing
	print_index_header(path, semantic)?;
	let result = run_indexing(path, semantic)?;

	print_index_stats(&result.stats)?;
	print_semantic_stats(&result)?;
	let mut out = std::io::stdout().lock();
	writeln!(
		out,
		"\nDone in {} ms",
		start.elapsed().as_millis(),
	)?;
	Ok(())
}

/// Print initial indexing header
fn print_index_header(
	path: &str,
	semantic: bool,
) -> std::io::Result<()> {
	let mut out = std::io::stdout().lock();
	writeln!(
		out, "Indexing project at: {}", path
	)?;
	if semantic {
		writeln!(
			out, "  Semantic analysis: enabled"
		)?;
	}
	Ok(())
}

/// Run the indexing operation
fn run_indexing(
	path: &str,
	semantic: bool,
) -> Result<
	crate::indexer::IndexResult,
	super::error::CommandError,
> {
	let service = DefaultIndexService::new();
	let opts = IndexOptions {
		flags: crate::service::index::types
			::IndexFlags {
			semantic,
			verbose: false,
			persistence: true,
		},
	};
	service.index_project(
		Path::new(path), &opts,
	).map_err(Into::into)
}

/// Display basic index statistics
fn print_index_stats(
	stats: &crate::indexer::CrawlStats,
) -> std::io::Result<()> {
	let mut out = std::io::stdout().lock();
	writeln!(out, "\nIndex Statistics:")?;
	print_stat_lines(&mut out, stats)?;
	Ok(())
}

/// Print individual stat lines
fn print_stat_lines(
	out: &mut impl Write,
	stats: &crate::indexer::CrawlStats,
) -> std::io::Result<()> {
	writeln!(
		out, "  Files discovered: {}",
		stats.files_found,
	)?;
	writeln!(
		out, "  Files parsed:     {}",
		stats.files_parsed,
	)?;
	writeln!(
		out, "  Files failed:     {}",
		stats.files_failed,
	)?;
	writeln!(
		out, "  Symbols found:    {}",
		stats.symbols_found,
	)?;
	writeln!(
		out, "  Duration:         {} ms",
		stats.duration_ms,
	)?;
	Ok(())
}

/// Display semantic analysis stats if available
fn print_semantic_stats(
	result: &crate::indexer::IndexResult,
) -> std::io::Result<()> {
	let graph = match result.semantic_graph {
		Some(ref graph) => graph,
		None => return Ok(()),
	};
	let stats = graph.stats();
	let mut out = std::io::stdout().lock();

	writeln!(out, "\nSemantic Analysis:")?;
	writeln!(
		out,
		"  Definitions:    {}",
		stats.total_definitions,
	)?;
	writeln!(
		out,
		"  Unique symbols: {}",
		stats.unique_symbols,
	)?;
	Ok(())
}
