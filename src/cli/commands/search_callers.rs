//! Display callers/callees from SemanticGraph.
//!
//! Pure presentation: formats caller/callee
//! results for CLI output.

use crate::indexer::semantic::{
	ReferenceContext, SemanticGraph,
};
use crate::retrieval::query::{
	CallerDirection, CallerQuery,
};

use super::error::CommandResult;

/// Handle a caller query using the semantic graph
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

/// Display all functions that call the symbol
fn display_callers(
	name: &str,
	graph: &SemanticGraph,
) -> CommandResult {
	let refs = graph.find_references(name);
	let calls: Vec<_> = refs
		.iter()
		.filter(|r| {
			r.context == ReferenceContext::Call
		})
		.collect();

	if calls.is_empty() {
		println!(
			"No callers found for '{}'", name
		);
		println!(
			"\nTip: Make sure the symbol \
			name is exact (case-sensitive)"
		);
		return Ok(());
	}

	println!("Callers of '{}':\n", name);
	for (i, r) in calls.iter().enumerate() {
		let file = r
			.location
			.file
			.file_name()
			.and_then(|f| f.to_str())
			.unwrap_or("?");
		println!(
			"  {}. {}:{} ({})",
			i + 1,
			file,
			r.location.line,
			r.location.file.display()
		);
	}
	println!(
		"\nTotal: {} call site(s)", calls.len()
	);
	Ok(())
}

/// Display all functions called by the symbol
fn display_callees(
	name: &str,
	graph: &SemanticGraph,
) -> CommandResult {
	let defs = graph.find_definitions(name);
	if defs.is_empty() {
		println!(
			"Symbol '{}' not found in definitions",
			name
		);
		return Ok(());
	}

	println!(
		"Callees of '{}' (functions it calls):\n",
		name
	);
	println!(
		"Note: Callee tracking requires body \
		analysis (not yet fully implemented)"
	);

	println!("\nDefinition locations:");
	for (i, def) in defs.iter().enumerate() {
		let file = def
			.symbol
			.location
			.file
			.file_name()
			.and_then(|f| f.to_str())
			.unwrap_or("?");
		println!(
			"  {}. {} ({}:{})",
			i + 1,
			def.fqn,
			file,
			def.symbol.location.line
		);
	}
	Ok(())
}

