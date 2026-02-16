//! Core QueryInterpreter Implementation
//!
//! Provides the main interpreter structure and constructors.
//! Supports lazy-loading the Phi-3 model on first use.

use super::super::llm::Phi3Model;
use crate::retrieval::models::ModelResult;

/// Log message when lazy-loading the expansion model
const LAZY_LOAD_MSG: &str =
	"[daemon] Loading Phi-3 query expansion \
	 model (first use)...";

/// Log message after successful lazy-load
const LAZY_LOAD_OK: &str =
	"[daemon] Query expansion model loaded (lazy)";

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

	/// Create interpreter with LLM loaded eagerly
	pub fn with_model_id(
		model_id: &str,
	) -> ModelResult<Self> {
		let model =
			Phi3Model::from_model_id(model_id)?;
		Ok(Self { model: Some(model) })
	}

	/// Check if LLM is available
	pub fn has_llm(&self) -> bool {
		self.model.is_some()
	}

	/// Lazy-load the LLM model on first use
	///
	/// Returns true if the model is available after
	/// this call. Logs and returns false on failure.
	#[allow(clippy::print_stderr)]
	pub fn ensure_loaded(
		&mut self,
		model_id: &str,
	) -> bool {
		if self.model.is_some() {
			return true;
		}
		eprintln!("{}", LAZY_LOAD_MSG);
		match Phi3Model::from_model_id(model_id) {
			Ok(model) => {
				self.model = Some(model);
				eprintln!("{}", LAZY_LOAD_OK);
				true
			}
			Err(err) => {
				eprintln!(
					"[daemon] Failed to load \
					 expansion: {}", err
				);
				false
			}
		}
	}
}

impl Default for QueryInterpreter {
	fn default() -> Self {
		Self::new()
	}
}
