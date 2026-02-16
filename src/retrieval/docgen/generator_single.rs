//! Single-entry documentation generation.

use crate::retrieval::docgen::entry::DocEntry;
use crate::retrieval::docgen::generator::{
	DocGenerator, MAX_DOC_TOKENS,
};
use crate::retrieval::docgen::generator_utils::{
	clean_generated_doc, extract_code_snippet,
};
use crate::retrieval::docgen::prompts::{
	build_prompt, PromptInput,
};
use crate::retrieval::models::{ModelError, ModelResult};

/// Single-entry generation methods.
impl DocGenerator {
	/// Generate documentation for a single entry.
	pub fn generate(
		&mut self,
		entry: &mut DocEntry,
	) -> ModelResult<()> {
		ensure_snippet(entry)?;
		let prompt = build_entry_prompt(entry);

		let model = self.model.as_mut().ok_or_else(|| {
			ModelError::NotLoaded(
				"DocGenerator model not loaded".into(),
			)
		})?;

		entry.mark_generating();

		match model.generate(&prompt, MAX_DOC_TOKENS) {
			Ok(doc) => {
				let cleaned = clean_generated_doc(&doc);
				entry.mark_ready(cleaned);
				Ok(())
			}
			Err(err) => {
				entry.mark_failed();
				Err(err)
			}
		}
	}

	/// Generate documentation from a pre-built prompt.
	pub fn generate_from_prompt(
		&mut self,
		entry: &mut DocEntry,
		prompt: &str,
	) -> ModelResult<()> {
		let model = self.model.as_mut().ok_or_else(|| {
			ModelError::NotLoaded(
				"DocGenerator model not loaded".into(),
			)
		})?;

		entry.mark_generating();

		match model.generate(prompt, MAX_DOC_TOKENS) {
			Ok(doc) => {
				let cleaned = clean_generated_doc(&doc);
				entry.mark_ready(cleaned);
				Ok(())
			}
			Err(err) => {
				entry.mark_failed();
				Err(err)
			}
		}
	}
}

/// Fill in code snippet if empty.
fn ensure_snippet(
	entry: &mut DocEntry,
) -> ModelResult<()> {
	if entry.code_snippet.is_empty() {
		entry.code_snippet = extract_code_snippet(
			&entry.file_path,
			entry.line,
		)?;
	}
	Ok(())
}

/// Build the LLM prompt for an entry.
fn build_entry_prompt(entry: &DocEntry) -> String {
	build_prompt(PromptInput {
		kind: entry.kind,
		name: &entry.name,
		signature: entry.signature.as_deref(),
		code_snippet: &entry.code_snippet,
		user_comment: entry.user_comment.as_deref(),
		parent: entry.links.parent.as_deref(),
	})
}
