//! Reference processing functions.
//!
//! Contains logic for extracting symbol references (usages)
//! from tree-sitter matches.

use std::path::Path;

use tree_sitter::Query;

use crate::indexer::semantic::ReferenceContext;
use crate::indexer::symbols::CodeLocation;

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
	// Find the most specific capture
	let capture = match_.captures.first()?;
	let capture_name = reference_query.capture_names()[capture.index as usize];
	let node = capture.node;
	let text = &source[node.byte_range()];

	// Determine reference context from capture name
	let context = determine_reference_context(capture_name)?;

	// Skip keywords and common identifiers that aren't real references
	if is_rust_keyword(text) || text == "self" || text == "Self" {
		return None;
	}

	let location = CodeLocation::new(
		file_path.to_path_buf(),
		node.start_position().row + 1,
		node.start_position().column + 1,
		node.start_byte(),
		node.end_byte() - node.start_byte(),
	);

	Some(ExtractedReference {
		name: text.to_string(),
		location,
		context,
	})
}

/// Determine the reference context from a capture name.
/// Returns None for captures that should be skipped.
fn determine_reference_context(capture_name: &str) -> Option<ReferenceContext> {
	match capture_name {
		"call.name" => Some(ReferenceContext::Call),
		"method_call.name" => Some(ReferenceContext::Call),
		"type_ref.name" => Some(ReferenceContext::Type),
		"use.name" | "use.type_name" | "use.simple"
		| "use.list_item" | "use.list_type_item" => {
			Some(ReferenceContext::Import)
		}
		"field_access.name" => Some(ReferenceContext::FieldAccess),
		"ident.name" => Some(ReferenceContext::Identifier),
		"scoped.name" => Some(ReferenceContext::Identifier),
		_ => None, // Skip expression/declaration captures, we want names
	}
}

/// Deduplicate references by sorting and removing duplicates.
/// References with same name and location are considered duplicates.
pub fn deduplicate_references(references: &mut Vec<ExtractedReference>) {
	references.sort_by(|a, b| {
		a.location.line.cmp(&b.location.line)
			.then(a.location.column.cmp(&b.location.column))
			.then(a.name.cmp(&b.name))
	});
	references.dedup_by(|a, b| {
		a.name == b.name
			&& a.location.line == b.location.line
			&& a.location.column == b.location.column
	});
}
