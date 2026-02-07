//! Fallback Intent Detection
//!
//! Detects query intent from keywords when LLM fails.

use crate::retrieval::daemon::protocol::QueryIntent;

use super::super::parsing::contains_word;

/// Detect intent from query keywords using word boundaries
pub(super) fn detect_fallback_intent(
	query: &str,
) -> QueryIntent {
	let q = query.to_lowercase();

	if contains_word(&q, "where")
		|| contains_word(&q, "definition")
		|| contains_word(&q, "defined")
	{
		QueryIntent::FindDefinition
	} else if contains_word(&q, "used")
		|| contains_word(&q, "calls")
		|| contains_word(&q, "references")
		|| contains_word(&q, "usages")
	{
		QueryIntent::FindUsages
	} else if contains_word(&q, "how")
		|| contains_word(&q, "what")
		|| contains_word(&q, "explain")
		|| contains_word(&q, "understand")
	{
		QueryIntent::Understand
	} else if contains_word(&q, "fix")
		|| contains_word(&q, "change")
		|| contains_word(&q, "modify")
		|| contains_word(&q, "update")
	{
		QueryIntent::Modify
	} else if contains_word(&q, "bug")
		|| contains_word(&q, "error")
		|| contains_word(&q, "debug")
		|| contains_word(&q, "issue")
	{
		QueryIntent::Debug
	} else {
		QueryIntent::Search
	}
}
