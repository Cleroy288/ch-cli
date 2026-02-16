//! Symbol info command - show detailed info
//! about a symbol (definition, refs, callers).

use std::io::Write;
use std::path::Path;
use std::sync::Arc;

use crate::indexer::Symbol;
use crate::indexer::semantic::SemanticGraph;
use crate::service::search::types_navigation::{
	DefinitionHit,
};
use crate::service::{
	DefaultSearchService, SearchService,
};

use super::error::{CommandError, CommandResult};

mod info_refs;
#[doc(hidden)]
pub mod info_source;

#[allow(clippy::struct_excessive_bools)]
/// Boolean sections for the info command
#[derive(Debug, Clone, Default)]
pub struct InfoSections {
	/// show source code
	pub code: bool,
	/// show callers
	pub callers: bool,
	/// show callees
	pub callees: bool,
	/// show references
	pub refs: bool,
}

/// Raw CLI flags for the info command
#[derive(Default)]
pub struct InfoFlags {
	/// which sections to display
	pub sections: InfoSections,
}

/// Display options for the info command
pub struct InfoDisplayOpts {
	/// which sections to display
	pub sections: InfoSections,
}

impl InfoDisplayOpts {
	/// Build from CLI flags and an `all` toggle
	pub fn from_flags(
		flags: InfoFlags,
		all: bool,
	) -> Self {
		let sec = &flags.sections;
		Self {
			sections: InfoSections {
				code: sec.code || all,
				callers: sec.callers || all,
				callees: sec.callees || all,
				refs: sec.refs || all,
			},
		}
	}
}

/// Execute the `info` command
pub fn info_command(
	symbol: &str,
	opts: &InfoDisplayOpts,
) -> CommandResult {
	let service = DefaultSearchService::new();
	let defs = service.find_definition(
		symbol, Path::new("."),
	)?;
	let mut out = std::io::stdout().lock();
	if defs.is_empty() {
		writeln!(
			out, "Symbol '{}' not found.", symbol
		)?;
		return Ok(());
	}

	let sec = &opts.sections;
	let need_graph =
		sec.callers || sec.callees || sec.refs;
	let graph = if need_graph {
		build_graph()?
	} else {
		None
	};

	display_all_defs(&defs, &graph, opts)
}

/// Display info for all definition hits
fn display_all_defs(
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
		.and_then(|fname| fname.to_str())
		.unwrap_or("?");

	writeln!(
		out, "\n{} {}", symbol.kind, symbol.name
	)?;
	writeln!(
		out,
		"   File: {}:{}",
		file, symbol.location.line
	)?;
	writeln!(
		out,
		"   Path: {}",
		symbol.location.file.display()
	)?;
	print_symbol_details(out, symbol)?;
	Ok(())
}

/// Print signature and doc comment if present
fn print_symbol_details(
	out: &mut impl Write,
	symbol: &Symbol,
) -> std::io::Result<()> {
	if let Some(ref sig) = symbol.signature {
		writeln!(out, "\nSignature:\n   {}", sig)?;
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
		info_refs::display_callers(name, graph);
	}
	if sec.callees {
		info_refs::display_callees(name, graph);
	}
	if sec.refs {
		info_refs::display_references(name, graph);
	}
}

/// Result type for optional graph building
type GraphResult =
	Result<Option<Arc<SemanticGraph>>, CommandError>;

/// Build semantic graph for detail sections
fn build_graph() -> GraphResult {
	use crate::indexer::IndexManager;
	let manager = IndexManager::new()
		.with_semantic_analysis();
	let result =
		manager.index_project(Path::new("."))?;
	Ok(result.semantic_graph)
}
