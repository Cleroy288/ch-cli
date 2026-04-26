use std::path::PathBuf;

use tantivy::schema::{Field, Value};
use tantivy::TantivyDocument;

use crate::indexer::search::schema::SchemaFields;
use crate::indexer::symbols::{ByteSpan, CodeLocation, Symbol};

use super::conversion_parsing::{
	parse_symbol_kind, parse_visibility,
};

pub fn doc_to_symbol(
	fields: &SchemaFields,
	doc: &TantivyDocument,
) -> Option<Symbol> {
	let symbol = extract_base_symbol(fields, doc)?;
	Some(attach_optional(symbol, fields, doc))
}

/// Extract optional non-empty string from a doc field
fn extract_opt_str(
	doc: &TantivyDocument,
	field: Field,
) -> Option<String> {
	let val = doc.get_first(field)?;
	let text = val.as_str()?;
	if text.is_empty() {
		None
	} else {
		Some(text.to_string())
	}
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
	let kind_str =
		doc.get_first(fields.symbol_kind)?.as_str()?;
	let path =
		doc.get_first(fields.file_path)?.as_str()?;
	let line =
		doc.get_first(fields.line)?.as_u64()? as usize;
	let col =
		doc.get_first(fields.column)?.as_u64()? as usize;
	let vis_str =
		doc.get_first(fields.visibility)?.as_str()?;

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
	if let Some(sig) =
		extract_opt_str(doc, fields.signature)
	{
		symbol = symbol.with_signature(sig);
	}
	if let Some(par) =
		extract_opt_str(doc, fields.parent)
	{
		symbol = symbol.with_parent(par);
	}
	if let Some(cnt) =
		extract_opt_str(doc, fields.content)
	{
		symbol = symbol.with_content(cnt);
	}
	symbol
}
