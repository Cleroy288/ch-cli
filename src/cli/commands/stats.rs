use std::io::Write;
use std::path::Path;
use std::time::Instant;

use crate::service::index::types::IndexOptions;
use crate::service::{DefaultIndexService, IndexService};

use super::error::CommandResult;
use super::stats_format::{
	print_basic_stat_lines,
	print_semantic_stat_lines,
	print_symbol_kinds,
};

pub fn stats_command() -> CommandResult {
	let start = Instant::now();
	let service = DefaultIndexService::default();
	let opts = IndexOptions::persistent();
	let result = service.index_project(
		Path::new("."),
		&opts,
	)?;

	print_basic_stats(&result.stats)?;
	print_semantic_breakdown(&result)?;
	let mut out = std::io::stdout().lock();
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
	print_basic_stat_lines(&mut out, stats)
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
	print_semantic_stat_lines(&mut out, &stats)?;
	print_symbol_kinds(&mut out, graph)
}
