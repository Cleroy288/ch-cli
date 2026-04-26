use std::path::Path;

use crate::indexer::symbols::{
	Symbol, SymbolKind,
};

use super::symbol_building_node::{
	extract_body, node_to_location,
	should_extract_body,
};
use super::symbol_processing::CaptureData;

pub fn build_symbol<'src>(
	data: CaptureData<'src>,
	source: &str,
	file_path: &Path,
) -> Option<Symbol> {
	let name = data.name?;
	let kind = data.kind?;
	let node = data.def_node?;

	let location =
		node_to_location(file_path, &node);
	let symbol = Symbol::new(
		name.to_string(), kind, location,
	)
	.with_visibility(data.visibility);

	let symbol =
		apply_parent(symbol, data.parent_type);
	let symbol =
		apply_body(symbol, kind, source, &node);
	Some(symbol)
}

fn apply_parent(
	symbol: Symbol,
	parent: Option<&str>,
) -> Symbol {
	match parent {
		Some(name) => {
			symbol.with_parent(name.to_string())
		}
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
