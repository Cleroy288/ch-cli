use std::path::Path;

use streaming_iterator::StreamingIterator;
use tree_sitter::{Language, Parser, Query, QueryCursor};

use crate::indexer::queries::{
	RUST_REFERENCES_QUERY, RUST_SYMBOLS_QUERY,
};
use crate::indexer::Symbol;

use super::reference_processing::{
	deduplicate_references, process_reference_match,
};
use super::symbol_processing::process_match;
use super::types::{ExtractedReference, ParseError, Result};

/// Rust language parser using Tree-sitter.
/// Extracts symbols and references from Rust source.
pub struct RustParser {
	parser: Parser,
	pub(crate) language: Language,
	symbol_query: Query,
	reference_query: Query,
}

impl RustParser {
	pub fn new() -> Result<Self> {
		let mut parser = Parser::new();
		let language: Language =
			tree_sitter_rust::LANGUAGE.into();

		parser.set_language(&language).map_err(|e| {
			ParseError::Language(e.to_string())
		})?;

		let sym_q = Query::new(&language, RUST_SYMBOLS_QUERY)
			.map_err(|e| {
				ParseError::Query(format!("{:?}", e))
			})?;
		let ref_q =
			Query::new(&language, RUST_REFERENCES_QUERY)
				.map_err(|e| {
					ParseError::Query(format!("{:?}", e))
				})?;

		Ok(Self {
			parser,
			language,
			symbol_query: sym_q,
			reference_query: ref_q,
		})
	}

	pub fn parse_source(
		&mut self,
		source: &str,
		file_path: &Path,
	) -> Result<Vec<Symbol>> {
		let tree = self.parse_tree(source, file_path)?;
		let root = tree.root_node();
		let mut cursor = QueryCursor::new();
		let mut symbols: Vec<Symbol> = Vec::new();

		let mut matches = cursor.matches(
			&self.symbol_query,
			root, source.as_bytes(),
		);
		while let Some(m) = matches.next() {
			if let Some(sym) = process_match(
				m, source, file_path,
				&self.symbol_query,
			) {
				symbols.push(sym);
			}
		}

		Ok(symbols)
	}

	pub fn parse_references(
		&mut self,
		source: &str,
		file_path: &Path,
	) -> Result<Vec<ExtractedReference>> {
		let tree = self.parse_tree(source, file_path)?;
		let root = tree.root_node();
		let mut cursor = QueryCursor::new();
		let mut refs: Vec<ExtractedReference> = Vec::new();

		let mut matches = cursor.matches(
			&self.reference_query,
			root, source.as_bytes(),
		);
		while let Some(m) = matches.next() {
			if let Some(r) = process_reference_match(
				m, source, file_path,
				&self.reference_query,
			) {
				refs.push(r);
			}
		}

		deduplicate_references(&mut refs);
		Ok(refs)
	}

	fn parse_tree(
		&mut self,
		source: &str,
		file_path: &Path,
	) -> Result<tree_sitter::Tree> {
		self.parser.parse(source, None).ok_or_else(|| {
			ParseError::ParseFailed(
				file_path.display().to_string(),
			)
		})
	}
}
