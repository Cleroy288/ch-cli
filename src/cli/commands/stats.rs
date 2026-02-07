//! Stats command implementation.
//!
//! Thin handler that delegates to IndexService.

use std::path::Path;
use std::time::Instant;

use crate::indexer::SymbolKind;
use crate::service::index::types::IndexOptions;
use crate::service::{DefaultIndexService, IndexService};

use super::error::CommandResult;

/// Execute the `stats` command - show index statistics
pub fn stats_command() -> CommandResult {
	let start = Instant::now(); // timing

	let service = DefaultIndexService::new();
	let opts = IndexOptions {
		semantic: true,
		verbose: false,
		persistence: true,
	};
	let result = service.index_project(
		Path::new("."),
		&opts,
	)?;

	print_basic_stats(&result.stats);
	print_semantic_breakdown(&result);
	println!(
		"\nStats generated in {} ms",
		start.elapsed().as_millis(),
	);

	Ok(())
}

/// Display basic crawl statistics
fn print_basic_stats(
	stats: &crate::indexer::CrawlStats,
) {
	println!("Index Statistics:\n");
	println!(
		"  Files discovered:  {}",
		stats.files_found,
	);
	println!(
		"  Files parsed:      {}",
		stats.files_parsed,
	);
	println!(
		"  Files failed:      {}",
		stats.files_failed,
	);
	println!(
		"  Symbols found:     {}",
		stats.symbols_found,
	);
	println!(
		"  Indexing time:     {} ms",
		stats.duration_ms,
	);
}

/// Display semantic graph breakdown by symbol kind
fn print_semantic_breakdown(
	result: &crate::indexer::IndexResult,
) {
	let graph = match result.semantic_graph {
		Some(ref g) => g,
		None => return,
	};
	let stats = graph.stats();

	println!("\nSemantic Analysis:\n");
	println!(
		"  Total definitions: {}",
		stats.total_definitions,
	);
	println!(
		"  Unique symbols:    {}",
		stats.unique_symbols,
	);
	println!(
		"  Files analyzed:    {}",
		stats.files_analyzed,
	);
	println!(
		"  Total references:  {}",
		stats.total_references,
	);

	print_symbol_kinds(graph);
}

/// Print per-kind symbol counts from the graph
fn print_symbol_kinds(
	graph: &crate::indexer::SemanticGraph,
) {
	println!("\nSymbol breakdown:");
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
			println!(
				"  {:12}: {}",
				format!("{}", kind),
				count,
			);
		}
	}
}
