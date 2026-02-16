//! Reference display helpers for info command.

use std::io::Write;

use crate::indexer::semantic::{
	ReferenceContext, SemanticGraph,
};

/// Display all functions that call this symbol
pub(super) fn display_callers(
	symbol_name: &str,
	graph: &SemanticGraph,
) {
	let references =
		graph.find_references(symbol_name);
	let callers: Vec<&_> = references
		.into_iter()
		.filter(|ref_item| {
			ref_item.context
				== ReferenceContext::Call
		})
		.collect();

	let mut out = std::io::stdout().lock();
	writeln!(
		out,
		"\n\u{1F4DE} Callers ({} call sites):",
		callers.len()
	)
	.ok();

	if callers.is_empty() {
		writeln!(out, "   No callers found").ok();
		return;
	}
	print_caller_list(&mut out, &callers);
}

/// Print caller locations (max 10)
fn print_caller_list(
	out: &mut impl Write,
	callers: &[&crate::indexer::semantic
		::SymbolReference],
) {
	for (idx, ref_item) in
		callers.iter().take(10).enumerate()
	{
		let file = ref_file_name(ref_item);
		writeln!(
			out, "   {}. {}:{}",
			idx + 1, file,
			ref_item.location.line,
		)
		.ok();
	}
	if callers.len() > 10 {
		writeln!(
			out, "   ... and {} more",
			callers.len() - 10,
		)
		.ok();
	}
}

/// Extract short file name from a reference
fn ref_file_name(
	ref_item: &crate::indexer::semantic
		::SymbolReference,
) -> &str {
	ref_item
		.location
		.file
		.file_name()
		.and_then(|fname| fname.to_str())
		.unwrap_or("?")
}

/// Display what functions this symbol calls
pub(super) fn display_callees(
	symbol_name: &str,
	graph: &SemanticGraph,
) {
	let definitions =
		graph.find_definitions(symbol_name);
	let mut out = std::io::stdout().lock();

	if definitions.is_empty() {
		writeln!(
			out, "\n\u{1F4E4} Callees:"
		)
		.ok();
		writeln!(
			out, "   (Definition not found)"
		)
		.ok();
		return;
	}

	let callees =
		find_callees_near_def(definitions[0], graph);
	print_callee_list(&mut out, &callees);
}

/// Find call references near a definition
fn find_callees_near_def<'graph>(
	def: &crate::indexer::semantic::Definition,
	graph: &'graph SemanticGraph,
) -> Vec<&'graph crate::indexer::semantic
	::SymbolReference>
{
	let def_file = &def.symbol.location.file;
	let def_line = def.symbol.location.line;
	graph
		.references_in_file(def_file)
		.into_iter()
		.filter(|ref_item| {
			ref_item.context
				== ReferenceContext::Call
				&& ref_item.location.line > def_line
				&& ref_item.location.line
					< def_line + 100
		})
		.collect()
}

/// Print callee list, deduplicated by name
fn print_callee_list(
	out: &mut impl Write,
	callees: &[&crate::indexer::semantic
		::SymbolReference],
) {
	writeln!(
		out,
		"\n\u{1F4E4} Callees (functions called):"
	)
	.ok();

	if callees.is_empty() {
		writeln!(
			out, "   No function calls found"
		)
		.ok();
		return;
	}
	let mut seen =
		std::collections::HashSet::new();
	for reference in callees.iter() {
		if seen.insert(&reference.name) {
			writeln!(
				out,
				"   - {} (line {})",
				reference.name,
				reference.location.line
			)
			.ok();
		}
	}
}

/// Display all references to this symbol
pub(super) fn display_references(
	symbol_name: &str,
	graph: &SemanticGraph,
) {
	let references =
		graph.find_references(symbol_name);
	let mut out = std::io::stdout().lock();

	writeln!(
		out,
		"\n\u{1F517} References ({} total):",
		references.len()
	)
	.ok();

	if references.is_empty() {
		writeln!(out, "   No references found")
			.ok();
		return;
	}

	print_ref_counts(&mut out, &references);
	print_first_refs(&mut out, &references);
}

/// Print reference counts grouped by context
fn print_ref_counts(
	out: &mut impl Write,
	references: &[&crate::indexer::semantic
		::SymbolReference],
) {
	let counts = count_by_context(references);
	print_nonzero(out, "Calls", counts.0);
	print_nonzero(out, "Type usages", counts.1);
	print_nonzero(out, "Imports", counts.2);
	print_nonzero(out, "Other", counts.3);
}

/// Count references by context type
/// Returns (calls, types, imports, others)
fn count_by_context(
	references: &[&crate::indexer::semantic
		::SymbolReference],
) -> (usize, usize, usize, usize) {
	let calls = references
		.iter()
		.filter(|ref_item| {
			ref_item.context
				== ReferenceContext::Call
		})
		.count();
	let types = references
		.iter()
		.filter(|ref_item| {
			ref_item.context
				== ReferenceContext::Type
		})
		.count();
	let imports = references
		.iter()
		.filter(|ref_item| {
			ref_item.context
				== ReferenceContext::Import
		})
		.count();
	let others =
		references.len() - calls - types - imports;
	(calls, types, imports, others)
}

/// Print a labeled count if non-zero
fn print_nonzero(
	out: &mut impl Write,
	label: &str,
	count: usize,
) {
	if count > 0 {
		writeln!(
			out, "   {}: {}", label, count
		)
		.ok();
	}
}

/// Print first 5 references with locations
fn print_first_refs(
	out: &mut impl Write,
	references: &[&crate::indexer::semantic
		::SymbolReference],
) {
	writeln!(out, "\n   First 5 references:")
		.ok();
	for (idx, ref_item) in
		references.iter().take(5).enumerate()
	{
		let file = ref_item
			.location
			.file
			.file_name()
			.and_then(|fname| fname.to_str())
			.unwrap_or("?");
		writeln!(
			out,
			"   {}. {}:{} ({:?})",
			idx + 1,
			file,
			ref_item.location.line,
			ref_item.context
		)
		.ok();
	}
}
