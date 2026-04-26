use std::fs;

use crate::domain::errors::search::SearchError;
use crate::indexer::{
	CodeLocation, IndexResult, ReferenceContext,
	SemanticGraph, SymbolKind,
};

use super::navigation_impl::require_graph;
use super::types::{CallerHit, SymbolDetails};

/// Max lines to extract around a symbol definition
const SNIPPET_LINES: usize = 20;

pub(super) fn get_symbol_info(
	symbol: &str,
	result: &IndexResult,
) -> Result<Option<SymbolDetails>, SearchError> {
	let graph = require_graph(result)?;
	let defs = graph.find_definitions(symbol);
	if defs.is_empty() {
		return Ok(None);
	}
	let def = &defs[0];
	let source = load_snippet(&def.symbol.location);
	let callers = collect_callers(graph, symbol);
	let refs: Vec<CodeLocation> = graph
		.find_references(symbol)
		.into_iter()
		.map(|sym_ref| sym_ref.location.clone())
		.collect();

	Ok(Some(SymbolDetails {
		symbol: def.symbol.clone(),
		source_code: source,
		callers,
		callees: vec![],
		references: refs,
	}))
}

pub(super) fn search_callers(
	symbol: &str,
	result: &IndexResult,
) -> Result<Vec<CallerHit>, SearchError> {
	let graph = require_graph(result)?;
	Ok(collect_callers(graph, symbol))
}

/// Find call-site references with caller names
fn collect_callers(
	graph: &SemanticGraph,
	symbol: &str,
) -> Vec<CallerHit> {
	graph
		.find_references(symbol)
		.into_iter()
		.filter(|sym_ref| {
			sym_ref.context == ReferenceContext::Call
		})
		.map(|sym_ref| {
			let name = resolve_caller(
				graph, &sym_ref.location,
			);
			CallerHit {
				file: sym_ref.location.file.clone(),
				line: sym_ref.location.line,
				context: String::new(),
				caller_name: name,
			}
		})
		.collect()
}

/// Find the function/method containing a call site.
/// starts before the given line in the same file.
fn resolve_caller(
	graph: &SemanticGraph,
	loc: &CodeLocation,
) -> Option<String> {
	let defs = graph.definitions_in_file(&loc.file);
	defs.into_iter()
		.filter(|def| {
			let kind = def.symbol.kind;
			kind == SymbolKind::Function
				|| kind == SymbolKind::Method
		})
		.filter(|def| def.symbol.location.line <= loc.line)
		.max_by_key(|def| def.symbol.location.line)
		.map(|def| def.symbol.name.clone())
}

fn load_snippet(
	loc: &CodeLocation,
) -> Option<String> {
	let content =
		fs::read_to_string(&loc.file).ok()?;
	let lines: Vec<&str> =
		content.lines().collect();
	// line is 1-indexed, convert to 0-indexed
	let start = loc.line.saturating_sub(1);
	let end =
		(start + SNIPPET_LINES).min(lines.len());
	Some(lines[start..end].join("\n"))
}
