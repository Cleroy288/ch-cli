use std::io::Write;

pub fn print_stat_lines(
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

pub fn print_semantic_stats(
	out: &mut impl Write,
	result: &crate::indexer::IndexResult,
) -> std::io::Result<()> {
	let graph = match result.semantic_graph {
		Some(ref g) => g,
		None => return Ok(()),
	};
	let stats = graph.stats();
	writeln!(out, "\nSemantic Analysis:")?;
	writeln!(
		out, "  Definitions:    {}",
		stats.total_definitions,
	)?;
	writeln!(
		out, "  Unique symbols: {}",
		stats.unique_symbols,
	)?;
	Ok(())
}
