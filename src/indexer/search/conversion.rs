//! Conversion functions between Symbol and Tantivy documents.

use std::path::PathBuf;

use tantivy::schema::Value;
use tantivy::TantivyDocument;

use crate::indexer::search::schema::SchemaFields;
use crate::indexer::symbols::{
	ByteSpan, CodeLocation, DocumentType, Symbol,
};

use tantivy::schema::Field;

use super::conversion_parsing::{parse_symbol_kind, parse_visibility};

/// Convert a Symbol to a Tantivy document
pub fn symbol_to_doc(
	fields: &SchemaFields,
	symbol: &Symbol,
) -> TantivyDocument {
	let mut doc = TantivyDocument::new();
	add_required_fields(&mut doc, fields, symbol);
	add_doc_type(&mut doc, fields, symbol);
	add_optional_fields(&mut doc, fields, symbol);
	doc
}

/// Add required fields to the Tantivy document
fn add_required_fields(
	doc: &mut TantivyDocument,
	fields: &SchemaFields,
	symbol: &Symbol,
) {
	doc.add_text(fields.symbol_name, &symbol.name);
	doc.add_text(fields.symbol_kind, symbol.kind.to_string());
	let path = symbol.location.file.to_string_lossy();
	doc.add_text(fields.file_path, path);
	doc.add_u64(fields.line, symbol.location.line as u64);
	doc.add_u64(fields.column, symbol.location.column as u64);
	doc.add_text(fields.visibility, symbol.visibility.to_string());
}

/// Add document type field based on file path
fn add_doc_type(
	doc: &mut TantivyDocument,
	fields: &SchemaFields,
	symbol: &Symbol,
) {
	let doc_type = DocumentType::from_path(&symbol.location.file);
	let type_str = match doc_type {
		DocumentType::SourceCode => "SourceCode",
		DocumentType::Documentation => "Documentation",
		DocumentType::Notes => "Notes",
		DocumentType::Benchmark => "Benchmark",
		DocumentType::Test => "Test",
	};
	doc.add_text(fields.document_type, type_str);
}

/// Add optional fields (signature, fqn, parent, content)
fn add_optional_fields(
	doc: &mut TantivyDocument,
	fields: &SchemaFields,
	symbol: &Symbol,
) {
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
}

/// Extract optional non-empty string from a Tantivy doc field
fn extract_opt_str(
	doc: &TantivyDocument,
	field: Field,
) -> Option<String> {
	let val = doc.get_first(field)?;
	let text = val.as_str()?;
	if text.is_empty() { None } else { Some(text.to_string()) }
}

/// Convert a Tantivy document back to a Symbol
pub fn doc_to_symbol(
	fields: &SchemaFields,
	doc: &TantivyDocument,
) -> Option<Symbol> {
	let symbol = extract_base_symbol(fields, doc)?;
	Some(attach_optional(symbol, fields, doc))
}

/// Extract the base symbol from required doc fields
#[allow(clippy::cognitive_complexity)]
fn extract_base_symbol(
	fields: &SchemaFields,
	doc: &TantivyDocument,
) -> Option<Symbol> {
	let name = doc
		.get_first(fields.symbol_name)?
		.as_str()?
		.to_string();
	let kind_str = doc
		.get_first(fields.symbol_kind)?.as_str()?;
	let path = doc
		.get_first(fields.file_path)?.as_str()?;
	let line = doc
		.get_first(fields.line)?.as_u64()? as usize;
	let col = doc
		.get_first(fields.column)?.as_u64()? as usize;
	let vis_str = doc
		.get_first(fields.visibility)?.as_str()?;

	let kind = parse_symbol_kind(kind_str)?;
	let vis = parse_visibility(vis_str);
	let location = CodeLocation::new(
		PathBuf::from(path), line, col, ByteSpan::ZERO,
	);

	Some(
		Symbol::new(name, kind, location)
			.with_visibility(vis),
	)
}

/// Attach optional fields to the symbol
fn attach_optional(
	mut symbol: Symbol,
	fields: &SchemaFields,
	doc: &TantivyDocument,
) -> Symbol {
	if let Some(sig) = extract_opt_str(doc, fields.signature) {
		symbol = symbol.with_signature(sig);
	}
	if let Some(par) = extract_opt_str(doc, fields.parent) {
		symbol = symbol.with_parent(par);
	}
	if let Some(cnt) = extract_opt_str(doc, fields.content) {
		symbol = symbol.with_content(cnt);
	}
	symbol
}
