use tantivy::TantivyDocument;

use crate::indexer::search::schema::SchemaFields;
use crate::indexer::symbols::{DocumentType, Symbol};

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
	doc.add_text(
		fields.visibility,
		symbol.visibility.to_string(),
	);
}

fn add_doc_type(
	doc: &mut TantivyDocument,
	fields: &SchemaFields,
	symbol: &Symbol,
) {
	let doc_type =
		DocumentType::from_path(&symbol.location.file);
	let type_str = match doc_type {
		DocumentType::SourceCode => "SourceCode",
		DocumentType::Documentation => "Documentation",
		DocumentType::Notes => "Notes",
		DocumentType::Benchmark => "Benchmark",
		DocumentType::Test => "Test",
	};
	doc.add_text(fields.document_type, type_str);
}

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
