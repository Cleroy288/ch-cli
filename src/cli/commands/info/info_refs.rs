//! Reference display helpers for info command.

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
	let callers: Vec<_> = references
		.iter()
		.filter(|r| {
			r.context == ReferenceContext::Call
		})
		.collect();

	println!(
		"\n\u{1F4DE} Callers ({} call sites):",
		callers.len()
	);

	if callers.is_empty() {
		println!("   No callers found");
		return;
	}

	for (i, r) in callers.iter().take(10).enumerate()
	{
		let file = r
			.location
			.file
			.file_name()
			.and_then(|f| f.to_str())
			.unwrap_or("?");
		println!(
			"   {}. {}:{}",
			i + 1,
			file,
			r.location.line
		);
	}

	if callers.len() > 10 {
		println!(
			"   ... and {} more",
			callers.len() - 10
		);
	}
}

/// Display what functions this symbol calls
pub(super) fn display_callees(
	symbol_name: &str,
	graph: &SemanticGraph,
) {
	let definitions =
		graph.find_definitions(symbol_name);

	if definitions.is_empty() {
		println!("\n\u{1F4E4} Callees:");
		println!("   (Definition not found)");
		return;
	}

	let def = &definitions[0];
	let def_file = &def.symbol.location.file;
	let def_line = def.symbol.location.line;

	// Calls within ~100 lines of definition
	let refs_in_file =
		graph.references_in_file(def_file);
	let callees: Vec<_> = refs_in_file
		.into_iter()
		.filter(|r| {
			r.context == ReferenceContext::Call
				&& r.location.line > def_line
				&& r.location.line < def_line + 100
		})
		.collect();

	println!(
		"\n\u{1F4E4} Callees (functions called):"
	);

	if callees.is_empty() {
		println!("   No function calls found");
		return;
	}

	// Deduplicate by name
	let mut seen =
		std::collections::HashSet::new();
	for reference in callees.iter() {
		if seen.insert(&reference.name) {
			println!(
				"   - {} (line {})",
				reference.name,
				reference.location.line
			);
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

	println!(
		"\n\u{1F517} References ({} total):",
		references.len()
	);

	if references.is_empty() {
		println!("   No references found");
		return;
	}

	print_ref_counts(&references);
	print_first_refs(&references);
}

/// Print reference counts grouped by context
fn print_ref_counts(
	references: &[&crate::indexer::semantic::SymbolReference],
) {
	let calls = references
		.iter()
		.filter(|r| {
			r.context == ReferenceContext::Call
		})
		.count();
	let types = references
		.iter()
		.filter(|r| {
			r.context == ReferenceContext::Type
		})
		.count();
	let imports = references
		.iter()
		.filter(|r| {
			r.context == ReferenceContext::Import
		})
		.count();
	let others =
		references.len() - calls - types - imports;

	if calls > 0 {
		println!("   Calls: {}", calls);
	}
	if types > 0 {
		println!("   Type usages: {}", types);
	}
	if imports > 0 {
		println!("   Imports: {}", imports);
	}
	if others > 0 {
		println!("   Other: {}", others);
	}
}

/// Print first 5 references with locations
fn print_first_refs(
	references: &[&crate::indexer::semantic::SymbolReference],
) {
	println!("\n   First 5 references:");
	for (i, r) in
		references.iter().take(5).enumerate()
	{
		let file = r
			.location
			.file
			.file_name()
			.and_then(|f| f.to_str())
			.unwrap_or("?");
		println!(
			"   {}. {}:{} ({:?})",
			i + 1,
			file,
			r.location.line,
			r.context
		);
	}
}
