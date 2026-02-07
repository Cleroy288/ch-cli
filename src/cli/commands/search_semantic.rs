//! Semantic search formatting helpers.
//!
//! Display logic for hybrid search results.
//! Business logic (search, boosting, reranking) now
//! lives in service::search::semantic_impl.

use crate::service::search::types::SearchResultHit;

/// Format a semantic result line with rank info
pub fn format_semantic_hit(
	num: usize,
	hit: &SearchResultHit,
) {
	let file = hit
		.symbol
		.location
		.file
		.file_name()
		.and_then(|f| f.to_str())
		.unwrap_or("?");

	let kw = hit
		.keyword_rank
		.map(|r| format!("K:{}", r))
		.unwrap_or_else(|| "-".to_string());
	let sem = hit
		.semantic_rank
		.map(|r| format!("S:{}", r))
		.unwrap_or_else(|| "-".to_string());
	let rr = hit
		.rerank_score
		.map(|s| format!(" R:{:.3}", s))
		.unwrap_or_default();

	println!(
		"  {}. {} {} ({}:{}) \
		[{:.4} {} {}{}]",
		num,
		hit.symbol.kind,
		hit.symbol.name,
		file,
		hit.symbol.location.line,
		hit.score,
		kw,
		sem,
		rr
	);

	if let Some(ref sig) = hit.symbol.signature {
		println!("     {}", sig);
	}
}
