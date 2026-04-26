use std::io::Write;

use crate::service::search::types::SearchResult;

use super::search_helpers::print_full_content;

/// Format and display search results to stdout
pub(crate) fn format_search_output(
	query: &str,
	result: &SearchResult,
	full: bool,
) {
	let mut out = std::io::stdout().lock();
	if result.hits.is_empty() {
		writeln!(
			out, "No results found for '{}'", query
		)
		.ok();
		return;
	}
	writeln!(
		out, "Search results for '{}':\n", query
	)
	.ok();
	for (idx, hit) in
		result.hits.iter().enumerate()
	{
		print_hit(&mut out, idx + 1, hit);
	}
	if full {
		print_full_content(&result.hits);
	}
}

/// Print a single search hit line
fn print_hit(
	out: &mut impl Write,
	rank: usize,
	hit: &crate::service::search::types
		::SearchResultHit,
) {
	let file = hit.symbol.location.file
		.file_name()
		.and_then(|fname| fname.to_str())
		.unwrap_or("?");
	writeln!(
		out, "  {}. {} {} ({}:{}) [{:.4}]",
		rank, hit.symbol.kind, hit.symbol.name,
		file, hit.symbol.location.line, hit.score,
	).ok();
	if let Some(ref sig) = hit.symbol.signature {
		writeln!(out, "     {}", sig).ok();
	}
}
