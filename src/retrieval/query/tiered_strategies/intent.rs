//! Query intent detection

use crate::retrieval::daemon::protocol::QueryIntent;

/// Detect query intent from keywords
#[doc(hidden)]
pub fn detect_intent(query: &str) -> QueryIntent {
	let lower = query.to_lowercase();

	if is_definition_query(&lower) {
		QueryIntent::FindDefinition
	} else if is_usages_query(&lower) {
		QueryIntent::FindUsages
	} else if is_understand_query(&lower) {
		QueryIntent::Understand
	} else {
		QueryIntent::Search
	}
}

/// Check if query asks for definition
fn is_definition_query(lower: &str) -> bool {
	lower.contains("where")
		|| lower.contains("definition")
		|| lower.contains("defined")
}

/// Check if query asks for usages
fn is_usages_query(lower: &str) -> bool {
	lower.contains("used")
		|| lower.contains("calls")
		|| lower.contains("references")
}

/// Check if query asks for understanding
fn is_understand_query(lower: &str) -> bool {
	lower.contains("how")
		|| lower.contains("what")
		|| lower.contains("explain")
}
