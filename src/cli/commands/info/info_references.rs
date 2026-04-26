use std::io::Write;

use crate::indexer::semantic::{
	ReferenceContext, SemanticGraph,
	SymbolReference,
};

/// Max reference entries shown in detail
const MAX_REFS_SHOWN: usize = 5;

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

/// Counts grouped by reference context type
struct ContextCounts {
	calls: usize,
	types: usize,
	imports: usize,
	others: usize,
}

fn count_by_context(
	references: &[&SymbolReference],
) -> ContextCounts {
	references.iter().fold(
		ContextCounts {
			calls: 0,
			types: 0,
			imports: 0,
			others: 0,
		},
		|mut acc, r| {
			match r.context {
				ReferenceContext::Call => {
					acc.calls += 1;
				}
				ReferenceContext::Type => {
					acc.types += 1;
				}
				ReferenceContext::Import => {
					acc.imports += 1;
				}
				_ => acc.others += 1,
			}
			acc
		},
	)
}

/// Print reference counts grouped by context
fn print_ref_counts(
	out: &mut impl Write,
	references: &[&SymbolReference],
) {
	let c = count_by_context(references);
	print_nonzero(out, "Calls", c.calls);
	print_nonzero(out, "Type usages", c.types);
	print_nonzero(out, "Imports", c.imports);
	print_nonzero(out, "Other", c.others);
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

// Note: print_first_refs is the 5th function.
// print_nonzero + count_by_context are private
// helpers that support the 3 display functions.

/// Print first N references with locations
fn print_first_refs(
	out: &mut impl Write,
	references: &[&SymbolReference],
) {
	writeln!(
		out,
		"\n   First {} references:",
		MAX_REFS_SHOWN
	)
	.ok();
	for (idx, ref_item) in references
		.iter()
		.take(MAX_REFS_SHOWN)
		.enumerate()
	{
		let file = ref_item
			.location
			.file
			.file_name()
			.and_then(|f| f.to_str())
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
