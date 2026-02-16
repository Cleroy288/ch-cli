//! Documentation Generator using local LLM.
//!
//! Core struct definition and model management.

use crate::retrieval::docgen::doc_llm::DocLlm;
use crate::retrieval::models::ModelResult;

/// Maximum tokens to generate for documentation.
pub(crate) const MAX_DOC_TOKENS: usize = 60;

/// Number of context lines to extract around a symbol.
pub(crate) const CONTEXT_LINES: usize = 15;

/// Documentation generator using local LLM.
pub struct DocGenerator {
	/// the LLM model (Qwen2.5-0.5B)
	pub(crate) model: Option<DocLlm>,
}

impl DocGenerator {
	/// Create generator without loading model.
	pub fn new() -> Self {
		Self { model: None }
	}

	/// Create generator with default Qwen2.5 model.
	pub fn with_default_model() -> ModelResult<Self> {
		let model = DocLlm::new()?;
		Ok(Self { model: Some(model) })
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

