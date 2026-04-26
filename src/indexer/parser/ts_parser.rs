use std::path::Path;

use streaming_iterator::StreamingIterator;
use tree_sitter::{Language, Parser, Query, QueryCursor};

use crate::indexer::queries::TS_SYMBOLS_QUERY;
use crate::indexer::symbols::Symbol;

use super::symbol_processing::process_match;
use super::types::{ParseError, Result};

/// TypeScript / TSX parser using Tree-sitter.
pub struct TsParser {
	parser: Parser,
	symbol_query: Query,
}

impl TsParser {
	/// Create a TsParser for .ts files.
	pub fn typescript() -> Result<Self> {
		let lang: Language =
			tree_sitter_typescript::LANGUAGE_TYPESCRIPT
				.into();
		Self::with_language(lang)
	}

	/// Create a TsParser for .tsx files.
	pub fn tsx() -> Result<Self> {
		let lang: Language =
			tree_sitter_typescript::LANGUAGE_TSX.into();
		Self::with_language(lang)
	}

	fn with_language(language: Language) -> Result<Self> {
		let mut parser = Parser::new();
		parser
			.set_language(&language)
			.map_err(|e| {
				ParseError::Language(e.to_string())
			})?;

		let symbol_query =
			Query::new(&language, TS_SYMBOLS_QUERY)
				.map_err(|e| {
					ParseError::Query(
						format!("{:?}", e),
					)
				})?;

		Ok(Self { parser, symbol_query })
	}
}

impl TsParser {
	pub fn parse_file<P: AsRef<Path>>(
		&mut self,
		path: P,
	) -> Result<Vec<Symbol>> {
		let path = path.as_ref();
		let source = std::fs::read_to_string(path)?;
		self.parse_source(&source, path)
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
