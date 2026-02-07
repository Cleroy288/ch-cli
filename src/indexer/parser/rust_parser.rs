//! Rust language parser using Tree-sitter.
//!
//! Provides the main RustParser struct for parsing Rust source files.

use std::fs;
use std::path::Path;

use streaming_iterator::StreamingIterator;
use tree_sitter::{Language, Parser, Query, QueryCursor};

use crate::indexer::queries::{RUST_REFERENCES_QUERY, RUST_SYMBOLS_QUERY};
use crate::indexer::symbols::Symbol;

use super::reference_processing::{
	deduplicate_references, process_reference_match,
};
use super::symbol_processing::process_match;
use super::types::{ExtractedReference, ParseError, Result};

/// Rust language parser using Tree-sitter.
/// Provides methods to parse Rust source files
/// and extract symbols and references.
pub struct RustParser {
	parser: Parser, // tree-sitter parser instance
	pub(crate) language: Language, // rust language grammar
	symbol_query: Query, // compiled query for symbol extraction
	reference_query: Query, // compiled query for reference extraction
}

impl RustParser {
	/// Create a new RustParser instance.
	/// Initializes the tree-sitter parser with Rust grammar and compiles queries.
	pub fn new() -> Result<Self> {
		let mut parser = Parser::new();
		let language: Language = tree_sitter_rust::LANGUAGE.into();

		parser
			.set_language(&language)
			.map_err(|e| ParseError::LanguageError(e.to_string()))?;

		let symbol_query = Query::new(&language, RUST_SYMBOLS_QUERY)
			.map_err(|e| ParseError::QueryError(format!("{:?}", e)))?;

		let reference_query = Query::new(&language, RUST_REFERENCES_QUERY)
			.map_err(|e| ParseError::QueryError(format!("{:?}", e)))?;

		Ok(Self {
			parser,
			language,
			symbol_query,
			reference_query,
		})
	}

	/// Parse a Rust source file and extract all symbols.
	/// Reads the file and delegates to parse_source.
	pub fn parse_file<P: AsRef<Path>>(&mut self, path: P) -> Result<Vec<Symbol>> {
		let path = path.as_ref();
		let source_code = fs::read_to_string(path)?;
		self.parse_source(&source_code, path)
	}

	/// Parse Rust source code from a string.
	/// Returns a vector of all symbols found in the source.
	pub fn parse_source(
		&mut self,
		source: &str,
		file_path: &Path,
	) -> Result<Vec<Symbol>> {
		let tree = self
			.parser
			.parse(source, None)
			.ok_or_else(|| ParseError::ParseFailed(file_path.display().to_string()))?;

		let root_node = tree.root_node();
		let mut symbols: Vec<Symbol> = Vec::new(); // collected symbols
		let mut cursor = QueryCursor::new();

		let mut matches = cursor.matches(
			&self.symbol_query,
			root_node,
			source.as_bytes(),
		);

		// Use StreamingIterator API
		while let Some(match_) = matches.next() {
			let sym = process_match(
				match_, source, file_path,
				&self.symbol_query,
			);
			if let Some(symbol) = sym {
				symbols.push(symbol);
			}
		}

		Ok(symbols)
	}

	/// Parse a Rust source file and extract all references (symbol usages).
	/// Reads the file and delegates to parse_references.
	pub fn parse_file_references<P: AsRef<Path>>(
		&mut self,
		path: P,
	) -> Result<Vec<ExtractedReference>> {
		let path = path.as_ref();
		let source_code = fs::read_to_string(path)?;
		self.parse_references(&source_code, path)
	}

	/// Parse Rust source code and extract all references.
	/// Returns a deduplicated vector of all symbol references.
	pub fn parse_references(
		&mut self,
		source: &str,
		file_path: &Path,
	) -> Result<Vec<ExtractedReference>> {
		let tree = self
			.parser
			.parse(source, None)
			.ok_or_else(|| ParseError::ParseFailed(file_path.display().to_string()))?;

		let root_node = tree.root_node();
		let mut references: Vec<ExtractedReference> = Vec::new();
		let mut cursor = QueryCursor::new();

		let mut matches = cursor.matches(
			&self.reference_query,
			root_node,
			source.as_bytes(),
		);

		while let Some(match_) = matches.next() {
			let extracted = process_reference_match(
				match_, source, file_path,
				&self.reference_query,
			);
			if let Some(reference) = extracted {
				references.push(reference);
			}
		}

		// Deduplicate references (same name and location)
		deduplicate_references(&mut references);

		Ok(references)
	}
}

