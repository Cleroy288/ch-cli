use std::path::Path;

use crate::indexer::symbols::{
	ByteSpan, CodeLocation, SymbolKind,
};

/// Max characters to extract from function body
const MAX_BODY_CONTENT_LEN: usize = 500;

pub(super) fn node_to_location(
	file_path: &Path,
	node: &tree_sitter::Node,
) -> CodeLocation {
	let bytes = ByteSpan {
		offset: node.start_byte(),
		length: node.end_byte()
			- node.start_byte(),
	};
	CodeLocation::new(
		file_path.to_path_buf(),
		node.start_position().row + 1,
		node.start_position().column + 1,
		bytes,
	)
}

pub(super) fn should_extract_body(
	kind: SymbolKind,
) -> bool {
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
pub(super) fn extract_body(
	source: &str,
	node: &tree_sitter::Node,
) -> Option<String> {
	let start = node.start_byte();
	let raw_end = node.end_byte()
		.min(start + MAX_BODY_CONTENT_LEN);
	let end =
		snap_char_boundary_down(source, raw_end, start);
	let content = source.get(start..end)?;

	if end < node.end_byte()
		&& content.len() >= MAX_BODY_CONTENT_LEN
	{
		if let Some(last_space) =
			content.rfind(char::is_whitespace)
		{
			return Some(
				content[..last_space].to_string()
					+ "...",
			);
		}
	}
	Some(content.to_string())
}

/// Find the nearest valid char boundary at or before idx
fn snap_char_boundary_down(
	s: &str,
	idx: usize,
	floor: usize,
) -> usize {
	if idx >= s.len() {
		return s.len();
	}
	let mut i = idx;
	while i > floor && !s.is_char_boundary(i) {
		i -= 1;
	}
	i
}
