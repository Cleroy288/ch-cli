use std::io::Write;

use crate::indexer::SymbolKind;

pub fn print_basic_stat_lines(
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

pub fn print_semantic_stat_lines(
	out: &mut impl Write,
	stats: &crate::indexer::semantic
		::SemanticStats,
) -> std::io::Result<()> {
	writeln!(
		out, "  Total definitions: {}",
		stats.total_definitions,
	)?;
	writeln!(
		out, "  Unique symbols:    {}",
		stats.unique_symbols,
	)?;
	writeln!(
		out, "  Files analyzed:    {}",
		stats.files_analyzed,
	)?;
	writeln!(
		out, "  Total references:  {}",
		stats.total_references,
	)?;
	Ok(())
}

pub fn print_symbol_kinds(
	out: &mut impl Write,
	graph: &crate::indexer::SemanticGraph,
) -> std::io::Result<()> {
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
		let count =
			graph.find_by_kind(kind).len();
		if count > 0 {
			writeln!(
				out, "  {:12}: {}",
				kind.to_string(), count,
			)?;
		}
	}
	Ok(())
}
