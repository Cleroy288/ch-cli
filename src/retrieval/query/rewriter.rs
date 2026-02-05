//! Query Rewriter for Conceptual Queries
//!
//! Implements multi-query rewriting (DMQR-RAG approach) to improve
//! retrieval for conceptual/natural language queries.

use crate::retrieval::daemon::DaemonClient;
use crate::retrieval::RetrievalResult;

/// A rewritten query variant
#[derive(Debug, Clone)]
pub struct RewrittenQuery {
	/// the rewritten query text
	pub text: String,
	/// type of rewrite applied
	pub rewrite_type: RewriteType,
	/// confidence in this rewrite (0.0 - 1.0)
	pub confidence: f32,
}

/// Type of query rewriting applied
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RewriteType {
	/// Original query unchanged
	Original,
	/// Extracted likely symbol names
	SymbolExtraction,
	/// Query decomposed into sub-queries
	Decomposition,
	/// Conceptual terms mapped to technical terms
	ConceptMapping,
}

/// Query rewriter for improving conceptual query retrieval
pub struct QueryRewriter<'a> {
	/// daemon client for LLM operations
	daemon: &'a DaemonClient,
}

impl<'a> QueryRewriter<'a> {
	/// Create a new query rewriter
	pub fn new(daemon: &'a DaemonClient) -> Self {
		Self { daemon }
	}

	/// Rewrite a conceptual query into multiple search variants
	pub fn rewrite(&self, query: &str) -> RetrievalResult<Vec<RewrittenQuery>> {
		let mut variants = Vec::new();

		// 1. Add original query
		variants.push(RewrittenQuery {
			text: query.to_string(),
			rewrite_type: RewriteType::Original,
			confidence: 1.0,
		});

		// 2. Extract potential symbol names
		let symbols = self.extract_symbols(query);
		if !symbols.is_empty() {
			variants.push(RewrittenQuery {
				text: symbols.join(" "),
				rewrite_type: RewriteType::SymbolExtraction,
				confidence: 0.9,
			});
		}

		// 3. Map conceptual terms to technical terms
		let mapped = self.map_concepts(query);
		if mapped != query {
			variants.push(RewrittenQuery {
				text: mapped,
				rewrite_type: RewriteType::ConceptMapping,
				confidence: 0.85,
			});
		}

		// 4. Decompose if complex query
		let decomposed = self.decompose(query);
		for sub_query in decomposed {
			if sub_query != query {
				variants.push(RewrittenQuery {
					text: sub_query,
					rewrite_type: RewriteType::Decomposition,
					confidence: 0.8,
				});
			}
		}

		Ok(variants)
	}

	/// Extract potential symbol names from query
	fn extract_symbols(&self, query: &str) -> Vec<String> {
		let mut symbols = Vec::new();

		// Extract words that look like code identifiers
		for word in query.split_whitespace() {
			let clean = word.trim_matches(|c: char| !c.is_alphanumeric() && c != '_');

			// CamelCase: AuthService, RetrievalPipeline
			if self.is_camel_case(clean) {
				symbols.push(clean.to_string());
			}
			// snake_case: parse_config, build_index
			else if self.is_snake_case(clean) {
				symbols.push(clean.to_string());
			}
			// SCREAMING_CASE: MAX_TOKENS
			else if self.is_screaming_case(clean) {
				symbols.push(clean.to_string());
			}
		}

		symbols
	}

	/// Check if word is CamelCase
	fn is_camel_case(&self, word: &str) -> bool {
		if word.len() < 2 {
			return false;
		}
		let chars: Vec<char> = word.chars().collect();
		chars[0].is_uppercase()
			&& chars.iter().any(|c| c.is_lowercase())
			&& chars.iter().filter(|c| c.is_uppercase()).count() >= 2
	}

	/// Check if word is snake_case
	fn is_snake_case(&self, word: &str) -> bool {
		word.contains('_')
			&& word.chars().all(|c| c.is_lowercase() || c.is_numeric() || c == '_')
			&& word.len() >= 3
	}

	/// Check if word is SCREAMING_CASE
	fn is_screaming_case(&self, word: &str) -> bool {
		word.contains('_')
			&& word.chars().all(|c| c.is_uppercase() || c.is_numeric() || c == '_')
			&& word.len() >= 3
	}

