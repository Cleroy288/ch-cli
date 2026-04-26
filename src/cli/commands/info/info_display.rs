use std::io::Write;
use std::sync::Arc;

use crate::indexer::Symbol;
use crate::indexer::semantic::SemanticGraph;
use crate::service::search::types_navigation::{
	DefinitionHit,
};

use crate::cli::commands::error::{
	CommandResult,
};
use super::{
	info_callers, info_callees,
	info_references, info_source,
	InfoDisplayOpts, InfoSections,
};

/// Display info for all definition hits
pub(super) fn display_all_defs(
	defs: &[DefinitionHit],
	graph: &Option<Arc<SemanticGraph>>,
	opts: &InfoDisplayOpts,
) -> CommandResult {
	let mut out = std::io::stdout().lock();
	for (idx, def) in defs.iter().enumerate() {
		if idx > 0 {
			writeln!(
				out,
				"\n{}",
				"\u{2500}".repeat(60)
			)?;
		}
		display_symbol_header(
			&mut out, &def.symbol,
		)?;
		display_sections(
			&def.symbol, graph, opts,
		)?;
	}
	Ok(())
}

/// Display symbol header (kind, name, location)
fn display_symbol_header(
	out: &mut impl Write,
	symbol: &Symbol,
) -> std::io::Result<()> {
	let file = symbol
		.location
		.file
		.file_name()
		.and_then(|f| f.to_str())
		.unwrap_or("?");
	writeln!(
		out, "\n{} {}", symbol.kind, symbol.name
	)?;
	writeln!(
		out, "   File: {}:{}", file,
		symbol.location.line,
	)?;
	writeln!(
		out, "   Path: {}",
		symbol.location.file.display(),
	)?;
	print_symbol_details(out, symbol)
}

/// Print signature and doc comment if present
fn print_symbol_details(
	out: &mut impl Write,
	symbol: &Symbol,
) -> std::io::Result<()> {
	if let Some(ref sig) = symbol.signature {
		writeln!(
			out, "\nSignature:\n   {}", sig
		)?;
	}
	if let Some(ref doc) = symbol.doc_comment {
		writeln!(out, "\nDocumentation:")?;
		for line in doc.lines() {
			writeln!(out, "   {}", line)?;
		}
	}
	Ok(())
}

/// Display optional detail sections
fn display_sections(
	symbol: &Symbol,
	graph: &Option<Arc<SemanticGraph>>,
	opts: &InfoDisplayOpts,
) -> CommandResult {
	if opts.sections.code {
		info_source::display_source_code(symbol)?;
	}
	if let Some(ref sem_graph) = graph {
		display_graph_sections(
			&symbol.name, sem_graph,
			&opts.sections,
		);
	}
	Ok(())
}

/// Display graph-based sections
fn display_graph_sections(
	name: &str,
	graph: &SemanticGraph,
	sec: &InfoSections,
) {
	if sec.callers {
		info_callers::display_callers(
			name, graph,
		);
	}
	if sec.callees {
		info_callees::display_callees(
			name, graph,
		);
	}
	if sec.refs {
		info_references::display_references(
			name, graph,
		);
	}
}
