//! LLM expansion with query rewriting

use std::time::Instant;

use crate::retrieval::daemon::protocol::SearchSpec;
use crate::retrieval::query::rewriter::{
	QueryRewriter, RewrittenQuery,
};
use crate::retrieval::query::tiered::{
	TieredQueryExpander, TieredResult, TierUsed,
};
use crate::retrieval::RetrievalResult;

use super::intent::detect_intent;

/// Expand using LLM with query rewriting
pub(crate) fn expand_with_llm_impl(
	expander: &TieredQueryExpander,
	query: &str,
	start: Instant,
) -> RetrievalResult<TieredResult> {
	let rewriter = QueryRewriter::new(expander.daemon);
	let rewrites = rewriter.rewrite(query)?;

	let all_symbols =
		collect_rewrite_symbols(expander, &rewrites);

	if !all_symbols.is_empty() {
		return build_rewrite_result(
			query,
			all_symbols,
			&rewrites,
			start,
		);
	}

	fallback_direct_llm(expander, query, start)
}

/// Fallback to direct LLM expansion
fn fallback_direct_llm(
	expander: &TieredQueryExpander,
	query: &str,
	start: Instant,
) -> RetrievalResult<TieredResult> {
	let spec = expander.daemon.expand(query.to_string())?;
	let time_ms = start.elapsed().as_millis() as u64;

	Ok(TieredResult {
		spec,
		tier_used: TierUsed::LlmExpansion,
		time_ms,
	})
}

/// Collect symbols from rewritten queries
fn collect_rewrite_symbols(
	expander: &TieredQueryExpander,
	rewrites: &[RewrittenQuery],
) -> Vec<String> {
	let mut all_symbols: Vec<String> = Vec::new();
	for rewrite in rewrites {
		let fast_result =
			expander.parser.extract_symbols(&rewrite.text);
		for sym in fast_result.symbols {
			if !all_symbols.contains(&sym.name) {
				all_symbols.push(sym.name);
			}
		}
	}
	all_symbols
}

/// Build TieredResult from rewrite-extracted symbols
fn build_rewrite_result(
	query: &str,
	symbols: Vec<String>,
	rewrites: &[RewrittenQuery],
	start: Instant,
) -> RetrievalResult<TieredResult> {
	let hints: Vec<String> =
		rewrites.iter().map(|r| r.text.clone()).collect();

	let spec = SearchSpec {
		original_query: query.to_string(),
		symbol_names: symbols,
		intent: detect_intent(query),
		file_filters: Vec::new(),
		context_hints: hints,
	};
	let time_ms = start.elapsed().as_millis() as u64;

	Ok(TieredResult {
		spec,
		tier_used: TierUsed::LlmExpansion,
		time_ms,
	})
}
