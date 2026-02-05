//! Documentation Generator using local LLM.
//!
//! Uses the Phi-3 model to generate documentation for code symbols.

use std::fs;
use std::path::Path;

use crate::retrieval::docgen::entry::DocEntry;
use crate::retrieval::docgen::prompts::build_prompt;
use crate::retrieval::models::{ModelError, ModelResult};
use crate::retrieval::query::Phi3Model;

/// Maximum tokens to generate for documentation.
const MAX_DOC_TOKENS: usize = 200;

/// Number of context lines to extract around a symbol.
const CONTEXT_LINES: usize = 30;

/// Documentation generator using local LLM.
pub struct DocGenerator {
	/// the LLM model
	model: Option<Phi3Model>,
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
	pub fn with_model_id(model_id: &str) -> ModelResult<Self> {
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

	/// Generate documentation for a single entry.
	pub fn generate(&mut self, entry: &mut DocEntry) -> ModelResult<()> {
		let model = self.model.as_mut().ok_or_else(|| {
			ModelError::NotLoaded("DocGenerator model not loaded".to_string())
		})?;

		// extract code snippet if not already set
		if entry.code_snippet.is_empty() {
			entry.code_snippet = extract_code_snippet(&entry.file_path, entry.line)?;
		}

		// build prompt
		let prompt = build_prompt(
			entry.kind,
			&entry.name,
			entry.signature.as_deref(),
			&entry.code_snippet,
			entry.user_comment.as_deref(),
			entry.links.parent.as_deref(),
		);

		// mark as generating
		entry.mark_generating();

		// generate documentation
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

	/// Generate documentation for multiple entries.
	/// Returns number of successfully generated docs.
	pub fn generate_batch(&mut self, entries: &mut [&mut DocEntry]) -> ModelResult<usize> {
		let mut success_count = 0;

		for entry in entries.iter_mut() {
			match self.generate(*entry) {
				Ok(()) => success_count += 1,
				Err(e) => {
					eprintln!(
						"[docgen] Failed to generate doc for {}: {}",
						entry.name, e
					);
				}
			}
		}

		Ok(success_count)
	}

	/// Generate documentation for entries by ID from a store.
	pub fn generate_for_ids(
		&mut self,
		store: &mut crate::retrieval::docgen::DocStore,
		ids: &[String],
	) -> ModelResult<usize> {
		let mut success_count = 0;

		for id in ids {
			if let Some(entry) = store.get_mut(id) {
				// extract code snippet if needed
				if entry.code_snippet.is_empty() {
					if let Ok(snippet) = extract_code_snippet(&entry.file_path, entry.line) {
						entry.code_snippet = snippet;
					}
				}

				match self.generate(entry) {
					Ok(()) => success_count += 1,
					Err(e) => {
						eprintln!(
							"[docgen] Failed to generate doc for {}: {}",
							entry.name, e
						);
					}
				}
			}
		}

		Ok(success_count)
	}
}

impl Default for DocGenerator {
	fn default() -> Self {
		Self::new()
	}
}

/// Extract code snippet from file around a given line.
fn extract_code_snippet(file_path: &Path, line: usize) -> ModelResult<String> {
	let content = fs::read_to_string(file_path).map_err(|e| {
		ModelError::WeightLoad(format!("Failed to read file {}: {}", file_path.display(), e))
	})?;

	let lines: Vec<&str> = content.lines().collect();

	if line == 0 || line > lines.len() {
		return Ok(String::new());
	}

	// 0-indexed line number
	let line_idx = line - 1;

	// calculate range (CONTEXT_LINES / 2 before and after)
	let half_context = CONTEXT_LINES / 2;
	let start = line_idx.saturating_sub(half_context);
	let end = (line_idx + half_context).min(lines.len());

	// find the end of the current construct (look for closing brace at same level)
	let mut brace_count = 0;
	let mut found_open = false;
	let mut actual_end = end;

	for (i, line_content) in lines.iter().enumerate().skip(line_idx).take(CONTEXT_LINES * 2) {
		for ch in line_content.chars() {
			if ch == '{' {
				brace_count += 1;
				found_open = true;
			} else if ch == '}' {
				brace_count -= 1;
				if found_open && brace_count == 0 {
					actual_end = (i + 1).min(lines.len());
					break;
				}
			}
		}
		if found_open && brace_count == 0 {
			break;
		}
	}

	let end = actual_end.min(start + CONTEXT_LINES);
	let snippet: String = lines[start..end].join("\n");

	Ok(snippet)
}

/// Clean up generated documentation text.
fn clean_generated_doc(doc: &str) -> String {
	let mut cleaned = doc.trim().to_string();

	// remove any leading "Description:" or similar prefixes
	let prefixes = ["Description:", "description:", "Doc:", "doc:"];
	for prefix in prefixes {
		if cleaned.starts_with(prefix) {
			cleaned = cleaned[prefix.len()..].trim().to_string();
		}
	}

	// remove markdown code blocks if the model wrapped the response
	if cleaned.starts_with("```") {
		if let Some(end) = cleaned.rfind("```") {
			if end > 3 {
				// find end of first line (skip ```lang)
				let start = cleaned.find('\n').map(|i| i + 1).unwrap_or(3);
				cleaned = cleaned[start..end].trim().to_string();
			}
		}
	}

	// truncate if too long (keep first ~500 chars)
	if cleaned.len() > 500 {
		if let Some(pos) = cleaned[..500].rfind(". ") {
			cleaned = cleaned[..=pos].to_string();
		} else {
			cleaned.truncate(500);
			cleaned.push_str("...");
		}
	}

	cleaned
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test_clean_generated_doc() {
		let doc = "Description: This is a test function.";
		let cleaned = clean_generated_doc(doc);
		assert_eq!(cleaned, "This is a test function.");

		let doc_with_prefix = "description: Another test.";
		let cleaned2 = clean_generated_doc(doc_with_prefix);
		assert_eq!(cleaned2, "Another test.");
	}

	#[test]
	fn test_clean_generated_doc_code_block() {
		let doc = "```\nThis is inside a code block.\n```";
		let cleaned = clean_generated_doc(doc);
		assert_eq!(cleaned, "This is inside a code block.");
	}

	#[test]
	fn test_generator_not_ready() {
		let generator = DocGenerator::new();
		assert!(!generator.is_ready());
	}
}
