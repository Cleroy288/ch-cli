//! Feedback Action String Parsing
//!
//! Parses user input strings into FeedbackAction variants.
//! Supports commands like "more", "search", "filter", "related".

use super::feedback_actions::FeedbackAction;

/// Parse a feedback action from string input
pub fn parse_feedback_action(
	input: &str,
) -> Option<FeedbackAction> {
	let input = input.trim().to_lowercase();

	if input == "accept"
		|| input == "done"
		|| input == "ok"
	{
		return Some(FeedbackAction::Accept);
	}

	if let Some(action) = try_parse_more_context(&input) {
		return Some(action);
	}

	if let Some(action) = try_parse_search(&input) {
		return Some(action);
	}

	if let Some(action) = try_parse_filter(&input) {
		return Some(action);
	}

	if let Some(action) = try_parse_related(&input) {
		return Some(action);
	}

	None
}

/// Try to parse a "more" or "expand" command
fn try_parse_more_context(
	input: &str,
) -> Option<FeedbackAction> {
	if !input.starts_with("more ")
		&& !input.starts_with("expand ")
	{
		return None;
	}

	let symbols: Vec<String> = input
		.split_whitespace()
		.skip(1)
		.map(String::from)
		.collect();

	if symbols.is_empty() {
		return None;
	}

	Some(FeedbackAction::MoreContext { symbols })
}

/// Try to parse a "search" or "find" command
fn try_parse_search(
	input: &str,
) -> Option<FeedbackAction> {
	if !input.starts_with("search ")
		&& !input.starts_with("find ")
	{
		return None;
	}

	let query = input
		.split_whitespace()
		.skip(1)
		.collect::<Vec<_>>()
		.join(" ");

	if query.is_empty() {
		return None;
	}

	Some(FeedbackAction::RefineSearch { query })
}

/// Try to parse a "filter" or "in" command
fn try_parse_filter(
	input: &str,
) -> Option<FeedbackAction> {
	if !input.starts_with("filter ")
		&& !input.starts_with("in ")
	{
		return None;
	}

	let patterns: Vec<String> = input
		.split_whitespace()
		.skip(1)
		.map(String::from)
		.collect();

	if patterns.is_empty() {
		return None;
	}

	Some(FeedbackAction::FilterFiles { patterns })
}

/// Try to parse a "related" command
fn try_parse_related(
	input: &str,
) -> Option<FeedbackAction> {
	if !input.starts_with("related ") {
		return None;
	}

	let symbols: Vec<String> = input
		.split_whitespace()
		.skip(1)
		.map(String::from)
		.collect();

	if symbols.is_empty() {
		return None;
	}

	Some(FeedbackAction::ExpandRelated { symbols })
}
