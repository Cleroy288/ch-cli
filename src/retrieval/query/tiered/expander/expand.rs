//! Query Expansion Logic
//!
//! Handles the tiered expansion strategy.

use std::time::Instant;

use crate::retrieval::RetrievalResult;

use super::core::TieredQueryExpander;
use super::super::result::TieredResult;
use super::super::validation::try_validated_path;

impl<'a> TieredQueryExpander<'a> {
	/// Expand query using tiered approach
	pub fn expand(
		&self,
		query: &str,
	) -> RetrievalResult<TieredResult> {
		let start = Instant::now();
		let fast_result = self.parser.extract_symbols(query);

		if should_use_llm(&fast_result, &self.config) {
			return self.expand_with_llm(query, start);
		}

		let names: Vec<String> = fast_result
			.symbols
			.iter()
			.map(|s| s.name.clone())
			.collect();

		try_validated_path(self, query, names, start)
	}

	/// Expand with LLM (delegates to strategy)
	pub(crate) fn expand_with_llm(
		&self,
		query: &str,
		start: Instant,
	) -> RetrievalResult<TieredResult> {
		crate::retrieval::query::tiered_strategies::expand_with_llm_impl(
			self, query, start,
		)
	}

	/// Build fast path spec (delegates to strategy)
	pub(crate) fn build_fast_path_spec(
		&self,
		query: &str,
		symbols: Vec<String>,
	) -> crate::retrieval::daemon::protocol::SearchSpec {
		crate::retrieval::query::tiered_strategies::build_fast_path_spec_impl(
			query, symbols,
		)
	}
}

/// Check if query should use LLM
fn should_use_llm(
	fast_result: &super::super::super::fast_path::FastPathResult,
	config: &super::super::config::TieredConfig,
) -> bool {
	use super::super::super::fast_path::FastPathIntent;

	fast_result.intent == FastPathIntent::Conceptual
		|| fast_result.symbols.is_empty()
		|| fast_result.confidence < config.confidence_threshold
		|| fast_result.intent == FastPathIntent::Mixed
}
