use std::io::Write;

use crate::indexer::semantic::{
	ReferenceContext, SemanticGraph,
};
use crate::service::search::caller::{
	CallerDirection, CallerQuery,
};

use super::error::CommandResult;

pub fn handle_caller_query(
	query: &CallerQuery,
	graph: &SemanticGraph,
) -> CommandResult {
	let name = &query.symbol_name;
	match query.direction {
		CallerDirection::Callers => {
			display_callers(name, graph)
		}
		CallerDirection::Callees => {
			display_callees(name, graph)
		}
	}
}

fn display_callers(
	name: &str,
	graph: &SemanticGraph,
) -> CommandResult {
	let refs = graph.find_references(name);
	let calls: Vec<_> = refs
		.iter()
		.filter(|ref_item| {
			ref_item.context
				== ReferenceContext::Call
		})
		.collect();

	let mut out = std::io::stdout().lock();
	if calls.is_empty() {
		writeln!(
			out,
			"No callers found for '{}'",
			name,
		)?;
		writeln!(
			out,
			"\nTip: Make sure the symbol \
			name is exact (case-sensitive)"
		)?;
		return Ok(());
	}
	print_caller_entries(&mut out, name, &calls)
}

fn print_caller_entries(
	out: &mut impl Write,
	name: &str,
	calls: &[&&crate::indexer::semantic
		::SymbolReference],
) -> CommandResult {
	writeln!(out, "Callers of '{}':\n", name)?;
	for (idx, ref_item) in
		calls.iter().enumerate()
	{
		let file = ref_item
			.location
			.file
			.file_name()
			.and_then(|fname| fname.to_str())
			.unwrap_or("?");
		writeln!(
			out,
			"  {}. {}:{} ({})",
			idx + 1,
			file,
			ref_item.location.line,
			ref_item.location.file.display()
		)?;
	}
	writeln!(
		out,
		"\nTotal: {} call site(s)",
		calls.len(),
	)?;
	Ok(())
}

fn display_callees(
	name: &str,
	graph: &SemanticGraph,
) -> CommandResult {
	let defs = graph.find_definitions(name);
	let mut out = std::io::stdout().lock();
	if defs.is_empty() {
		writeln!(
			out,
			"Symbol '{}' not found \
			in definitions",
			name,
		)?;
		return Ok(());
	}
	writeln!(out, "Callees of '{name}':")?;
	writeln!(
		out,
		"\nNote: Callee tracking requires \
		body analysis (not yet implemented)"
	)?;
	writeln!(out, "\nDefinition locations:")?;
	print_def_entries(&mut out, &defs)
}

fn print_def_entries(
	out: &mut impl Write,
	defs: &[&crate::indexer::semantic
		::Definition],
) -> CommandResult {
	for (idx, def) in defs.iter().enumerate() {
		let file = def
			.symbol
			.location
			.file
			.file_name()
			.and_then(|f| f.to_str())
			.unwrap_or("?");
		writeln!(
			out,
			"  {}. {} ({}:{})",
			idx + 1,
			def.fqn,
			file,
			def.symbol.location.line,
		)?;
	}
	Ok(())
}
