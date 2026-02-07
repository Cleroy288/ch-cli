//! Structured Output Types
//!
//! Provides JSON-serializable output types for LLM consumption.
//! Separates code, doc, and notes context for clear presentation.

use serde::{Deserialize, Serialize};

use super::output_structured::{
	CodeResult, DocResult, NotesResult,
};

/// Structured output for LLM consumption
/// Contains separate sections for code, docs, and notes
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StructuredOutput {
	/// original query
	pub query: String,
	/// detected intent
	pub intent: String,
	/// code search results with full file content
	pub code_context: Vec<CodeResult>,
	/// documentation search results
	pub doc_context: Vec<DocResult>,
	/// notes search results
	pub notes_context: Vec<NotesResult>,
}

impl StructuredOutput {
	/// Create new structured output
	pub fn new(query: String, intent: String) -> Self {
		Self {
			query,
			intent,
			code_context: Vec::new(),
			doc_context: Vec::new(),
			notes_context: Vec::new(),
		}
	}

	/// Add code result
	pub fn add_code(&mut self, result: CodeResult) {
		self.code_context.push(result);
	}

	/// Add doc result
	pub fn add_doc(&mut self, result: DocResult) {
		self.doc_context.push(result);
	}

	/// Add notes result
	pub fn add_notes(&mut self, result: NotesResult) {
		self.notes_context.push(result);
	}

	/// Get total results count
	pub fn total_results(&self) -> usize {
		self.code_context.len()
			+ self.doc_context.len()
			+ self.notes_context.len()
	}
}

