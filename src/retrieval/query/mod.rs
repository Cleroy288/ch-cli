//! Query Expansion Module
//!
//! Uses a local LLM (Phi-3-mini) to interpret natural language queries
//! and convert them into structured SearchSpec for code search.
//!
//! Also provides fast-path optimization via tiered expansion:
//! - Tier 1: Regex-based symbol extraction
//! - Tier 2: SemanticGraph validation
//! - Tier 3: LLM fallback for conceptual queries

pub mod fast_path;
pub mod interpreter;
pub mod llm;
pub mod rewriter;
pub mod structure;
pub mod tiered;
pub mod validator;

pub use fast_path::{FastPathIntent, FastPathParser, FastPathResult, SymbolCandidate};
pub use interpreter::QueryInterpreter;
pub use llm::Phi3Model;
pub use rewriter::{QueryRewriter, RewriteType, RewrittenQuery};
pub use structure::{
	detect_structure_query, find_module_structure, list_source_directories,
	ModuleInfo, StructureQuery, SubmoduleDecl,
};
pub use tiered::{TierUsed, TieredConfig, TieredQueryExpander, TieredResult};
pub use validator::{SymbolValidator, ValidatedSymbol, ValidationResult};

use crate::retrieval::daemon::protocol::{FileFilter, QueryIntent, SearchSpec};

/// Common English stop words that should not be treated as code symbols.
/// These words are filtered from LLM output and fallback parsing.
const STOP_WORDS: &[&str] = &[
	// Question words
	"how", "where", "what", "why", "when", "which", "who", "whom",
	// Articles and conjunctions
	"the", "a", "an", "and", "or", "but", "if", "then", "else",
	// Common verbs
	"is", "are", "was", "were", "be", "been", "being",
	"have", "has", "had", "do", "does", "did", "done",
	"can", "could", "would", "should", "will", "shall", "may", "might", "must",
	// Prepositions
	"to", "for", "in", "of", "on", "at", "by", "with", "from", "about",
	"into", "through", "during", "before", "after", "above", "below",
	// Query action verbs
	"find", "get", "show", "list", "explain", "describe", "tell", "give",
	"work", "works", "working", "use", "uses", "used", "using",
	"implement", "implemented", "implementing", "implementation",
	"call", "calls", "called", "calling",
	"define", "defined", "defining", "definition",
	// Pronouns
	"this", "that", "these", "those", "it", "its", "i", "me", "my", "we", "our",
	// Other common words
	"all", "any", "some", "no", "not", "only", "just", "also", "very",
	"so", "as", "such", "like", "than", "too", "more", "most", "less",
	// Code-query specific words that aren't symbols
	"code", "file", "files", "function", "functions", "class", "classes",
	"method", "methods", "variable", "variables", "module", "modules",
];

/// Filter stop words from a list of potential symbols.
/// Returns only words that are likely to be actual code identifiers.
pub fn filter_stop_words(symbols: Vec<String>) -> Vec<String> {
	symbols
		.into_iter()
		.filter(|s| {
			let lower = s.to_lowercase();
			// filter stop words
			!STOP_WORDS.contains(&lower.as_str())
			// filter single characters
			&& s.len() > 1
			// filter pure numbers
			&& !s.chars().all(|c| c.is_numeric())
		})
		.collect()
}

/// Extract potential code identifiers from a query string.
/// Looks for CamelCase, snake_case, and SCREAMING_CASE patterns.
pub fn extract_identifiers(query: &str) -> Vec<String> {
	let mut identifiers = Vec::new();

	for word in query.split(|c: char| !c.is_alphanumeric() && c != '_') {
		if word.is_empty() || word.len() < 2 {
			continue;
		}

		// check if it looks like a code identifier
		let is_camel = word.chars().any(|c| c.is_uppercase())
			&& word.chars().any(|c| c.is_lowercase());
		let is_snake = word.contains('_') && word.chars().all(|c| c.is_alphanumeric() || c == '_');
		let is_screaming = word.chars().all(|c| c.is_uppercase() || c.is_numeric() || c == '_')
			&& word.len() > 2;

		if is_camel || is_snake || is_screaming {
			if !STOP_WORDS.contains(&word.to_lowercase().as_str()) {
				identifiers.push(word.to_string());
			}
		}
	}

	identifiers
}

