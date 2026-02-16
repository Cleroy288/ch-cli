//! Semantic search formatting helpers.
//!
//! Display logic for hybrid search results.
//! Business logic (search, boosting, reranking) now
//! lives in service::search::semantic_impl.

use std::io::Write;

use crate::service::search::types::SearchResultHit;

/// Format a semantic result line with rank info
pub fn format_semantic_hit(
	num: usize,
	hit: &SearchResultHit,
) {
	let file = extract_filename(hit);
	let ranks = format_rank_info(hit);

	let mut out = std::io::stdout().lock();
	writeln!(
		out,
		"  {}. {} {} ({}:{}) \
		[{:.4} {}]",
		num,
		hit.symbol.kind,
		hit.symbol.name,
		file,
		hit.symbol.location.line,
		hit.score,
		ranks,
	)
	.ok();
	if let Some(ref sig) = hit.symbol.signature {
		writeln!(out, "     {}", sig).ok();
	}
}

/// Extract display filename from a hit
fn extract_filename(
	hit: &SearchResultHit,
) -> &str {
	hit.symbol
		.location
		.file
		.file_name()
		.and_then(|fname| fname.to_str())
		.unwrap_or("?")
}

/// Format rank labels for display
fn format_rank_info(
	hit: &SearchResultHit,
) -> String {
	let kw_rank = hit
		.keyword_rank
		.map(|rank| format!("K:{}", rank))
		.unwrap_or_else(|| "-".to_string());
	let sem_rank = hit
		.semantic_rank
		.map(|rank| format!("S:{}", rank))
		.unwrap_or_else(|| "-".to_string());
	let rr_score = hit
		.rerank_score
		.map(|score| format!(" R:{:.3}", score))
		.unwrap_or_default();
	format!("{} {}{}", kw_rank, sem_rank, rr_score)
}
