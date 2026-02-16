//! Symbol processing functions.
//!
//! Contains the logic for processing tree-sitter
//! query matches into Symbol objects.

use std::path::Path;

use tree_sitter::Query;

use crate::indexer::symbols::{
	Symbol, SymbolKind, Visibility,
};

use super::helpers::parse_visibility;
use super::symbol_building::build_symbol;

/// Intermediate capture data from a query match
pub(super) struct CaptureData<'src> {
	pub(super) name: Option<&'src str>,
	pub(super) kind: Option<SymbolKind>,
	pub(super) visibility: Visibility,
	pub(super) def_node: Option<tree_sitter::Node<'src>>,
	pub(super) parent_type: Option<&'src str>,
}

/// Process a query match and extract a symbol
pub fn process_match(
	match_: &tree_sitter::QueryMatch,
	source: &str,
	file_path: &Path,
	symbol_query: &Query,
) -> Option<Symbol> {
	let data = extract_captures(
		match_, source, symbol_query,
	);
	build_symbol(data, source, file_path)
}

/// Extract capture data from all match captures
fn extract_captures<'src>(
	match_: &'src tree_sitter::QueryMatch,
	source: &'src str,
	symbol_query: &Query,
) -> CaptureData<'src> {
	let mut data = CaptureData {
		name: None,
		kind: None,
		visibility: Visibility::Private,
		def_node: None,
		parent_type: None,
	};

	for capture in match_.captures {
		let cap_name = symbol_query
			.capture_names()[capture.index as usize];
		let node = capture.node;
		let text = &source[node.byte_range()];
		apply_capture(&mut data, cap_name, text, node);
	}

	data
}

/// Apply a single capture to the data struct
#[allow(clippy::cognitive_complexity)]
fn apply_capture<'src>(
	data: &mut CaptureData<'src>,
	cap_name: &str,
	text: &'src str,
	node: tree_sitter::Node<'src>,
) {
	match cap_name {
		n if n.ends_with(".name") => {
			data.name = Some(text);
			data.kind = kind_from_prefix(n);
		}
		n if n.ends_with(".def") => {
			data.def_node = Some(node);
		}
		n if n.ends_with(".visibility") => {
			data.visibility = parse_visibility(text);
		}
		n if n.ends_with(".parent_type")
			|| n.ends_with(".parent") =>
		{
			data.parent_type = Some(text);
		}
		"impl.type" => {
			data.name = Some(text);
			data.kind = Some(SymbolKind::Impl);
		}
		_ => {}
	}
}

/// Derive SymbolKind from the capture name prefix
fn kind_from_prefix(name: &str) -> Option<SymbolKind> {
	let prefix = name.split('.').next()?;
	match prefix {
		"function" => Some(SymbolKind::Function),
		"struct" => Some(SymbolKind::Struct),
		"enum" => Some(SymbolKind::Enum),
		"trait" => Some(SymbolKind::Trait),
		"impl" => Some(SymbolKind::Impl),
		"const" => Some(SymbolKind::Constant),
		"static" => Some(SymbolKind::Static),
		"type" => Some(SymbolKind::TypeAlias),
		"mod" => Some(SymbolKind::Module),
		"macro" => Some(SymbolKind::Macro),
		"method" => Some(SymbolKind::Method),
		"variant" => Some(SymbolKind::EnumVariant),
		"field" => Some(SymbolKind::Field),
		_ => None,
	}
}
