use crate::domain::memory::Interaction;
use crate::domain::memory_helpers;
use crate::indexer::memory::types::{
	MemoryHit, MemoryStats,
};

/// No results placeholder
const NO_RESULTS: &str = "(no results)";

pub fn format_interactions(
	items: &[Interaction],
) -> String {
	if items.is_empty() {
		return NO_RESULTS.to_string();
	}
	items.iter().map(format_one).collect::<Vec<_>>()
		.join("\n---\n")
}

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

fn response_text(
	resp: &crate::domain::memory::AiResponse,
) -> &str {
	use crate::domain::memory::AiResponse;
	match resp {
		AiResponse::Answer { text }
		| AiResponse::Question { text }
		| AiResponse::CodeChange { text, .. } => {
			text
		}
	}
}
