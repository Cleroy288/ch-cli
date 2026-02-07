//! Query Parsing Utilities
//!
//! Provides stop-word filtering, identifier extraction,
//! LLM response parsing, and fallback query parsing.

use crate::retrieval::daemon::protocol::{
	FileFilter, QueryIntent, SearchSpec,
};

/// Common English stop words to filter from symbols
const STOP_WORDS: &[&str] = &[
	// Question words
	"how", "where", "what", "why", "when", "which",
	"who", "whom",
	// Articles and conjunctions
	"the", "a", "an", "and", "or", "but", "if", "then",
	"else",
	// Common verbs
	"is", "are", "was", "were", "be", "been", "being",
	"have", "has", "had", "do", "does", "did", "done",
	"can", "could", "would", "should", "will", "shall",
	"may", "might", "must",
	// Prepositions
	"to", "for", "in", "of", "on", "at", "by", "with",
	"from", "about", "into", "through", "during",
	"before", "after", "above", "below",
	// Query action verbs
	"find", "get", "show", "list", "explain", "describe",
	"tell", "give", "work", "works", "working", "use",
	"uses", "used", "using", "implement", "implemented",
	"implementing", "implementation", "call", "calls",
	"called", "calling", "define", "defined", "defining",
	"definition",
	// Pronouns
	"this", "that", "these", "those", "it", "its", "i",
	"me", "my", "we", "our",
	// Other common words
	"all", "any", "some", "no", "not", "only", "just",
	"also", "very", "so", "as", "such", "like", "than",
	"too", "more", "most", "less",
	// Code-query specific non-symbols
	"code", "file", "files", "function", "functions",
	"class", "classes", "method", "methods", "variable",
	"variables", "module", "modules",
];

/// Filter stop words from potential symbols
pub fn filter_stop_words(
	symbols: Vec<String>,
) -> Vec<String> {
	symbols
		.into_iter()
		.filter(|s| {
			let lower = s.to_lowercase();
			!STOP_WORDS.contains(&lower.as_str())
				&& s.len() > 1
				&& !s.chars().all(|c| c.is_numeric())
		})
		.collect()
}

/// Extract potential code identifiers from a query
pub fn extract_identifiers(query: &str) -> Vec<String> {
	let mut identifiers = Vec::new();

	for word in
		query.split(|c: char| !c.is_alphanumeric() && c != '_')
	{
		if word.is_empty() || word.len() < 2 {
			continue;
		}
		if is_code_identifier(word) {
			identifiers.push(word.to_string());
		}
	}

	identifiers
}

/// Check if a word looks like a code identifier
fn is_code_identifier(word: &str) -> bool {
	let is_camel = word.chars().any(|c| c.is_uppercase())
		&& word.chars().any(|c| c.is_lowercase());
	let is_snake = word.contains('_')
		&& word
			.chars()
			.all(|c| c.is_alphanumeric() || c == '_');
	let is_screaming = word
		.chars()
		.all(|c| c.is_uppercase() || c.is_numeric() || c == '_')
		&& word.len() > 2;

	(is_camel || is_snake || is_screaming)
		&& !STOP_WORDS
			.contains(&word.to_lowercase().as_str())
}

/// Check if query contains a whole word (not substring)
pub fn contains_word(query: &str, word: &str) -> bool {
	let query_bytes = query.as_bytes();

	for (i, _) in query.match_indices(word) {
		let before_ok = i == 0
			|| !query_bytes[i - 1].is_ascii_alphanumeric();
		let after_ok = i + word.len() >= query.len()
			|| !query_bytes[i + word.len()]
				.is_ascii_alphanumeric();

		if before_ok && after_ok {
			return true;
		}
	}
	false
}

/// Parse intent string to QueryIntent enum
pub fn parse_intent(intent: &str) -> QueryIntent {
	match intent.to_lowercase().as_str() {
		"find_definition" | "definition" | "goto" => {
			QueryIntent::FindDefinition
		}
		"find_usages" | "usages" | "references"
		| "refs" => QueryIntent::FindUsages,
		"understand" | "explain" | "how" => {
			QueryIntent::Understand
		}
		"modify" | "change" | "update" | "fix" => {
			QueryIntent::Modify
		}
		"debug" | "error" | "bug" | "issue" => {
			QueryIntent::Debug
		}
		_ => QueryIntent::Search,
	}
}

