//! Structure Query Detection Core
//!
//! Defines the StructureQuery type and main detection function.

use super::patterns::{
	detect_modules_in, detect_structure_of, detect_whats_in,
	detect_x_structure,
};

/// Result of structure query detection
#[derive(Debug, Clone)]
pub struct StructureQuery {
	/// the target directory/module name
	pub target: String,
	/// whether this is a module query (vs directory)
	pub is_module: bool,
}

/// Detect if a query is asking about structure
/// Returns Some(StructureQuery) if detected, None otherwise
pub fn detect_structure_query(
	query: &str,
) -> Option<StructureQuery> {
	let query_lower = query.to_lowercase();

	if let Some(sq) = detect_modules_in(&query_lower) {
		return Some(sq);
	}
	if let Some(sq) = detect_whats_in(&query_lower) {
		return Some(sq);
	}
	if let Some(sq) = detect_structure_of(&query_lower) {
		return Some(sq);
	}
	detect_x_structure(&query_lower)
}
