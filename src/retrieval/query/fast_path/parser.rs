//! FastPathParser implementation

use regex::Regex;

use super::super::fast_path_patterns::{
	FastPathIntent, FastPathResult, PatternType,
	SOURCE_CODE_PATTERNS, SymbolCandidate,
};

use super::extraction::extract_by_regex;
use super::intent::is_conceptual_query;
use super::result::{build_result, deduplicate_symbols};

/// Parser for extracting symbols via regex patterns
pub struct FastPathParser {
	/// regex for CamelCase symbols
	pub(crate) camel_case: Regex,
	/// regex for snake_case symbols
	pub(crate) snake_case: Regex,
	/// regex for SCREAMING_CASE symbols
	pub(crate) screaming_case: Regex,
}

impl FastPathParser {
	/// Create a new FastPathParser with compiled regexes
	pub fn new() -> Self {
		Self {
			camel_case: Regex::new(
				r"\b[A-Z][a-z]+(?:[A-Z][a-z0-9]+)+\b",
			)
			.unwrap(),
			snake_case: Regex::new(
				r"\b[a-z][a-z0-9]*(?:_[a-z0-9]+)+\b",
			)
			.unwrap(),
			screaming_case: Regex::new(
				r"\b[A-Z][A-Z0-9]*(?:_[A-Z0-9]+)+\b",
			)
			.unwrap(),
		}
	}

	/// Check if query explicitly requests source code
	pub fn wants_source_code(&self, query: &str) -> bool {
		let lower = query.to_lowercase();
		SOURCE_CODE_PATTERNS
			.iter()
			.any(|p| lower.contains(p))
	}

	/// Extract symbol candidates from a query
	pub fn extract_symbols(
		&self,
		query: &str,
	) -> FastPathResult {
		let mut symbols = Vec::new();
		let is_conceptual = is_conceptual_query(query);

		extract_by_regex(
			&self.camel_case,
			query,
			PatternType::CamelCase,
			0.95,
			&mut symbols,
		);
		extract_by_regex(
			&self.screaming_case,
			query,
			PatternType::ScreamingCase,
			0.9,
			&mut symbols,
		);
		extract_by_regex(
			&self.snake_case,
			query,
			PatternType::SnakeCase,
			0.85,
			&mut symbols,
		);

		deduplicate_symbols(&mut symbols);
		build_result(symbols, is_conceptual)
	}
}

impl Default for FastPathParser {
	fn default() -> Self {
		Self::new()
	}
}
