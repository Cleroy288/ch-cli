//! Symbol processing functions.
//!
//! Contains the logic for processing tree-sitter
//! query matches into Symbol objects.

use std::path::Path;

use tree_sitter::Query;

use crate::indexer::symbols::{CodeLocation, Symbol, SymbolKind, Visibility};

use super::helpers::parse_visibility;

/// Max characters to extract from function body for indexing
const MAX_BODY_CONTENT_LEN: usize = 500;

/// Extract body content from a node, limited to max_len characters.
/// Truncates at word boundary if possible to avoid cutting mid-word.
fn extract_body_content(
	source: &str,
	node: &tree_sitter::Node,
	max_len: usize,
) -> Option<String> {
	let start = node.start_byte(); // start byte of the node
	let end = node.end_byte().min(start + max_len); // end byte, capped at max_len
	let content = source.get(start..end)?; // slice of source code

	// Truncate at word boundary if content was cut short
	if end < node.end_byte() && content.len() >= max_len {
		if let Some(last_space) = content.rfind(char::is_whitespace) {
			return Some(content[..last_space].to_string() + "...");
		}
	}
	Some(content.to_string())
}

/// Process a query match and extract a symbol.
/// Maps tree-sitter capture names to symbol kinds and extracts metadata.
pub fn process_match(
	match_: &tree_sitter::QueryMatch,
	source: &str,
	file_path: &Path,
	symbol_query: &Query,
) -> Option<Symbol> {
	let mut name: Option<&str> = None; // symbol name from capture
	let mut kind: Option<SymbolKind> = None; // symbol kind
	let mut visibility: Visibility = Visibility::Private;
	let mut def_node: Option<tree_sitter::Node> = None;
	let mut parent_type: Option<&str> = None; // parent type for methods/fields

	for capture in match_.captures {
		let capture_name = symbol_query.capture_names()[capture.index as usize];
		let node = capture.node;
		let text = &source[node.byte_range()];

		match capture_name {
			// Function captures
			"function.name" => {
				name = Some(text);
				kind = Some(SymbolKind::Function);
			}
			"function.def" => def_node = Some(node),
			"function.visibility" => visibility = parse_visibility(text),

			// Struct captures
			"struct.name" => {
				name = Some(text);
				kind = Some(SymbolKind::Struct);
			}
			"struct.def" => def_node = Some(node),
			"struct.visibility" => visibility = parse_visibility(text),

			// Enum captures
			"enum.name" => {
				name = Some(text);
				kind = Some(SymbolKind::Enum);
			}
			"enum.def" => def_node = Some(node),
			"enum.visibility" => visibility = parse_visibility(text),

			// Trait captures
			"trait.name" => {
				name = Some(text);
				kind = Some(SymbolKind::Trait);
			}
			"trait.def" => def_node = Some(node),
			"trait.visibility" => visibility = parse_visibility(text),

			// Impl captures
			"impl.type" => {
				name = Some(text);
				kind = Some(SymbolKind::Impl);
			}
			"impl.def" => def_node = Some(node),

			// Const captures
			"const.name" => {
				name = Some(text);
				kind = Some(SymbolKind::Constant);
			}
			"const.def" => def_node = Some(node),
			"const.visibility" => visibility = parse_visibility(text),

			// Static captures
			"static.name" => {
				name = Some(text);
				kind = Some(SymbolKind::Static);
			}
			"static.def" => def_node = Some(node),
			"static.visibility" => visibility = parse_visibility(text),

			// Type alias captures
			"type.name" => {
				name = Some(text);
				kind = Some(SymbolKind::TypeAlias);
			}
			"type.def" => def_node = Some(node),
			"type.visibility" => visibility = parse_visibility(text),

			// Module captures
			"mod.name" => {
				name = Some(text);
				kind = Some(SymbolKind::Module);
			}
			"mod.def" => def_node = Some(node),
			"mod.visibility" => visibility = parse_visibility(text),

			// Macro captures
			"macro.name" => {
				name = Some(text);
				kind = Some(SymbolKind::Macro);
			}
			"macro.def" => def_node = Some(node),

			// Method captures
			"method.name" => {
				name = Some(text);
				kind = Some(SymbolKind::Method);
			}
			"method.def" => def_node = Some(node),
			"method.visibility" => visibility = parse_visibility(text),
			"method.parent_type" => parent_type = Some(text),

			// Enum variant captures
			"variant.name" => {
				name = Some(text);
				kind = Some(SymbolKind::EnumVariant);
			}
			"variant.def" => def_node = Some(node),
			"variant.parent" => parent_type = Some(text),

			// Field captures
			"field.name" => {
				name = Some(text);
				kind = Some(SymbolKind::Field);
			}
			"field.def" => def_node = Some(node),
			"field.visibility" => visibility = parse_visibility(text),
			"field.parent" => parent_type = Some(text),

			_ => {}
		}
	}

	// Build the symbol if we have the required information
	let name = name?;
	let kind = kind?;
	let node = def_node?;

	let location = CodeLocation::new(
		file_path.to_path_buf(),
		node.start_position().row + 1, // Convert to 1-indexed
		node.start_position().column + 1,
		node.start_byte(),
		node.end_byte() - node.start_byte(),
	);

	let mut symbol = Symbol::new(
		name.to_string(), kind, location,
	)
	.with_visibility(visibility);

	if let Some(parent) = parent_type {
		symbol = symbol.with_parent(parent.to_string());
	}

	// Extract body content for searchable symbol types
	if matches!(
		kind,
		SymbolKind::Function
			| SymbolKind::Method
			| SymbolKind::Struct
			| SymbolKind::Enum
			| SymbolKind::Trait
			| SymbolKind::Impl
			| SymbolKind::Macro
	) {
		let body = extract_body_content(
			source, &node, MAX_BODY_CONTENT_LEN,
		);
		if let Some(body) = body {
			symbol = symbol.with_content(body);
		}
	}

	Some(symbol)
}
