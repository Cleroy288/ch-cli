//! Query expansion methods for the retrieval pipeline.
//!
//! Handles tiered, LLM, and simple fallback expansion.

use std::io::Write;

use crate::retrieval::daemon::protocol::{
	QueryIntent, SearchSpec,
};
use crate::retrieval::query::{
	fallback_parse, TieredConfig, TieredQueryExpander,
};

use super::core::RetrievalPipeline;

impl RetrievalPipeline {
	/// Expand query: tiered, LLM, or simple fallback
	pub(super) fn expand_for_retrieval(
		&self,
		query: &str,
	) -> SearchSpec {
		if !self.config.flags.expand_query {
			return simple_search_spec(query);
		}
		if !self.config.flags.tiered_expansion {
			let _ = writeln!(
				std::io::stderr().lock(),
				"[pipeline] Expanding via LLM..."
			);
			return self.expand_query_safe(query);
		}
		self.tiered_expand(query)
	}

	/// Use tiered expansion with fast-path
	fn tiered_expand(
		&self,
		query: &str,
	) -> SearchSpec {
		let config = self.build_tiered_config();
		let expander = TieredQueryExpander::with_config(
			&self.daemon,
			self.graph.as_deref(),
			config,
		);
		match expander.expand(query) {
			Ok(result) => {
				log_tiered_result(&result);
				result.spec
			}
			Err(err) => {
				let _ = writeln!(
					std::io::stderr().lock(),
					"[pipeline] Tiered failed: {}, \
					using fallback",
					err
				);
				self.fallback_query_expansion(query)
			}
		}
	}

	/// Build tiered config from pipeline settings
	fn build_tiered_config(&self) -> TieredConfig {
		TieredConfig {
			confidence_threshold:
				self.config.fast_path_threshold,
			importance_threshold:
				self.config.importance_threshold,
			existence_threshold: 0.5,
		}
	}
}

/// Log tiered expansion result to stderr
fn log_tiered_result(
	result: &crate::retrieval::query::TieredResult,
) {
	let _ = writeln!(
		std::io::stderr().lock(),
		"[pipeline] Tiered: {:?} in {}ms",
		result.tier_used, result.time_ms
	);
}

/// Build a simple search spec without expansion
pub(super) fn simple_search_spec(
	query: &str,
) -> SearchSpec {
	SearchSpec {
		original_query: query.to_string(),
		symbol_names: query
			.split_whitespace()
			.map(String::from)
			.collect(),
		intent: QueryIntent::Search,
		file_filters: Vec::new(),
		context_hints: Vec::new(),
	}
}

/// Expand query for structured pipeline (simplified)
pub(super) fn expand_for_structured(
	pipeline: &RetrievalPipeline,
	query: &str,
) -> SearchSpec {
	if pipeline.config.flags.expand_query {
		pipeline.expand_query_safe(query)
	} else {
		fallback_parse(query)
	}
}
