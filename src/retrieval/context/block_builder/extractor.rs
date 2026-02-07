//! Code Extraction for BlockBuilder
//!
//! Handles extracting code snippets from source files.

use std::fs;

use crate::indexer::Symbol;
use crate::retrieval::RetrievalResult;

use super::core::BlockBuilder;
use super::super::code_extractor::find_symbol_end;

impl<'a> BlockBuilder<'a> {
	/// Extract code snippet for a symbol
	pub(super) fn extract_code(
		&self,
		symbol: &Symbol,
	) -> RetrievalResult<String> {
		let path = &symbol.location.file;
		let start_line = symbol.location.line;

		// read the file
		let content = fs::read_to_string(path)?;
		let lines: Vec<&str> = content.lines().collect();

		// calculate line range
		let context_before = self.config.context_lines_before;
		let context_after = self.config.context_lines_after;

		let start =
			start_line.saturating_sub(context_before + 1);
		let sym_end =
			find_symbol_end(&lines, start_line - 1, symbol);
		let end = sym_end + context_after;
		let end = end.min(lines.len());

		// extract lines with line numbers
		let mut snippet = String::new();
		for (i, line) in lines[start..end].iter().enumerate()
		{
			let line_num = start + i + 1;
			snippet.push_str(&format!(
				"{:4} | {}\n",
				line_num, line
			));
		}

		Ok(snippet)
	}
}