/// Default prompt template for query expansion
pub const QUERY_EXPANSION_PROMPT: &str = r#"You are a code search assistant. Given a natural language query about code, extract:
1. Symbol names (functions, classes, structs, etc.) to search for
2. The user's intent (find_definition, find_usages, understand, modify, debug, search)
3. File patterns to filter (e.g., "*.rs", "src/auth/**")

Query: {query}

Respond in this exact JSON format:
{
  "symbols": ["symbol1", "symbol2"],
  "intent": "search",
  "file_patterns": ["*.rs"],
  "hints": ["any additional context"]
}

JSON response:"#;

/// Parse intent string to QueryIntent enum
pub fn parse_intent(intent: &str) -> QueryIntent {
	match intent.to_lowercase().as_str() {
		"find_definition" | "definition" | "goto" => QueryIntent::FindDefinition,
		"find_usages" | "usages" | "references" | "refs" => QueryIntent::FindUsages,
		"understand" | "explain" | "how" => QueryIntent::Understand,
		"modify" | "change" | "update" | "fix" => QueryIntent::Modify,
		"debug" | "error" | "bug" | "issue" => QueryIntent::Debug,
		_ => QueryIntent::Search,
	}
}

/// Parse LLM response JSON into SearchSpec
pub fn parse_llm_response(query: &str, response: &str) -> SearchSpec {
	// try to parse JSON from response
	if let Ok(json) = serde_json::from_str::<serde_json::Value>(response) {
		let raw_symbols: Vec<String> = json
			.get("symbols")
			.and_then(|v| v.as_array())
			.map(|arr| {
				arr.iter()
					.filter_map(|v| v.as_str().map(String::from))
					.collect()
			})
			.unwrap_or_default();

		// filter stop words from LLM output
		let mut symbols = filter_stop_words(raw_symbols);

		// if all symbols were filtered, extract identifiers from query
		if symbols.is_empty() {
			symbols = extract_identifiers(query);
		}

		let intent = json
			.get("intent")
			.and_then(|v| v.as_str())
			.map(parse_intent)
			.unwrap_or(QueryIntent::Search);

		let file_filters = json
			.get("file_patterns")
			.and_then(|v| v.as_array())
			.map(|arr| {
				arr.iter()
					.filter_map(|v| {
						v.as_str().map(|s| FileFilter {
							pattern: s.to_string(),
							include: true,
						})
					})
					.collect()
			})
			.unwrap_or_default();

		let hints = json
			.get("hints")
			.and_then(|v| v.as_array())
			.map(|arr| {
				arr.iter()
					.filter_map(|v| v.as_str().map(String::from))
					.collect()
			})
			.unwrap_or_default();

		SearchSpec {
			original_query: query.to_string(),
			symbol_names: symbols,
			intent,
			file_filters,
			context_hints: hints,
		}
	} else {
		// fallback: extract words from query
		fallback_parse(query)
	}
}

