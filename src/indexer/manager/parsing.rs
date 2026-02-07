//! File parsing logic for the index manager.

use std::path::Path;

use crate::indexer::crawler::{FileResult, Language};
use crate::indexer::doc_parser::DocParser;
use crate::indexer::parser::{ExtractedReference, RustParser};
use crate::indexer::Symbol;

use super::builder::IndexManager;

impl IndexManager {
	/// Parse references from a file
	pub(super) fn parse_references(&self, path: &Path) -> Result<Vec<ExtractedReference>, String> {
		let language = Language::from_path(path);

		match language {
			Some(Language::Rust) => {
				let mut parser = RustParser::new().map_err(|e| e.to_string())?;
				parser
					.parse_file_references(path)
					.map_err(|e| e.to_string())
			}
			Some(Language::Markdown) => Ok(Vec::new()), // no references in markdown
			None => Ok(Vec::new()),
		}
	}

	/// Parse a single file
	pub(super) fn parse_file(&self, path: &Path) -> FileResult {
		// Determine language and create appropriate parser
		let language = Language::from_path(path);

		match language {
			Some(Language::Rust) => self.parse_rust_file(path),
			Some(Language::Markdown) => {
				// Parse markdown into document chunks
				match self.parse_markdown(path) {
					Ok(symbols) => FileResult {
						path: path.to_path_buf(),
						symbols,
						error: None,
					},
					Err(_) => FileResult {
						path: path.to_path_buf(),
						symbols: Vec::new(),
						error: None,
					},
				}
			}
			None => FileResult {
				path: path.to_path_buf(),
				symbols: Vec::new(),
				error: None,
			},
		}
	}

	/// Parse markdown file into document chunks by headers
	/// Uses DocParser for hierarchical section extraction
	pub(super) fn parse_markdown(&self, path: &Path) -> std::io::Result<Vec<Symbol>> {
		let content = std::fs::read_to_string(path)?;
		Ok(DocParser::parse(path, &content))
	}

	/// Parse Rust file with doc comment extraction
	/// Enriches symbols with their associated doc comments
	pub(super) fn parse_rust_file(&self, path: &Path) -> FileResult {
		// create parser
		let mut parser = match RustParser::new() {
			Ok(p) => p,
			Err(e) => {
				return FileResult {
					path: path.to_path_buf(),
					symbols: Vec::new(),
					error: Some(e),
				};
			}
		};

		// read source for doc extraction
		let source = match std::fs::read_to_string(path) {
			Ok(s) => s,
			Err(e) => {
				return FileResult {
					path: path.to_path_buf(),
					symbols: Vec::new(),
					error: Some(
						crate::indexer::parser::ParseError::IoError(e)
					),
				};
			}
		};

		// parse symbols
		let mut symbols = match parser.parse_source(&source, path) {
			Ok(s) => s,
			Err(e) => {
				return FileResult {
					path: path.to_path_buf(),
					symbols: Vec::new(),
					error: Some(e),
				};
			}
		};

		// extract doc comments for each symbol
		for symbol in &mut symbols {
			let doc = RustParser::extract_item_doc(
				&source,
				symbol.location.line
			);
			if let Some(doc) = doc {
				symbol.doc_comment = Some(doc);
			}
		}

		// extract module docs and create module symbol
		if let Some(module_doc) = RustParser::extract_module_docs(&source) {
			let module_name = path
				.file_stem()
				.and_then(|s| s.to_str())
				.unwrap_or("module");

			let module_symbol = Symbol::new(
				format!("{}_docs", module_name),
				crate::indexer::SymbolKind::DocumentChunk,
				crate::indexer::CodeLocation::new(
					path.to_path_buf(), 1, 1, 0, 0
				),
			)
			.with_content(module_doc.clone())
			.with_doc_comment(module_doc);

			symbols.push(module_symbol);
		}

		FileResult {
			path: path.to_path_buf(),
			symbols,
			error: None,
		}
	}
}
