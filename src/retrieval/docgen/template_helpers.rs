//! Shared utility functions for template-based
//! documentation generation.
//!
//! Provides text extraction and formatting helpers
//! used by template_generate and template_composites.

/// Extract type annotation from a signature string.
///
/// Looks for `: Type` or `= Type` patterns in signatures
/// like `const MAX: usize = 100` or `x: i32`.
/// Skips `::` (path separators).
pub fn extract_type_from_sig(sig: &str) -> &str {
	let trimmed = sig.trim();
	if let Some(pos) = find_single_colon(trimmed) {
		let after_colon = &trimmed[pos + 1..];
		let type_part = after_colon
			.split('=')
			.next()
			.unwrap_or(after_colon);
		return type_part.trim();
	}
	if let Some(pos) = trimmed.find('=') {
		return trimmed[pos + 1..].trim();
	}
	trimmed
}

/// Find position of a single `:` (not part of `::`)
fn find_single_colon(text: &str) -> Option<usize> {
	let bytes = text.as_bytes();
	for (i, &byte) in bytes.iter().enumerate() {
		if byte != b':' {
			continue;
		}
		let next_is_colon = bytes
			.get(i + 1)
			.is_some_and(|&val| val == b':');
		let prev_is_colon = i > 0
			&& bytes[i - 1] == b':';
		if !next_is_colon && !prev_is_colon {
			return Some(i);
		}
	}
	None
}

/// Extract visibility modifier from a signature.
///
/// Returns "pub", "pub(crate)", or "" for private.
pub fn extract_visibility(sig: &str) -> &str {
	let trimmed = sig.trim();
	if trimmed.starts_with("pub(crate)") {
		return "pub(crate)";
	}
	if trimmed.starts_with("pub(super)") {
		return "pub(super)";
	}
	if trimmed.starts_with("pub ") {
		return "pub";
	}
	""
}

/// Format parent context for documentation.
///
/// Produces ` in `Parent`` or ` in file.rs` as fallback.
pub fn format_parent_ctx(
	parent: Option<&str>,
	path: &std::path::Path,
) -> String {
	if let Some(par) = parent {
		return format!(" in `{}`", par);
	}
	let file_name = path
		.file_name()
		.and_then(|name| name.to_str())
		.unwrap_or("unknown");
	format!(" in {}", file_name)
}

/// Join items with commas, truncating at max count.
///
/// Appends "... and N more" when truncated.
pub fn join_truncated(
	items: &[String],
	max: usize,
) -> String {
	if items.is_empty() {
		return String::from("(none)");
	}
	if items.len() <= max {
		return items.join(", ");
	}
	let shown: Vec<&str> =
		items.iter().take(max).map(String::as_str).collect();
	let remaining = items.len() - max;
	format!(
		"{}, ... and {} more",
		shown.join(", "),
		remaining,
	)
}