/// Fallback parser when LLM response is not valid JSON
pub fn fallback_parse(query: &str) -> SearchSpec {
	// extract identifiers using our robust extractor
	let symbols = extract_identifiers(query);

	// detect intent from keywords in query
	let query_lower = query.to_lowercase();
	let intent = if query_lower.contains("where") || query_lower.contains("definition") || query_lower.contains("defined") {
		QueryIntent::FindDefinition
	} else if query_lower.contains("used") || query_lower.contains("calls") || query_lower.contains("references") || query_lower.contains("usages") {
		QueryIntent::FindUsages
	} else if query_lower.contains("how") || query_lower.contains("what") || query_lower.contains("explain") || query_lower.contains("understand") {
		QueryIntent::Understand
	} else if query_lower.contains("fix") || query_lower.contains("change") || query_lower.contains("modify") || query_lower.contains("update") {
		QueryIntent::Modify
	} else if query_lower.contains("bug") || query_lower.contains("error") || query_lower.contains("debug") || query_lower.contains("issue") {
		QueryIntent::Debug
	} else {
		QueryIntent::Search
	};

	SearchSpec {
		original_query: query.to_string(),
		symbol_names: symbols,
		intent,
		file_filters: Vec::new(),
		context_hints: Vec::new(),
	}
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test_parse_intent() {
		assert!(matches!(parse_intent("find_definition"), QueryIntent::FindDefinition));
		assert!(matches!(parse_intent("usages"), QueryIntent::FindUsages));
		assert!(matches!(parse_intent("understand"), QueryIntent::Understand));
		assert!(matches!(parse_intent("random"), QueryIntent::Search));
	}

	#[test]
	fn test_fallback_parse() {
		let spec = fallback_parse("where is AuthService defined");
		assert!(spec.symbol_names.contains(&"AuthService".to_string()));
		assert!(matches!(spec.intent, QueryIntent::FindDefinition));
	}

	#[test]
	fn test_parse_llm_response() {
		let json = r#"{"symbols": ["foo", "bar"], "intent": "understand", "file_patterns": ["*.rs"], "hints": []}"#;
		let spec = parse_llm_response("test query", json);
		assert_eq!(spec.symbol_names, vec!["foo", "bar"]);
		assert!(matches!(spec.intent, QueryIntent::Understand));
	}

	#[test]
	fn test_filter_stop_words() {
		// should filter common stop words
		let input = vec!["How".to_string(), "does".to_string(), "AuthService".to_string(), "work".to_string()];
		let filtered = filter_stop_words(input);
		assert_eq!(filtered, vec!["AuthService"]);

		// should filter single chars
		let input2 = vec!["a".to_string(), "b".to_string(), "Parser".to_string()];
		let filtered2 = filter_stop_words(input2);
		assert_eq!(filtered2, vec!["Parser"]);

		// should keep valid identifiers
		let input3 = vec!["BgeEmbedder".to_string(), "parse_config".to_string()];
		let filtered3 = filter_stop_words(input3);
		assert_eq!(filtered3, vec!["BgeEmbedder", "parse_config"]);
	}

	#[test]
	fn test_extract_identifiers() {
		// should extract CamelCase
		let ids = extract_identifiers("How does AuthService work?");
		assert!(ids.contains(&"AuthService".to_string()));
		assert!(!ids.contains(&"How".to_string()));

		// should extract snake_case
		let ids2 = extract_identifiers("find parse_config function");
		assert!(ids2.contains(&"parse_config".to_string()));

		// should extract from complex query
		let ids3 = extract_identifiers("How does the RetrievalPipeline work with HybridSearch?");
		assert!(ids3.contains(&"RetrievalPipeline".to_string()));
		assert!(ids3.contains(&"HybridSearch".to_string()));
	}

	#[test]
	fn test_parse_llm_response_filters_stop_words() {
		// LLM returns stop words - should be filtered
		let json = r#"{"symbols": ["How", "does", "BgeEmbedder", "work"], "intent": "understand", "file_patterns": [], "hints": []}"#;
		let spec = parse_llm_response("How does BgeEmbedder work?", json);
		assert_eq!(spec.symbol_names, vec!["BgeEmbedder"]);
		assert!(matches!(spec.intent, QueryIntent::Understand));
	}

	#[test]
	fn test_fallback_parse_extracts_identifiers() {
		// conceptual query with identifiers
		let spec = fallback_parse("How does the RetrievalPipeline work?");
		assert!(spec.symbol_names.contains(&"RetrievalPipeline".to_string()));
		assert!(!spec.symbol_names.contains(&"How".to_string()));
		assert!(matches!(spec.intent, QueryIntent::Understand));
	}
}
