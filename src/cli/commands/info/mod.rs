//! Symbol info command - show detailed info
//! about a symbol (definition, refs, callers).

use std::path::Path;

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

/// Execute the `info` command
pub fn info_command(
	symbol: &str,
	show_code: bool,
	show_callers: bool,
	show_callees: bool,
	show_refs: bool,
	show_all: bool,
) -> CommandResult {
	let code = show_code || show_all;
	let callers = show_callers || show_all;
	let callees = show_callees || show_all;
	let refs = show_refs || show_all;

	let service = DefaultSearchService::new();
	let defs = service.find_definition(
		symbol, Path::new("."),
	)?;
	if defs.is_empty() {
		println!(
			"Symbol '{}' not found.", symbol
		);
		return Ok(());
	}

	let need_graph = callers || callees || refs;
	let graph = if need_graph {
		build_graph()?
	} else {
		None
	};

	display_all_defs(
		&defs, &graph,
		code, callers, callees, refs,
	)
}

/// Display info for all definition hits
fn display_all_defs(
	defs: &[DefinitionHit],
	graph: &Option<SemanticGraph>,
	code: bool,
	callers: bool,
	callees: bool,
	refs: bool,
) -> CommandResult {
	for (i, def) in defs.iter().enumerate() {
		if i > 0 {
			println!(
				"\n{}", "\u{2500}".repeat(60)
			);
		}
		display_symbol_header(&def.symbol);
		display_sections(
			&def.symbol, graph,
			code, callers, callees, refs,
		)?;
	}
	Ok(())
}

/// Display symbol header (kind, name, location)
fn display_symbol_header(symbol: &Symbol) {
	let file = symbol
		.location
		.file
		.file_name()
		.and_then(|f| f.to_str())
		.unwrap_or("?");

	println!(
		"\n{} {}", symbol.kind, symbol.name
	);
	println!(
		"   File: {}:{}",
		file, symbol.location.line
	);
	println!(
		"   Path: {}",
		symbol.location.file.display()
	);
	if let Some(ref sig) = symbol.signature {
		println!("\nSignature:\n   {}", sig);
	}
	if let Some(ref doc) = symbol.doc_comment {
		println!("\nDocumentation:");
		for line in doc.lines() {
			println!("   {}", line);
		}
	}
}

/// Display optional detail sections
fn display_sections(
	symbol: &Symbol,
	graph: &Option<SemanticGraph>,
	code: bool,
	callers: bool,
	callees: bool,
	refs: bool,
) -> CommandResult {
	if code {
		info_source::display_source_code(symbol)?;
	}
	if let Some(ref g) = graph {
		if callers {
			info_refs::display_callers(
				&symbol.name, g,
			);
		}
		if callees {
			info_refs::display_callees(
				&symbol.name, g,
			);
		}
		if refs {
			info_refs::display_references(
				&symbol.name, g,
			);
		}
	}
	Ok(())
}

/// Build semantic graph for detail sections
fn build_graph(
) -> Result<Option<SemanticGraph>, CommandError> {
	use crate::indexer::IndexManager;
	let manager = IndexManager::new()
		.with_semantic_analysis();
	let result =
		manager.index_project(Path::new("."))?;
	Ok(result.semantic_graph)
}
