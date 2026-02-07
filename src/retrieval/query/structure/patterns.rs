//! Structure Query Pattern Detectors
//!
//! Provides pattern matching functions for different
//! structure query formats.

use super::core::StructureQuery;
use super::utils::extract_target_after;

/// Pattern: "modules in X" or "what modules are in X"
pub(super) fn detect_modules_in(
	query_lower: &str,
) -> Option<StructureQuery> {
	if !query_lower.contains("modules in")
		&& !query_lower.contains("modules are in")
	{
		return None;
	}
	let target = extract_target_after(query_lower, "in ")?;
	Some(StructureQuery {
		target,
		is_module: true,
	})
}

/// Pattern: "what's in X directory" or "what is in X"
pub(super) fn detect_whats_in(
	query_lower: &str,
) -> Option<StructureQuery> {
	if !query_lower.contains("what's in")
		&& !query_lower.contains("what is in")
	{
		return None;
	}
	let target = extract_target_after(query_lower, "in ")?;
	Some(StructureQuery {
		target,
		is_module: false,
	})
}

/// Pattern: "structure of X"
pub(super) fn detect_structure_of(
	query_lower: &str,
) -> Option<StructureQuery> {
	if !query_lower.contains("structure of") {
		return None;
	}
	let target =
		extract_target_after(query_lower, "structure of ")?;
	Some(StructureQuery {
		target,
		is_module: true,
	})
}

/// Pattern: "X structure" (word before "structure")
pub(super) fn detect_x_structure(
	query_lower: &str,
) -> Option<StructureQuery> {
	if !query_lower.contains(" structure") {
		return None;
	}
	let words: Vec<&str> =
		query_lower.split_whitespace().collect();
	for (i, word) in words.iter().enumerate() {
		if *word == "structure" && i > 0 {
			let target = words[i - 1].to_string();
			if target != "module"
				&& target != "directory"
				&& target.len() > 2
			{
				return Some(StructureQuery {
					target,
					is_module: true,
				});
			}
		}
	}
	None
}
