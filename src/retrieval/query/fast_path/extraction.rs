//! Symbol extraction from regex patterns

use regex::Regex;

use super::super::fast_path_patterns::{
	PatternType, SymbolCandidate, STOP_WORDS,
};

/// Parameters for a single regex extraction pass
pub(crate) struct ExtractionParams<'pat> {
	/// compiled regex to match symbols
	pub regex: &'pat Regex,
	/// pattern type tag for matched candidates
	pub pattern_type: PatternType,
	/// confidence score for matched candidates
	pub confidence: f32,
}

/// Extract matches of a single regex pattern
#[allow(clippy::min_ident_chars)]
pub(crate) fn extract_by_regex(
	params: &ExtractionParams<'_>,
	query: &str,
	symbols: &mut Vec<SymbolCandidate>,
) {
	for cap in params.regex.find_iter(query) {
		let name = cap.as_str().to_string();
		if !is_stop_word(&name) {
			symbols.push(SymbolCandidate {
				name,
				pattern_type: params.pattern_type,
				confidence: params.confidence,
			});
		}
	}
}

/// Check if a word is a stop word
fn is_stop_word(word: &str) -> bool {
	STOP_WORDS.contains(&word.to_lowercase().as_str())
}
