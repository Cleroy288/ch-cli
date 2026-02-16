//! Stats command implementation.
//!
//! Thin handler that delegates to IndexService.

use std::io::Write;
use std::path::Path;
use std::time::Instant;

use crate::indexer::SymbolKind;
use crate::service::index::types::IndexOptions;
use crate::service::{
	DefaultIndexService, IndexService,
};

use super::error::CommandResult;

/// Execute the `stats` command
pub fn stats_command() -> CommandResult {
	let start = Instant::now(); // timing
	let mut out = std::io::stdout().lock();

	let service = DefaultIndexService::new();
	let opts = IndexOptions {
		flags: crate::service::index::types
			::IndexFlags {
			semantic: true,
			verbose: false,
			persistence: true,
		},
	};
	let result = service.index_project(
		Path::new("."),
		&opts,
	)?;

	print_basic_stats(&result.stats)?;
	print_semantic_breakdown(&result)?;
	writeln!(
		out,
		"\nStats generated in {} ms",
		start.elapsed().as_millis(),
	)?;

	Ok(())
}

/// Display basic crawl statistics
fn print_basic_stats(
	stats: &crate::indexer::CrawlStats,
) -> std::io::Result<()> {
	let mut out = std::io::stdout().lock();
	writeln!(out, "Index Statistics:\n")?;
	print_basic_stat_lines(&mut out, stats)?;
	Ok(())
}

/// Print individual basic stat lines
fn print_basic_stat_lines(
	out: &mut impl Write,
	stats: &crate::indexer::CrawlStats,
) -> std::io::Result<()> {
	writeln!(
		out, "  Files discovered:  {}",
		stats.files_found,
	)?;
	writeln!(
		out, "  Files parsed:      {}",
		stats.files_parsed,
	)?;
	writeln!(
		out, "  Files failed:      {}",
		stats.files_failed,
	)?;
	writeln!(
		out, "  Symbols found:     {}",
		stats.symbols_found,
	)?;
	writeln!(
		out, "  Indexing time:     {} ms",
		stats.duration_ms,
	)?;
	Ok(())
}

/// Display semantic graph breakdown
fn print_semantic_breakdown(
	result: &crate::indexer::IndexResult,
) -> std::io::Result<()> {
	let graph = match result.semantic_graph {
		Some(ref graph) => graph,
		None => return Ok(()),
	};
	let stats = graph.stats();
	let mut out = std::io::stdout().lock();
	writeln!(out, "\nSemantic Analysis:\n")?;
	print_semantic_stats(&mut out, &stats)?;
	print_symbol_kinds(graph)?;
	Ok(())
}

/// Print semantic analysis stat lines
fn print_semantic_stats(
	out: &mut impl Write,
	stats: &crate::indexer::semantic::SemanticStats,
) -> std::io::Result<()> {
	writeln!(
		out,
		"  Total definitions: {}",
		stats.total_definitions,
	)?;
	writeln!(
		out,
		"  Unique symbols:    {}",
		stats.unique_symbols,
	)?;
	writeln!(
		out,
		"  Files analyzed:    {}",
		stats.files_analyzed,
	)?;
	writeln!(
		out,
		"  Total references:  {}",
		stats.total_references,
	)?;
	Ok(())
}

/// Print per-kind symbol counts from the graph
fn print_symbol_kinds(
	graph: &crate::indexer::SemanticGraph,
) -> std::io::Result<()> {
	let mut out = std::io::stdout().lock();
	writeln!(out, "\nSymbol breakdown:")?;
	let kinds = [
		SymbolKind::Function,
		SymbolKind::Struct,
		SymbolKind::Enum,
		SymbolKind::Trait,
		SymbolKind::Impl,
		SymbolKind::Method,
		SymbolKind::Constant,
		SymbolKind::Static,
		SymbolKind::Module,
	];
	for kind in kinds {
		let count = graph.find_by_kind(kind).len();
		if count > 0 {
			writeln!(
				out,
				"  {:12}: {}",
				format!("{}", kind),
				count,
			)?;
		}
	}
	Ok(())
}
