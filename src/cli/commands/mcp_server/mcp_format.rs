//! Format memory results as human-readable text
//! for MCP tool responses.

use crate::domain::memory::Interaction;
use crate::domain::memory_helpers;
use crate::indexer::memory::types::{
	MemoryHit, MemoryStats,
};

/// No results placeholder
const NO_RESULTS: &str = "(no results)";

/// Format a list of interactions as text
pub fn format_interactions(
	items: &[Interaction],
) -> String {
	if items.is_empty() {
		return NO_RESULTS.to_string();
	}
	items.iter().map(format_one).collect::<Vec<_>>()
		.join("\n---\n")
}

/// Format search hits with scores
pub fn format_search_hits(
	hits: &[MemoryHit],
) -> String {
	if hits.is_empty() {
		return NO_RESULTS.to_string();
	}
	hits.iter()
		.map(|hit| {
			format!(
				"[{:.3}] {}",
				hit.score,
				format_one(&hit.interaction),
			)
		})
		.collect::<Vec<_>>()
		.join("\n---\n")
}

/// Format memory stats as text
pub fn format_stats(stats: &MemoryStats) -> String {
	format!(
		"Total: {}\nSessions: {}\n\
		 Size: {} bytes\nLast updated: {}",
		stats.total,
		stats.sessions,
		stats.size_bytes,
		stats.last_updated,
	)
}

/// Format a single interaction
fn format_one(item: &Interaction) -> String {
	let rtype = memory_helpers::response_type_label(
		&item.response,
	);
	format!(
		"[{}] session={}\nInput: {}\nResponse: {}",
		rtype,
		item.session_id,
		item.input.text,
		response_text(&item.response),
	)
}

/// Extract text from any AiResponse variant
fn response_text(
	resp: &crate::domain::memory::AiResponse,
) -> &str {
	match resp {
		crate::domain::memory::AiResponse::Answer {
			text,
		} => text,
		crate::domain::memory::AiResponse::Question {
			text,
		} => text,
		crate::domain::memory::AiResponse::CodeChange {
			text, ..
		} => text,
	}
}
