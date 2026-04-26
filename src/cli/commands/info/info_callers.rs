use std::io::Write;

use crate::indexer::semantic::{
	ReferenceContext, SemanticGraph, SymbolReference,
};

/// Max caller entries to print
const MAX_CALLERS_SHOWN: usize = 10;

/// Display all functions that call this symbol
pub(super) fn display_callers(
	symbol_name: &str,
	graph: &SemanticGraph,
) {
	let references =
		graph.find_references(symbol_name);
	let callers: Vec<&_> = references
		.into_iter()
		.filter(|r| {
			r.context == ReferenceContext::Call
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

/// Print caller locations (max MAX_CALLERS_SHOWN)
fn print_caller_list(
	out: &mut impl Write,
	callers: &[&SymbolReference],
) {
	for (idx, ref_item) in callers
		.iter()
		.take(MAX_CALLERS_SHOWN)
		.enumerate()
	{
		let file = ref_file_name(ref_item);
		writeln!(
			out, "   {}. {}:{}",
			idx + 1, file,
			ref_item.location.line,
		)
		.ok();
	}
	if callers.len() > MAX_CALLERS_SHOWN {
		writeln!(
			out, "   ... and {} more",
			callers.len() - MAX_CALLERS_SHOWN,
		)
		.ok();
	}
}

/// Extract short file name from a reference
fn ref_file_name(
	ref_item: &SymbolReference,
) -> &str {
	ref_item
		.location
		.file
		.file_name()
		.and_then(|fname| fname.to_str())
		.unwrap_or("?")
}
