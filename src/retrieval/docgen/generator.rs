//! Documentation Generator using local LLM.
//!
//! Core struct definition and model management.

use crate::retrieval::models::ModelResult;
use crate::retrieval::query::Phi3Model;

/// Maximum tokens to generate for documentation.
pub(crate) const MAX_DOC_TOKENS: usize = 200;

/// Number of context lines to extract around a symbol.
pub(crate) const CONTEXT_LINES: usize = 30;

/// Documentation generator using local LLM.
pub struct DocGenerator {
	/// the LLM model
	pub(crate) model: Option<Phi3Model>,
}

impl DocGenerator {
	/// Create generator without loading model.
	pub fn new() -> Self {
		Self { model: None }
	}

	/// Create generator with default Phi-3 model.
	pub fn with_default_model() -> ModelResult<Self> {
		let model = Phi3Model::new()?;
		Ok(Self { model: Some(model) })
	}

	/// Create generator with specific model ID.
	pub fn with_model_id(
		model_id: &str,
	) -> ModelResult<Self> {
		let model = Phi3Model::from_model_id(model_id)?;
		Ok(Self { model: Some(model) })
	}

	/// Set the model.
	pub fn set_model(&mut self, model: Phi3Model) {
		self.model = Some(model);
	}

	/// Check if model is loaded.
	pub fn is_ready(&self) -> bool {
		self.model.is_some()
	}
}

impl Default for DocGenerator {
	fn default() -> Self {
		Self::new()
	}
}

