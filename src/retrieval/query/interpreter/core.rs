//! Core QueryInterpreter Implementation
//!
//! Provides the main interpreter structure and constructors.

use super::super::llm::Phi3Model;
use crate::retrieval::models::ModelResult;

/// Query interpreter that uses LLM for expansion
pub struct QueryInterpreter {
	/// the LLM model (None if not loaded)
	pub(super) model: Option<Phi3Model>,
}

impl QueryInterpreter {
	/// Create interpreter without loading model
	pub fn new() -> Self {
		Self { model: None }
	}

	/// Create interpreter with LLM loaded
	pub fn with_llm() -> ModelResult<Self> {
		let model = Phi3Model::new()?;
		Ok(Self { model: Some(model) })
	}

	/// Create interpreter with specific model ID
	pub fn with_model_id(
		model_id: &str,
	) -> ModelResult<Self> {
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
}

impl Default for QueryInterpreter {
	fn default() -> Self {
		Self::new()
	}
}
