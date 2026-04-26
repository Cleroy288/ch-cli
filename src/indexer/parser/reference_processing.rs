use std::path::Path;

use tree_sitter::Query;

use crate::indexer::semantic::ReferenceContext;
use crate::indexer::symbols::{ByteSpan, CodeLocation};

use super::helpers::is_rust_keyword;
use super::types::ExtractedReference;

/// Process a reference match and extract reference info.
/// Maps tree-sitter capture names to reference contexts.
pub fn process_reference_match(
	match_: &tree_sitter::QueryMatch,
	source: &str,
	file_path: &Path,
	reference_query: &Query,
) -> Option<ExtractedReference> {
	let capture = match_.captures.first()?;
	let cap_name = &reference_query
		.capture_names()[capture.index as usize];
	let node = capture.node;
	let text = &source[node.byte_range()];
	let context = determine_reference_context(cap_name)?;

	if is_rust_keyword(text) {
		return None;
	}

	let bytes = ByteSpan {
		offset: node.start_byte(),
		length: node.end_byte() - node.start_byte(),
	};
	let location = CodeLocation::new(
		file_path.to_path_buf(),
		node.start_position().row + 1,
		node.start_position().column + 1,
		bytes,
	);

	Some(ExtractedReference {
		name: text.to_string(),
		location,
		context,
	})
}

/// Returns None for captures that should be skipped.
fn determine_reference_context(capture_name: &str) -> Option<ReferenceContext> {
	match capture_name {
		"call.name" | "method_call.name" => {
			Some(ReferenceContext::Call)
		}
		"type_ref.name" => {
			Some(ReferenceContext::Type)
		}
		"use.name" | "use.type_name" | "use.simple"
		| "use.list_item" | "use.list_type_item" => {
			Some(ReferenceContext::Import)
		}
		"field_access.name" => {
			Some(ReferenceContext::FieldAccess)
		}
		"ident.name" | "scoped.name" => {
			Some(ReferenceContext::Identifier)
		}
		_ => None,
	}
}

/// Deduplicate references by sorting and removing duplicates.
/// References with same name and location are considered duplicates.
pub fn deduplicate_references(references: &mut Vec<ExtractedReference>) {
	references.sort_by(|lhs, rhs| {
		lhs.location.line.cmp(&rhs.location.line)
			.then(lhs.location.column.cmp(&rhs.location.column))
			.then(lhs.name.cmp(&rhs.name))
	});
	references.dedup_by(|lhs, rhs| {
		lhs.name == rhs.name
			&& lhs.location.line == rhs.location.line
			&& lhs.location.column == rhs.location.column
	});
}
