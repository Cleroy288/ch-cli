//! Ref count helpers for hub penalty.
//!
//! Pre-computes reference counts from SemanticGraph
//! so ref_count_penalty() uses real call-graph data.

use std::collections::HashMap;

use crate::indexer::semantic::SemanticGraph;

/// Pre-compute ref counts from SemanticGraph.
/// Maps symbol name → number of references.
pub fn precompute_ref_counts(
	graph: &SemanticGraph,
) -> HashMap<String, usize> {
	graph
		.all_symbol_names()
		.into_iter()
		.map(|name| {
			let count = graph.find_references(name).len();
			(name.clone(), count)
		})
		.collect()
}

/// Look up ref count for a symbol name.
/// Returns 0 if not found.
pub fn lookup_ref_count(
	name: &str,
	ref_counts: &HashMap<String, usize>,
) -> usize {
	ref_counts.get(name).copied().unwrap_or(0)
}
