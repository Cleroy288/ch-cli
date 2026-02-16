//! Rust file parsing with doc comment extraction.

use std::path::Path;

use crate::indexer::crawler::FileResult;
use crate::indexer::parser::RustParser;
use crate::indexer::symbols::{
	ByteSpan, CodeLocation, SymbolKind,
};
use crate::indexer::Symbol;

use super::builder::IndexManager;

impl IndexManager {
	/// Parse a Rust file with doc comment extraction
	pub(super) fn parse_rust_file(
		&self,
		path: &Path,
	) -> FileResult {
		let mut parser = match RustParser::new() {
			Ok(par) => par,
			Err(err) => return error_result(path, err),
		};

		let source = match std::fs::read_to_string(path) {
			Ok(src) => src,
			Err(err) => {
				let parse_err =
					crate::indexer::parser::ParseError::IoError(err);
				return error_result(path, parse_err);
			}
		};

		let mut symbols = match parser.parse_source(&source, path) {
			Ok(syms) => syms,
			Err(err) => return error_result(path, err),
		};

		enrich_with_docs(&source, &mut symbols);
		append_module_doc(path, &source, &mut symbols);

		FileResult {
			path: path.to_path_buf(),
			symbols,
			error: None,
		}
	}
}

/// Enrich symbols with their associated doc comments
fn enrich_with_docs(
	source: &str,
	symbols: &mut [Symbol],
) {
	for symbol in symbols.iter_mut() {
		let doc = RustParser::extract_item_doc(
			source, symbol.location.line,
		);
		if let Some(doc) = doc {
			symbol.doc_comment = Some(doc);
		}
	}
}

/// Append module-level doc symbol if module docs exist
fn append_module_doc(
	path: &Path,
	source: &str,
	symbols: &mut Vec<Symbol>,
) {
	let Some(module_doc) =
		RustParser::extract_module_docs(source)
	else {
		return;
	};

	let mod_name = path
		.file_stem()
		.and_then(|stem| stem.to_str())
		.unwrap_or("module");

	let location = CodeLocation::new(
		path.to_path_buf(), 1, 1,
		ByteSpan::ZERO,
	);
	let symbol = Symbol::new(
		format!("{}_docs", mod_name),
		SymbolKind::DocumentChunk,
		location,
	)
	.with_content(module_doc.clone())
	.with_doc_comment(module_doc);

	symbols.push(symbol);
}

/// Create a FileResult for a parse error
fn error_result(
	path: &Path,
	err: crate::indexer::parser::ParseError,
) -> FileResult {
	FileResult {
		path: path.to_path_buf(),
		symbols: Vec::new(),
		error: Some(err),
	}
}
