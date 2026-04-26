use crate::indexer::IndexStats;
use crate::service::search::types_navigation::{
	ReferenceResult, UsageLocation,
};

/// No results placeholder
const NO_RESULTS: &str = "(no results)";

pub fn format_references(
	result: &ReferenceResult,
) -> String {
	let mut out = String::new();
	if !result.definitions.is_empty() {
		out.push_str("Definitions:\n");
		append_locations(
			&mut out, &result.definitions,
		);
	}
	if !result.references.is_empty() {
		out.push_str("References:\n");
		append_locations(
			&mut out, &result.references,
		);
	}
	if out.is_empty() {
		return NO_RESULTS.to_string();
	}
	out
}

pub fn format_index_stats(
	stats: &IndexStats,
) -> String {
	format!(
		"Root: {}\nFiles: {}\nSymbols: {}\n\
		 Updated: {}\nVersion: {}",
		stats.root.display(),
		stats.file_count,
		stats.symbol_count,
		stats.last_updated,
		stats.version,
	)
}

/// Append usage locations to output
fn append_locations(
	out: &mut String,
	locs: &[UsageLocation],
) {
	for loc in locs {
		out.push_str(&format!(
			"  {}:{}\n",
			loc.file.display(),
			loc.line,
		));
	}
}
