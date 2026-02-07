//! Conversion functions between Symbol and Tantivy documents.

use std::path::PathBuf;

use tantivy::schema::Value;
use tantivy::TantivyDocument;

use crate::indexer::search::schema::SchemaFields;
use crate::indexer::symbols::{CodeLocation, DocumentType, Symbol};

use super::conversion_parsing::{parse_symbol_kind, parse_visibility};

/// Convert a Symbol to a Tantivy document
pub fn symbol_to_doc(
	fields: &SchemaFields,
	symbol: &Symbol,
) -> TantivyDocument {
	let mut doc = TantivyDocument::new(); // tantivy document to populate

	doc.add_text(fields.symbol_name, &symbol.name);
	doc.add_text(fields.symbol_kind, &symbol.kind.to_string());
	let file_path = symbol.location.file.to_string_lossy();
	doc.add_text(fields.file_path, file_path);
	doc.add_u64(fields.line, symbol.location.line as u64);
	doc.add_u64(fields.column, symbol.location.column as u64);
	doc.add_text(fields.visibility, &symbol.visibility.to_string());

	// Add document type based on file path
	let doc_type = DocumentType::from_path(&symbol.location.file);
	let doc_type_str = match doc_type {
		DocumentType::SourceCode => "SourceCode",
		DocumentType::Documentation => "Documentation",
		DocumentType::Notes => "Notes",
		DocumentType::Benchmark => "Benchmark",
		DocumentType::Test => "Test",
	};
	doc.add_text(fields.document_type, doc_type_str);

	// Optional fields
	if let Some(ref sig) = symbol.signature {
		doc.add_text(fields.signature, sig);
	}
	if let Some(ref fqn) = symbol.fqn {
		doc.add_text(fields.fqn, fqn);
	}
	if let Some(ref parent) = symbol.parent {
		doc.add_text(fields.parent, parent);
	}
	if let Some(ref content) = symbol.content {
		doc.add_text(fields.content, content);
	}

	doc
}

/// Convert a Tantivy document back to a Symbol
pub fn doc_to_symbol(
	fields: &SchemaFields,
	doc: &TantivyDocument,
) -> Option<Symbol> {
	let name = doc // symbol name
		.get_first(fields.symbol_name)?
		.as_str()?
		.to_string();
	let kind_str = doc.get_first(fields.symbol_kind)?.as_str()?;
	let file_path = doc.get_first(fields.file_path)?.as_str()?;
	let line = doc.get_first(fields.line)?.as_u64()? as usize;
	let column = doc.get_first(fields.column)?.as_u64()? as usize;
	let visibility_str = doc.get_first(fields.visibility)?.as_str()?;

	let kind = parse_symbol_kind(kind_str)?;
	let visibility = parse_visibility(visibility_str);
	let file_buf = PathBuf::from(file_path);
	let location = CodeLocation::new(file_buf, line, column, 0, 0);

	let mut symbol = Symbol::new(name, kind, location).with_visibility(visibility);

	// Optional fields
	if let Some(val) = doc.get_first(fields.signature) {
		if let Some(s) = val.as_str() {
			if !s.is_empty() {
				symbol = symbol.with_signature(s.to_string());
			}
		}
	}
	if let Some(val) = doc.get_first(fields.parent) {
		if let Some(s) = val.as_str() {
			if !s.is_empty() {
				symbol = symbol.with_parent(s.to_string());
			}
		}
	}
	if let Some(val) = doc.get_first(fields.content) {
		if let Some(s) = val.as_str() {
			if !s.is_empty() {
				symbol = symbol.with_content(s.to_string());
			}
		}
	}

	Some(symbol)
}
