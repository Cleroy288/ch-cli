//! Routes a search query to the right handler.
//! Order: structure > caller > text (fallback).

use crate::service::search::caller
	::detect_caller_query;
use crate::service::search::structure_query
	::detect_structure_query;

use super::error::CommandResult;
use super::search::SearchCommandOptions;
use super::search_helpers::handle_structure_query;

/// Route query to first matching search path.
pub fn dispatch(
	query: &str,
	opts: &SearchCommandOptions,
) -> CommandResult {
	if let Some(sq) = detect_structure_query(query) {
		return handle_structure_query(&sq.target);
	}
	if let Some(cq) = detect_caller_query(query) {
		return super::search::route_caller_query(&cq);
	}
	super::search::run_text_search(query, opts)
}
