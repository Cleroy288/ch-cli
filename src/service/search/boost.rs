use std::collections::HashMap;

use crate::indexer::semantic::SemanticGraph;

pub use crate::domain::query_intent::QueryIntent;

const HIGH_REF_THRESHOLD: usize = 20;
const MID_REF_THRESHOLD: usize = 10;
const HIGH_REF_PENALTY: f32 = 0.5;
const MID_REF_PENALTY: f32 = 0.7;

/// Hub functions (called from many places) get
/// de-ranked so leaf implementations rank higher.
pub fn ref_count_penalty(ref_count: usize) -> f32 {
	if ref_count > HIGH_REF_THRESHOLD {
		return HIGH_REF_PENALTY;
	}
	if ref_count > MID_REF_THRESHOLD {
		return MID_REF_PENALTY;
	}
	1.0
}

pub fn precompute_ref_counts(
	graph: &SemanticGraph,
) -> HashMap<String, usize> {
	graph
		.all_symbol_names()
		.into_iter()
		.map(|name| {
			let count =
				graph.find_references(name).len();
			(name.clone(), count)
		})
		.collect()
}

pub fn lookup_ref_count(
	name: &str,
	ref_counts: &HashMap<String, usize>,
) -> usize {
	ref_counts.get(name).copied().unwrap_or(0)
}
