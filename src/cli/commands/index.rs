use std::io::Write;
use std::path::Path;
use std::time::Instant;

use crate::service::index::types::IndexOptions;
use crate::service::{DefaultIndexService, IndexService};

use super::error::CommandResult;
use super::index_format::{
	print_semantic_stats, print_stat_lines,
};

pub fn index_command(
	path: &str,
	// TODO: wire verbose flag to IndexOptions
	_verbose: bool,
) -> CommandResult {
	let start = Instant::now();
	print_index_header(path)?;
	let result = run_indexing(path)?;

	print_index_stats(&result.stats)?;
	let mut out = std::io::stdout().lock();
	print_semantic_stats(&mut out, &result)?;
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
) -> std::io::Result<()> {
	let mut out = std::io::stdout().lock();
	writeln!(
		out, "Indexing project at: {}", path
	)?;
	writeln!(
		out, "  Semantic analysis: enabled"
	)?;
	Ok(())
}

fn run_indexing(
	path: &str,
) -> Result<
	crate::indexer::IndexResult,
	super::error::CommandError,
> {
	let service = DefaultIndexService::default();
	let opts = IndexOptions::persistent();
	service
		.index_project(Path::new(path), &opts)
		.map_err(Into::into)
}

/// Display basic index statistics
fn print_index_stats(
	stats: &crate::indexer::CrawlStats,
) -> std::io::Result<()> {
	let mut out = std::io::stdout().lock();
	writeln!(out, "\nIndex Statistics:")?;
	print_stat_lines(&mut out, stats)
}
