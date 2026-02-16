//! Build Symbol objects from extracted capture data.

use std::path::Path;

use crate::indexer::symbols::{
	ByteSpan, CodeLocation, Symbol, SymbolKind,
};

use super::symbol_processing::CaptureData;

/// Max characters to extract from function body
const MAX_BODY_CONTENT_LEN: usize = 500;

/// Build a Symbol from capture data
pub fn build_symbol<'src>(
	data: CaptureData<'src>,
	source: &str,
	file_path: &Path,
) -> Option<Symbol> {
	let name = data.name?;
	let kind = data.kind?;
	let node = data.def_node?;

	let location = node_to_location(file_path, &node);
	let symbol = Symbol::new(
		name.to_string(), kind, location,
	)
	.with_visibility(data.visibility);

	let symbol = apply_parent(symbol, data.parent_type);
	let symbol = apply_body(symbol, kind, source, &node);
	Some(symbol)
}

/// Set parent type on symbol if present
fn apply_parent(
	symbol: Symbol,
	parent: Option<&str>,
) -> Symbol {
	match parent {
		Some(name) => symbol.with_parent(name.to_string()),
		None => symbol,
	}
}

/// Extract and attach body content if applicable
fn apply_body(
	symbol: Symbol,
	kind: SymbolKind,
	source: &str,
	node: &tree_sitter::Node,
) -> Symbol {
	if !should_extract_body(kind) {
		return symbol;
	}
	match extract_body(source, node) {
		Some(body) => symbol.with_content(body),
		None => symbol,
	}
}

/// Convert a tree-sitter node to a CodeLocation
fn node_to_location(
	file_path: &Path,
	node: &tree_sitter::Node,
) -> CodeLocation {
	let bytes = ByteSpan {
		offset: node.start_byte(),
		length: node.end_byte() - node.start_byte(),
	};
	CodeLocation::new(
		file_path.to_path_buf(),
		node.start_position().row + 1,
		node.start_position().column + 1,
		bytes,
	)
}

/// Check if symbol kind warrants body extraction
fn should_extract_body(kind: SymbolKind) -> bool {
	matches!(
		kind,
		SymbolKind::Function
			| SymbolKind::Method
			| SymbolKind::Struct
			| SymbolKind::Enum
			| SymbolKind::Trait
			| SymbolKind::Impl
			| SymbolKind::Macro
	)
}

/// Extract body content, truncated at word boundary
fn extract_body(
	source: &str,
	node: &tree_sitter::Node,
) -> Option<String> {
	let start = node.start_byte();
	let end = node.end_byte().min(start + MAX_BODY_CONTENT_LEN);
	let content = source.get(start..end)?;

	if end < node.end_byte()
		&& content.len() >= MAX_BODY_CONTENT_LEN
	{
		if let Some(last_space) =
			content.rfind(char::is_whitespace)
		{
			return Some(
				content[..last_space].to_string() + "...",
			);
		}
	}
	Some(content.to_string())
}
