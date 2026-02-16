//! Fallback Intent Detection
//!
//! Detects query intent from keywords when LLM fails.

use crate::retrieval::daemon::protocol::QueryIntent;

use super::super::parsing::contains_word;

/// Intent check: (detection fn, resulting intent)
type IntentCheck = (fn(&str) -> bool, QueryIntent);

/// Detect intent from query keywords
pub(super) fn detect_fallback_intent(
	query: &str,
) -> QueryIntent {
	let lower = query.to_lowercase();
	match_intent(&lower)
}

/// Match intent via lookup table pattern
fn match_intent(lower: &str) -> QueryIntent {
	let checks: &[IntentCheck] = &[
		(is_definition_intent, QueryIntent::FindDefinition),
		(is_usages_intent, QueryIntent::FindUsages),
		(is_understand_intent, QueryIntent::Understand),
		(is_modify_intent, QueryIntent::Modify),
		(is_debug_intent, QueryIntent::Debug),
	];

	for (check, intent) in checks {
		if check(lower) {
			return intent.clone();
		}
	}
	QueryIntent::Search
}

/// Check for definition-related keywords
fn is_definition_intent(lower: &str) -> bool {
	contains_word(lower, "where")
		|| contains_word(lower, "definition")
		|| contains_word(lower, "defined")
}

/// Check for usage-related keywords
fn is_usages_intent(lower: &str) -> bool {
	contains_word(lower, "used")
		|| contains_word(lower, "calls")
		|| contains_word(lower, "references")
		|| contains_word(lower, "usages")
}

/// Check for understanding-related keywords
fn is_understand_intent(lower: &str) -> bool {
	contains_word(lower, "how")
		|| contains_word(lower, "what")
		|| contains_word(lower, "explain")
		|| contains_word(lower, "understand")
}

/// Check for modification-related keywords
fn is_modify_intent(lower: &str) -> bool {
	contains_word(lower, "fix")
		|| contains_word(lower, "change")
		|| contains_word(lower, "modify")
		|| contains_word(lower, "update")
}

/// Check for debug-related keywords
fn is_debug_intent(lower: &str) -> bool {
	contains_word(lower, "bug")
		|| contains_word(lower, "error")
		|| contains_word(lower, "debug")
		|| contains_word(lower, "issue")
}
