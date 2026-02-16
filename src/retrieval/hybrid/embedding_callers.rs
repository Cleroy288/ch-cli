//! Caller context for embedding text enrichment.
//!
//! Extracts caller module names from SemanticGraph
//! to add "called from X, Y" context to embedding
//! text. This helps differentiate functions by their
//! usage relationships.

use std::collections::HashMap;

use crate::indexer::semantic::{
	ReferenceContext, SemanticGraph,
};
use crate::indexer::symbols::Symbol;
use crate::retrieval::hybrid::embedding_text::file_context;

/// Max caller modules to include in embedding text
const MAX_CALLERS: usize = 3;

/// Separator between caller names in output
const CALLER_SEP: &str = ", ";

/// Prefix for the caller context string
const CALLER_PREFIX: &str = "called from ";

/// Build caller context string for a symbol.
///
/// Finds all Call references to `name` in the graph,
/// extracts unique caller module names, and formats
/// them as "called from module1, module2".
/// Returns None if no callers found.
pub fn caller_context(
	name: &str,
	graph: &SemanticGraph,
) -> Option<String> {
	let callers = extract_caller_names(name, graph);
	if callers.is_empty() {
		return None;
	}
	Some(format_caller_text(&callers))
}

/// Extract unique caller module names for a symbol.
///
/// Filters references to `name` for Call context,
/// extracts file stem as module name, deduplicates,
/// and returns up to MAX_CALLERS names sorted
/// alphabetically.
pub fn extract_caller_names(
	name: &str,
	graph: &SemanticGraph,
) -> Vec<String> {
	let refs = graph.find_references(name);

	let mut modules: Vec<String> = refs
		.iter()
		.filter(|ref_item| {
			ref_item.context == ReferenceContext::Call
		})
		.map(|ref_item| {
			file_context(&ref_item.location.file)
		})
		.collect();

	modules.sort();
	modules.dedup();
	modules.truncate(MAX_CALLERS);
	modules
}

/// Pre-compute caller context strings for all symbols.
///
/// Iterates symbols, builds caller context for each,
/// and returns a map of symbol name → context string.
pub fn precompute_caller_contexts(
	symbols: &[Symbol],
	graph: &SemanticGraph,
) -> HashMap<String, String> {
	symbols
		.iter()
		.filter_map(|sym| {
			let ctx = caller_context(&sym.name, graph)?;
			Some((sym.name.clone(), ctx))
		})
		.collect()
}

/// Format caller module names into embedding text.
///
/// Joins module names with comma separator and
/// prepends the "called from" prefix.
fn format_caller_text(callers: &[String]) -> String {
	let joined = callers.join(CALLER_SEP);
	format!("{}{}", CALLER_PREFIX, joined)
}
