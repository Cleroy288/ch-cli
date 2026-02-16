//! Structure Query Detection Core
//!
//! Defines the StructureQuery type and main detection.

use super::patterns::{
	detect_modules_in, detect_structure_of,
	detect_whats_in, detect_x_structure,
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
pub fn detect_structure_query(
	query: &str,
) -> Option<StructureQuery> {
	let query_lower = query.to_lowercase();

	if let Some(result) =
		detect_modules_in(&query_lower)
	{
		return Some(result);
	}
	if let Some(result) =
		detect_whats_in(&query_lower)
	{
		return Some(result);
	}
	if let Some(result) =
		detect_structure_of(&query_lower)
	{
		return Some(result);
	}
	detect_x_structure(&query_lower)
}
