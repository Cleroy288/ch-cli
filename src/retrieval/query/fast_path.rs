//! Fast Path Parser
//!
//! Extracts symbol candidates from queries using regex patterns.
//! Enables fast-path retrieval by bypassing LLM for explicit symbol queries.

use regex::Regex;

/// Stop words to filter out from symbol extraction
const STOP_WORDS: &[&str] = &[
	"how", "where", "what", "when", "why", "which", "who",
	"the", "a", "an", "is", "are", "was", "were", "be", "been",
	"does", "do", "did", "has", "have", "had", "can", "could",
	"will", "would", "should", "shall", "may", "might", "must",
	"find", "show", "get", "search", "look", "locate", "defined",
	"used", "called", "work", "works", "working", "implement",
];

/// Patterns indicating conceptual/explanatory queries
const CONCEPTUAL_PATTERNS: &[&str] = &[
	"why does",
	"why is",
	"how does",
	"how do",
	"explain",
	"what happens",
	"what is the purpose",
	"understand",
	"describe",
];

/// Type of pattern that matched a symbol candidate
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PatternType {
	/// CamelCase pattern (e.g., AuthService, BgeEmbedder)
	CamelCase,
	/// snake_case pattern (e.g., parse_config, get_user)
	SnakeCase,
	/// SCREAMING_CASE pattern (e.g., MAX_TOKENS, DEFAULT_LLM)
	ScreamingCase,
	/// Generic identifier (alphanumeric)
	Identifier,
}

/// A candidate symbol extracted from the query
#[derive(Debug, Clone)]
pub struct SymbolCandidate {
	/// the symbol name
	pub name: String,
	/// the pattern type that matched
	pub pattern_type: PatternType,
	/// confidence that this is a real symbol (0.0 - 1.0)
	pub confidence: f32,
}

/// Query intent classification for fast-path decision
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FastPathIntent {
	/// Explicit symbol lookup (high confidence fast-path)
	Explicit,
	/// Conceptual/explanatory query (needs LLM)
	Conceptual,
	/// Mixed: has symbols but also conceptual elements
	Mixed,
}

/// Result of fast-path parsing
#[derive(Debug, Clone)]
pub struct FastPathResult {
	/// extracted symbol candidates
	pub symbols: Vec<SymbolCandidate>,
	/// overall confidence score (0.0 - 1.0)
	pub confidence: f32,
	/// detected query intent
	pub intent: FastPathIntent,
	/// whether to use fast-path (skip LLM)
	pub use_fast_path: bool,
}

/// Parser for extracting symbols via regex patterns
pub struct FastPathParser {
	/// regex for CamelCase symbols
	camel_case: Regex,
	/// regex for snake_case symbols
	snake_case: Regex,
	/// regex for SCREAMING_CASE symbols
	screaming_case: Regex,
}

impl FastPathParser {
	/// Create a new FastPathParser with compiled regexes
	pub fn new() -> Self {
		Self {
			camel_case: Regex::new(r"\b[A-Z][a-z]+(?:[A-Z][a-z0-9]+)+\b").unwrap(),
			snake_case: Regex::new(r"\b[a-z][a-z0-9]*(?:_[a-z0-9]+)+\b").unwrap(),
			screaming_case: Regex::new(r"\b[A-Z][A-Z0-9]*(?:_[A-Z0-9]+)+\b").unwrap(),
		}
	}

	/// Check if a word is a stop word
	fn is_stop_word(&self, word: &str) -> bool {
		STOP_WORDS.contains(&word.to_lowercase().as_str())
	}

	/// Check if query is conceptual (needs LLM understanding)
	fn is_conceptual_query(&self, query: &str) -> bool {
		let lower = query.to_lowercase();
		CONCEPTUAL_PATTERNS.iter().any(|p| lower.contains(p))
	}