	/// Map conceptual terms to technical/code terms
	fn map_concepts(&self, query: &str) -> String {
		let lower = query.to_lowercase();
		let mut result = query.to_string();

		// Common conceptual -> technical mappings
		let mappings = [
			("how does", "implementation"),
			("how do", "implementation"),
			("what is", "definition"),
			("where is", "location"),
			("authentication", "auth"),
			("authorization", "auth"),
			("database", "db"),
			("configuration", "config"),
			("initialization", "init"),
			("retrieval", "retrieve search"),
			("pipeline", "Pipeline flow"),
			("works", "implementation"),
			("work", "implementation"),
		];

		for (concept, technical) in mappings {
			if lower.contains(concept) {
				// Add technical term to query
				if !result.to_lowercase().contains(technical) {
					result.push(' ');
					result.push_str(technical);
				}
			}
		}

		result
	}

	/// Decompose complex query into simpler sub-queries
	fn decompose(&self, query: &str) -> Vec<String> {
		let mut sub_queries = Vec::new();
		let lower = query.to_lowercase();

		// Split on "and", "with", "using"
		let connectors = ["and", "with", "using", "via", "through"];

		for connector in connectors {
			if lower.contains(&format!(" {} ", connector)) {
				let parts: Vec<&str> = query.split(&format!(" {} ", connector)).collect();
				if parts.len() == 2 {
					sub_queries.push(parts[0].trim().to_string());
					sub_queries.push(parts[1].trim().to_string());
				}
			}
		}

		// If "how does X work", extract X as key concept
		if lower.starts_with("how does ") && lower.ends_with(" work") {
			let mid = &query[9..query.len() - 5].trim();
			if !mid.is_empty() {
				sub_queries.push(mid.to_string());
				sub_queries.push(format!("{} implementation", mid));
			}
		}

		// If "the X Y" pattern, extract key terms
		if lower.contains("the ") {
			let after_the = lower.split("the ").nth(1);
			if let Some(rest) = after_the {
				let key_words: Vec<&str> = rest
					.split_whitespace()
					.filter(|w| !STOP_WORDS.contains(w))
					.take(3)
					.collect();
				if !key_words.is_empty() {
					sub_queries.push(key_words.join(" "));
				}
			}
		}

		sub_queries
	}
}

/// Common stop words to filter out
const STOP_WORDS: &[&str] = &[
	"how", "does", "do", "the", "a", "an", "is", "are", "was", "were",
	"what", "where", "when", "why", "which", "who",
	"work", "works", "working",
];

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test_is_camel_case() {
		let rewriter = QueryRewriter { daemon: &DaemonClient::new() };
		assert!(rewriter.is_camel_case("AuthService"));
		assert!(rewriter.is_camel_case("RetrievalPipeline"));
		assert!(!rewriter.is_camel_case("auth"));
		assert!(!rewriter.is_camel_case("AUTH"));
	}

	#[test]
	fn test_is_snake_case() {
		let rewriter = QueryRewriter { daemon: &DaemonClient::new() };
		assert!(rewriter.is_snake_case("parse_config"));
		assert!(rewriter.is_snake_case("build_index"));
		assert!(!rewriter.is_snake_case("parseConfig"));
		assert!(!rewriter.is_snake_case("BUILD_INDEX"));
	}

	#[test]
	fn test_extract_symbols() {
		let rewriter = QueryRewriter { daemon: &DaemonClient::new() };
		let symbols = rewriter.extract_symbols("how does RetrievalPipeline work");
		assert!(symbols.contains(&"RetrievalPipeline".to_string()));
	}

	#[test]
	fn test_map_concepts() {
		let rewriter = QueryRewriter { daemon: &DaemonClient::new() };
		let mapped = rewriter.map_concepts("how does retrieval work");
		assert!(mapped.contains("implementation"));
		assert!(mapped.contains("retrieve search"));
	}

	#[test]
	fn test_decompose() {
		let rewriter = QueryRewriter { daemon: &DaemonClient::new() };
		let decomposed = rewriter.decompose("how does the retrieval pipeline work");
		assert!(!decomposed.is_empty());
	}
}
