//! Query Interpretation Logic
//!
//! Handles interpreting natural language queries into SearchSpec.

use super::core::QueryInterpreter;
use super::super::{fallback_parse, parse_llm_response};
use crate::retrieval::daemon::protocol::SearchSpec;

impl QueryInterpreter {
	/// Interpret a natural language query into SearchSpec
	pub fn interpret(&mut self, query: &str) -> SearchSpec {
		// try LLM expansion if available
		if let Some(ref mut model) = self.model {
			match model.expand_query(query) {
				Ok(response) => {
					// try to parse LLM response
					let spec =
						parse_llm_response(query, &response);
					// if we got useful symbols, return it
					if !spec.symbol_names.is_empty() {
						return spec;
					}
				}
				Err(e) => {
					eprintln!(
						"[interpreter] LLM error: {}, fallback",
						e
					);
				}
			}
		}

		// fallback: simple heuristic parsing
		fallback_parse(query)
	}

	/// Interpret without LLM (always use fallback)
	pub fn interpret_simple(query: &str) -> SearchSpec {
		fallback_parse(query)
	}
}
