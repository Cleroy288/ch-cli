//! Query intent classification

use super::super::fast_path_patterns::{
	CONCEPTUAL_PATTERNS, FastPathIntent,
};

/// Check if query is conceptual (needs LLM)
pub(crate) fn is_conceptual_query(query: &str) -> bool {
	let lower = query.to_lowercase();
	CONCEPTUAL_PATTERNS.iter().any(|p| lower.contains(p))
}

/// Classify query intent based on conceptual flag
pub(crate) fn classify_intent(
	is_conceptual: bool,
	no_symbols: bool,
) -> FastPathIntent {
	if is_conceptual {
		if no_symbols {
			FastPathIntent::Conceptual
		} else {
			FastPathIntent::Mixed
		}
	} else if no_symbols {
		FastPathIntent::Conceptual
	} else {
		FastPathIntent::Explicit
	}
}
