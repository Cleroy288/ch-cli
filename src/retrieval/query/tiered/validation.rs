//! Symbol validation logic

use std::time::Instant;

use crate::indexer::SemanticGraph;
use crate::retrieval::RetrievalResult;

use super::expander::TieredQueryExpander;
use super::result::{TieredResult, TierUsed};
use super::super::validator::SymbolValidator;

/// Input for validated expansion
pub(crate) struct ValidationInput {
	/// extracted symbol names
	pub names: Vec<String>,
	/// timestamp when expansion started
	pub start: Instant,
}

/// Try validated fast-path, fallback to unvalidated
pub(crate) fn try_validated_path(
	expander: &TieredQueryExpander,
	query: &str,
	input: ValidationInput,
) -> RetrievalResult<TieredResult> {
	if let Some(graph) = expander.graph {
		return validated_expand(
			expander, query, graph, input,
		);
	}

	let spec = expander
		.build_fast_path_spec(query, input.names);
	let time_ms =
		input.start.elapsed().as_millis() as u64;

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
	graph: &SemanticGraph,
	input: ValidationInput,
) -> RetrievalResult<TieredResult> {
	let validator = SymbolValidator::new(graph);
	let validation =
		validator.validate_symbols(&input.names);

	if needs_llm_fallback(expander, &validation) {
		return expander.expand_with_llm(
			query, input.start,
		);
	}

	let validated: Vec<String> = validation
		.symbols
		.iter()
		.filter(|sym| sym.exists)
		.map(|sym| sym.name.clone())
		.collect();

	let spec = expander
		.build_fast_path_spec(query, validated);
	let time_ms =
		input.start.elapsed().as_millis() as u64;

	Ok(TieredResult {
		spec,
		tier_used: TierUsed::FastPathValidated,
		time_ms,
	})
}

/// Check if validation results require LLM fallback
fn needs_llm_fallback(
	expander: &TieredQueryExpander,
	validation: &super::super::validator::ValidationResult,
) -> bool {
	let below_existence = validation.existence_ratio
		< expander.config.existence_threshold;
	let below_importance = validation.avg_importance
		< expander.config.importance_threshold;
	below_existence || below_importance
}
