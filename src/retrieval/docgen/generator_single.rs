//! Single-entry documentation generation.

use crate::retrieval::docgen::entry::DocEntry;
use crate::retrieval::docgen::generator::{
	DocGenerator, MAX_DOC_TOKENS,
};
use crate::retrieval::docgen::generator_utils::{
	clean_generated_doc, extract_code_snippet,
};
use crate::retrieval::docgen::prompts::build_prompt;
use crate::retrieval::models::{ModelError, ModelResult};

/// Single-entry generation methods.
impl DocGenerator {
	/// Generate documentation for a single entry.
	pub fn generate(
		&mut self,
		entry: &mut DocEntry,
	) -> ModelResult<()> {
		let model = self.model.as_mut().ok_or_else(|| {
			ModelError::NotLoaded(
				"DocGenerator model not loaded".to_string(),
			)
		})?;

		if entry.code_snippet.is_empty() {
			entry.code_snippet = extract_code_snippet(
				&entry.file_path,
				entry.line,
			)?;
		}

		let prompt = build_prompt(
			entry.kind,
			&entry.name,
			entry.signature.as_deref(),
			&entry.code_snippet,
			entry.user_comment.as_deref(),
			entry.links.parent.as_deref(),
		);

		entry.mark_generating();

		match model.generate(&prompt, MAX_DOC_TOKENS) {
			Ok(doc) => {
				let cleaned = clean_generated_doc(&doc);
				entry.mark_ready(cleaned);
				Ok(())
			}
			Err(e) => {
				entry.mark_failed();
				Err(e)
			}
		}
	}

	/// Generate documentation from a pre-built prompt.
	///
	/// Skips code extraction and prompt building.
	/// Goes straight to LLM inference.
	pub fn generate_from_prompt(
		&mut self,
		entry: &mut DocEntry,
		prompt: &str,
	) -> ModelResult<()> {
		let model = self.model.as_mut().ok_or_else(|| {
			ModelError::NotLoaded(
				"DocGenerator model not loaded".to_string(),
			)
		})?;

		entry.mark_generating();

		match model.generate(prompt, MAX_DOC_TOKENS) {
			Ok(doc) => {
				let cleaned = clean_generated_doc(&doc);
				entry.mark_ready(cleaned);
				Ok(())
			}
			Err(e) => {
				entry.mark_failed();
				Err(e)
			}
		}
	}
}
