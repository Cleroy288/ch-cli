//! Structured result builders.
//!
//! Builds CodeResult, DocResult, and NotesResult from symbols.

use crate::indexer::Symbol;

use super::super::output_structured::{
	CodeResult, DocResult, NotesResult,
};
use super::core::RetrievalPipeline;

impl RetrievalPipeline {
	/// Build CodeResult with full file content
	pub(super) fn build_code_result(
		&self,
		symbol: &Symbol,
	) -> Option<CodeResult> {
		let file_path = &symbol.location.file;
		let content = std::fs::read_to_string(file_path).ok()?;
		let lines: Vec<&str> = content.lines().collect();
		let line_count = lines.len();
		let max_lines = self.config.max_lines_per_file;
		let truncated = line_count > max_lines;
		let full_content = if truncated {
			lines[..max_lines].join("\n")
		} else {
			content
		};

		Some(CodeResult {
			file: file_path.display().to_string(),
			symbol: symbol.name.clone(),
			kind: symbol.kind.to_string(),
			line: symbol.location.line,
			signature: symbol.signature.clone(),
			full_content,
			line_count,
			truncated,
			relevance_score: 1.0,
		})
	}

	/// Build DocResult from symbol
	pub(super) fn build_doc_result(
		&self,
		symbol: &Symbol,
	) -> DocResult {
		let content = symbol.content.clone().unwrap_or_default();
		DocResult {
			file: symbol.location.file.display().to_string(),
			section: symbol.name.clone(),
			content,
			relevance_score: 1.0,
		}
	}

	/// Build NotesResult from symbol
	pub(super) fn build_notes_result(
		&self,
		symbol: &Symbol,
	) -> NotesResult {
		let content = symbol.content.clone().unwrap_or_default();
		NotesResult {
			file: symbol.location.file.display().to_string(),
			section: symbol.name.clone(),
			content,
			relevance_score: 1.0,
		}
	}
}
