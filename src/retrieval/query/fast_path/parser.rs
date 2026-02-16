//! FastPathParser implementation

use regex::Regex;

use super::super::fast_path_patterns::{
	FastPathResult, PatternType,
	SOURCE_CODE_PATTERNS, SymbolCandidate,
};

use super::extraction::{
	extract_by_regex, ExtractionParams,
};
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

	/// Check if query requests source code
	pub fn wants_source_code(
		&self, query: &str,
	) -> bool {
		let lower = query.to_lowercase();
		SOURCE_CODE_PATTERNS
			.iter()
			.any(|pat| lower.contains(pat))
	}

	/// Extract symbol candidates from a query
	pub fn extract_symbols(
		&self,
		query: &str,
	) -> FastPathResult {
		let mut symbols = Vec::new();
		let is_conceptual = is_conceptual_query(query);

		self.extract_camel(query, &mut symbols);
		self.extract_screaming(query, &mut symbols);
		self.extract_snake(query, &mut symbols);
		deduplicate_symbols(&mut symbols);
		build_result(symbols, is_conceptual)
	}
}

/// Individual extraction passes.
impl FastPathParser {
	/// Extract CamelCase symbols
	fn extract_camel(
		&self,
		query: &str,
		symbols: &mut Vec<SymbolCandidate>,
	) {
		extract_by_regex(
			&ExtractionParams {
				regex: &self.camel_case,
				pattern_type: PatternType::CamelCase,
				confidence: 0.95,
			},
			query,
			symbols,
		);
	}

	/// Extract SCREAMING_CASE symbols
	fn extract_screaming(
		&self,
		query: &str,
		symbols: &mut Vec<SymbolCandidate>,
	) {
		extract_by_regex(
			&ExtractionParams {
				regex: &self.screaming_case,
				pattern_type: PatternType::ScreamingCase,
				confidence: 0.9,
			},
			query,
			symbols,
		);
	}

	/// Extract snake_case symbols
	fn extract_snake(
		&self,
		query: &str,
		symbols: &mut Vec<SymbolCandidate>,
	) {
		extract_by_regex(
			&ExtractionParams {
				regex: &self.snake_case,
				pattern_type: PatternType::SnakeCase,
				confidence: 0.85,
			},
			query,
			symbols,
		);
	}
}

impl Default for FastPathParser {
	fn default() -> Self {
		Self::new()
	}
}
