//! Symbol validation logic

use std::time::Instant;

use crate::indexer::SemanticGraph;
use crate::retrieval::RetrievalResult;

use super::expander::TieredQueryExpander;
use super::result::{TieredResult, TierUsed};
use super::super::validator::SymbolValidator;

/// Try validated fast-path, fallback to unvalidated
pub(crate) fn try_validated_path(
	expander: &TieredQueryExpander,
	query: &str,
	names: Vec<String>,
	start: Instant,
) -> RetrievalResult<TieredResult> {
	if let Some(graph) = expander.graph {
		return validated_expand(
			expander, query, names, graph, start,
		);
	}

	let spec = expander.build_fast_path_spec(query, names);
	let time_ms = start.elapsed().as_millis() as u64;

	Ok(TieredResult {
		spec,
		tier_used: TierUsed::FastPathUnvalidated,
		time_ms,
	})
}

/// Expand with graph validation
fn validated_expand(
	expander: &TieredQueryExpander,
	query: &str,
	names: Vec<String>,
	graph: &SemanticGraph,
	start: Instant,
) -> RetrievalResult<TieredResult> {
	let validator = SymbolValidator::new(graph);
	let validation = validator.validate_symbols(&names);

	let below_existence = validation.existence_ratio
		< expander.config.existence_threshold;
	let below_importance = validation.avg_importance
		< expander.config.importance_threshold;

	if below_existence || below_importance {
		return expander.expand_with_llm(query, start);
	}

	let validated: Vec<String> = validation
		.symbols
		.iter()
		.filter(|s| s.exists)
		.map(|s| s.name.clone())
		.collect();

	let spec = expander.build_fast_path_spec(query, validated);
	let time_ms = start.elapsed().as_millis() as u64;

	Ok(TieredResult {
		spec,
		tier_used: TierUsed::FastPathValidated,
		time_ms,
	})
}
