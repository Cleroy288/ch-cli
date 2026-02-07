//! Additional RustParser methods.
//!
//! Contains language accessor and doc extraction delegation methods.

use tree_sitter::Language;

use super::doc_extraction::{extract_item_doc, extract_module_docs};
use super::rust_parser::RustParser;

impl RustParser {
	/// Get the Tree-sitter language.
	pub fn language(&self) -> &Language {
		&self.language
	}

	/// Extract module-level doc comments (//!) from source code.
	/// Delegates to the doc_extraction module.
	pub fn extract_module_docs(source: &str) -> Option<String> {
		extract_module_docs(source)
	}

	/// Extract doc comment (///) above a symbol at a given line.
	/// Delegates to the doc_extraction module.
	pub fn extract_item_doc(source: &str, symbol_line: usize) -> Option<String> {
		extract_item_doc(source, symbol_line)
	}
}
