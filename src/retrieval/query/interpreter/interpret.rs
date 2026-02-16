//! Query Interpretation Logic
//!
//! Handles interpreting natural language queries
//! into SearchSpec.

use std::io::Write;

use super::core::QueryInterpreter;
use super::super::{fallback_parse, parse_llm_response};
use crate::retrieval::daemon::protocol::SearchSpec;

impl QueryInterpreter {
	/// Interpret a natural language query into SearchSpec
	pub fn interpret(
		&mut self,
		query: &str,
	) -> SearchSpec {
		if let Some(spec) = self.try_llm_expand(query)
		{
			return spec;
		}
		fallback_parse(query)
	}

	/// Try LLM expansion, return None on failure
	fn try_llm_expand(
		&mut self,
		query: &str,
	) -> Option<SearchSpec> {
		let model = self.model.as_mut()?;
		let response =
			match model.expand_query(query) {
				Ok(res) => res,
				Err(err) => {
					let _ = writeln!(
						std::io::stderr().lock(),
						"[interpreter] LLM error: {}",
						err
					);
					return None;
				}
			};
		let spec = parse_llm_response(query, &response);
		if spec.symbol_names.is_empty() {
			return None;
		}
		Some(spec)
	}

	/// Interpret without LLM (always use fallback)
	pub fn interpret_simple(query: &str) -> SearchSpec {
		fallback_parse(query)
	}
}
