//! Symbol extraction from regex patterns

use regex::Regex;

use super::super::fast_path_patterns::{
	PatternType, SymbolCandidate, STOP_WORDS,
};

/// Extract matches of a single regex pattern
pub(crate) fn extract_by_regex(
	regex: &Regex,
	query: &str,
	pattern_type: PatternType,
	confidence: f32,
	symbols: &mut Vec<SymbolCandidate>,
) {
	for cap in regex.find_iter(query) {
		let name = cap.as_str().to_string();
		if !is_stop_word(&name) {
			symbols.push(SymbolCandidate {
				name,
				pattern_type,
				confidence,
			});
		}
	}
}

/// Check if a word is a stop word
fn is_stop_word(word: &str) -> bool {
	STOP_WORDS.contains(&word.to_lowercase().as_str())
}
