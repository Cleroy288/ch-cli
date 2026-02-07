//! Structure Query Utilities
//!
//! Helper functions for extracting target names.

/// Articles and common words to skip when extracting target
const SKIP_WORDS: &[&str] =
	&["the", "a", "an", "this", "that", "our", "my"];

/// Extract target name after a keyword
pub(super) fn extract_target_after(
	query: &str,
	keyword: &str,
) -> Option<String> {
	let idx = query.find(keyword)?;
	let rest = &query[idx + keyword.len()..];

	for word in rest.split_whitespace() {
		let cleaned = word
			.trim_end_matches("directory")
			.trim_end_matches("module")
			.trim_end_matches("folder")
			.trim_end_matches('?')
			.to_lowercase();

		if SKIP_WORDS.contains(&cleaned.as_str()) {
			continue;
		}
		if cleaned.len() > 1 {
			return Some(cleaned);
		}
	}

	None
}