	/// Extract symbol candidates from a query
	pub fn extract_symbols(&self, query: &str) -> FastPathResult {
		let mut symbols = Vec::new(); // collected symbol candidates
		let is_conceptual = self.is_conceptual_query(query); // whether query is conceptual

		// Extract CamelCase symbols (highest confidence)
		for cap in self.camel_case.find_iter(query) {
			let name = cap.as_str().to_string();
			if !self.is_stop_word(&name) {
				symbols.push(SymbolCandidate {
					name,
					pattern_type: PatternType::CamelCase,
					confidence: 0.95,
				});
			}
		}

		// Extract SCREAMING_CASE symbols (high confidence for constants)
		for cap in self.screaming_case.find_iter(query) {
			let name = cap.as_str().to_string();
			if !self.is_stop_word(&name) {
				symbols.push(SymbolCandidate {
					name,
					pattern_type: PatternType::ScreamingCase,
					confidence: 0.9,
				});
			}
		}

		// Extract snake_case symbols (good confidence)
		for cap in self.snake_case.find_iter(query) {
			let name = cap.as_str().to_string();
			if !self.is_stop_word(&name) {
				symbols.push(SymbolCandidate {
					name,
					pattern_type: PatternType::SnakeCase,
					confidence: 0.85,
				});
			}
		}

		// Deduplicate symbols by name
		symbols.sort_by(|a, b| b.confidence.partial_cmp(&a.confidence).unwrap());
		symbols.dedup_by(|a, b| a.name == b.name);

		// Calculate overall confidence
		let overall_confidence = if symbols.is_empty() {
			0.0
		} else {
			symbols.iter().map(|s| s.confidence).sum::<f32>() / symbols.len() as f32
		};

		// Determine intent
		let intent = if is_conceptual {
			if symbols.is_empty() {
				FastPathIntent::Conceptual
			} else {
				FastPathIntent::Mixed
			}
		} else if symbols.is_empty() {
			FastPathIntent::Conceptual
		} else {
			FastPathIntent::Explicit
		};

		// Decide whether to use fast-path
		let use_fast_path = intent == FastPathIntent::Explicit && overall_confidence >= 0.7;

		FastPathResult {
			symbols,
			confidence: overall_confidence,
			intent,
			use_fast_path,
		}
	}
}

impl Default for FastPathParser {
	fn default() -> Self {
		Self::new()
	}
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test_extract_camel_case() {
		let parser = FastPathParser::new();
		let result = parser.extract_symbols("find AuthService");
		assert_eq!(result.symbols.len(), 1);
		assert_eq!(result.symbols[0].name, "AuthService");
		assert_eq!(result.symbols[0].pattern_type, PatternType::CamelCase);
		assert!(result.use_fast_path);
	}

	#[test]
	fn test_extract_snake_case() {
		let parser = FastPathParser::new();
		let result = parser.extract_symbols("where is parse_config");
		assert_eq!(result.symbols.len(), 1);
		assert_eq!(result.symbols[0].name, "parse_config");
		assert_eq!(result.symbols[0].pattern_type, PatternType::SnakeCase);
	}

	#[test]
	fn test_extract_screaming_case() {
		let parser = FastPathParser::new();
		let result = parser.extract_symbols("what is MAX_TOKENS");
		assert_eq!(result.symbols.len(), 1);
		assert_eq!(result.symbols[0].name, "MAX_TOKENS");
		assert_eq!(result.symbols[0].pattern_type, PatternType::ScreamingCase);
	}

	#[test]
	fn test_conceptual_query() {
		let parser = FastPathParser::new();
		let result = parser.extract_symbols("why does the daemon fail to start");
		assert_eq!(result.intent, FastPathIntent::Conceptual);
		assert!(!result.use_fast_path);
	}

	#[test]
	fn test_mixed_query() {
		let parser = FastPathParser::new();
		let result = parser.extract_symbols("how does AuthService handle tokens");
		assert_eq!(result.intent, FastPathIntent::Mixed);
		assert!(!result.use_fast_path);
	}

	#[test]
	fn test_multiple_symbols() {
		let parser = FastPathParser::new();
		let result = parser.extract_symbols("BgeEmbedder and HybridSearch");
		assert_eq!(result.symbols.len(), 2);
		assert!(result.use_fast_path);
	}

	#[test]
	fn test_stop_word_filter() {
		let parser = FastPathParser::new();
		// "find" and "search" should be filtered as stop words
		let result = parser.extract_symbols("find search");
		assert!(result.symbols.is_empty());
	}
}
