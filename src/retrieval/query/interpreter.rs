//! Query Interpreter
//!
//! Uses the Phi-3 LLM to interpret natural language queries
//! and convert them to structured SearchSpec.

use super::llm::Phi3Model;
use super::{fallback_parse, parse_llm_response};
use crate::retrieval::daemon::protocol::SearchSpec;
use crate::retrieval::models::ModelResult;

/// Query interpreter that uses LLM for expansion
pub struct QueryInterpreter {
	/// the LLM model (None if not loaded)
	model: Option<Phi3Model>,
}

impl QueryInterpreter {
	/// Create interpreter without loading model (use fallback parsing)
	pub fn new() -> Self {
		Self { model: None }
	}

	/// Create interpreter with LLM loaded
	pub fn with_llm() -> ModelResult<Self> {
		let model = Phi3Model::new()?;
		Ok(Self { model: Some(model) })
	}

	/// Create interpreter with specific model ID
	pub fn with_model_id(model_id: &str) -> ModelResult<Self> {
		let model = Phi3Model::from_model_id(model_id)?;
		Ok(Self { model: Some(model) })
	}

	/// Set the LLM model
	pub fn set_model(&mut self, model: Phi3Model) {
		self.model = Some(model);
	}

	/// Check if LLM is available
	pub fn has_llm(&self) -> bool {
		self.model.is_some()
	}

	/// Interpret a natural language query into SearchSpec
	pub fn interpret(&mut self, query: &str) -> SearchSpec {
		// try LLM expansion if available
		if let Some(ref mut model) = self.model {
			match model.expand_query(query) {
				Ok(response) => {
					// try to parse LLM response
					let spec = parse_llm_response(query, &response);
					// if we got useful symbols, return it
					if !spec.symbol_names.is_empty() {
						return spec;
					}
				}
				Err(e) => {
					eprintln!("[interpreter] LLM error: {}, using fallback", e);
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

impl Default for QueryInterpreter {
	fn default() -> Self {
		Self::new()
	}
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test_interpret_simple() {
		let spec = QueryInterpreter::interpret_simple("where is AuthService defined");
		assert!(spec.symbol_names.contains(&"AuthService".to_string()));
	}

	#[test]
	fn test_interpreter_fallback() {
		let mut interp = QueryInterpreter::new();
		let spec = interp.interpret("find the parse_config function");
		// should use fallback since no LLM loaded
		assert!(spec.symbol_names.contains(&"parse_config".to_string()));
	}
}
