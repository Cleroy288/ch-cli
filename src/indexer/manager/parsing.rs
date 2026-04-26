//! File parsing logic for the index manager.

use std::path::Path;

use crate::domain::errors::parse::ParseResult;
use crate::indexer::crawler::{FileResult, Language};
use crate::indexer::doc_parser::DocParser;
use crate::indexer::parser::{
	ExtractedReference, RustParser,
};

use super::builder::IndexManager;

impl IndexManager {
	/// Parse references from a file
	pub(super) fn parse_references(
		&self,
		path: &Path,
	) -> ParseResult<Vec<ExtractedReference>> {
		match Language::from_path(path) {
			Some(Language::Rust) => {
				let mut parser = RustParser::new()?;
				parser.parse_file_references(path)
			}
			_ => Ok(Vec::new()),
		}
	}

	/// Parse a single file based on its language
	pub(super) fn parse_file(
		&self,
		path: &Path,
	) -> FileResult {
		match Language::from_path(path) {
			Some(Language::Rust) => {
				self.parse_rust_file(path)
			}
			Some(Language::Markdown) => {
				self.parse_markdown_file(path)
			}
			None => empty_result(path),
		}
	}

	/// Parse markdown file into document chunks
	pub(super) fn parse_markdown_file(
		&self,
		path: &Path,
	) -> FileResult {
		match std::fs::read_to_string(path) {
			Ok(content) => FileResult {
				path: path.to_path_buf(),
				symbols: DocParser::parse(path, &content),
				error: None,
			},
			Err(_) => empty_result(path),
		}
	}
}

/// Create an empty FileResult with no symbols or errors
fn empty_result(path: &Path) -> FileResult {
	FileResult {
		path: path.to_path_buf(),
		symbols: Vec::new(),
		error: None,
	}
}
