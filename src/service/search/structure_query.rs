use super::structure_query_match::{
	detect_modules_in, detect_structure_of,
	detect_whats_in, detect_x_structure,
};

/// Parsed "show structure of X" query
#[derive(Debug, Clone)]
pub struct StructureQuery {
	pub target: String,
	pub is_module: bool,
}

/// Articles and common words to skip
const SKIP_WORDS: &[&str] =
	&["the", "a", "an", "this", "that", "our", "my"];

pub fn detect_structure_query(
	query: &str,
) -> Option<StructureQuery> {
	let lower = query.to_lowercase();
	if let Some(res) = detect_modules_in(&lower) {
		return Some(res);
	}
	if let Some(res) = detect_whats_in(&lower) {
		return Some(res);
	}
	if let Some(res) = detect_structure_of(&lower) {
		return Some(res);
	}
	detect_x_structure(&lower)
}

/// Extract target name after a keyword
pub(crate) fn extract_target(
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
