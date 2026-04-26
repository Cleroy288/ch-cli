use std::collections::HashSet;
use std::io::Write;

use crate::indexer::semantic::{
	Definition, ReferenceContext,
	SemanticGraph, SymbolReference,
};

/// Max lines after a definition to scan
const CALLEE_SEARCH_RANGE: usize = 100;

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
fn find_callees_near_def<'g>(
	def: &Definition,
	graph: &'g SemanticGraph,
) -> Vec<&'g SymbolReference> {
	let def_file = &def.symbol.location.file;
	let def_line = def.symbol.location.line;
	let end_line = def_line + CALLEE_SEARCH_RANGE;
	graph
		.references_in_file(def_file)
		.into_iter()
		.filter(|r| {
			r.context == ReferenceContext::Call
				&& r.location.line > def_line
				&& r.location.line < end_line
		})
		.collect()
}

/// Print callee list, deduplicated by name
fn print_callee_list(
	out: &mut impl Write,
	callees: &[&SymbolReference],
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
	let mut seen = HashSet::new();
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
