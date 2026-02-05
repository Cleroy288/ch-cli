//! Tiered Query Expander
//!
//! Implements 3-tier decision logic for query expansion:
//! 1. FastPathParser extracts symbols via regex
//! 2. SymbolValidator validates against SemanticGraph
//! 3. LLM fallback for conceptual queries

use std::time::Instant;

use crate::indexer::SemanticGraph;
use crate::retrieval::daemon::protocol::{QueryIntent, SearchSpec};
use crate::retrieval::daemon::DaemonClient;
use crate::retrieval::RetrievalResult;

use super::fast_path::{FastPathIntent, FastPathParser};
use super::rewriter::QueryRewriter;
use super::validator::SymbolValidator;

/// Which tier was used for query expansion
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TierUsed {
	/// Fast path with validated symbols (no LLM)
	FastPathValidated,
	/// Fast path without validation (no graph available)
	FastPathUnvalidated,
	/// LLM expansion (Phi-3)
	LlmExpansion,
}

/// Configuration for tiered expansion
#[derive(Debug, Clone)]
pub struct TieredConfig {
	/// minimum confidence to use fast-path (0.0 - 1.0)
	pub confidence_threshold: f32,
	/// minimum importance for validated symbols (0.0 - 1.0)
	pub importance_threshold: f32,
	/// minimum existence ratio for fast-path (0.0 - 1.0)
	pub existence_threshold: f32,
}

impl Default for TieredConfig {
	fn default() -> Self {
		Self {
			confidence_threshold: 0.7,
			importance_threshold: 0.5,
			existence_threshold: 0.5,
		}
	}
}

/// Result of tiered expansion
#[derive(Debug, Clone)]
pub struct TieredResult {
	/// the search specification
	pub spec: SearchSpec,
	/// which tier was used
	pub tier_used: TierUsed,
	/// time taken in milliseconds
	pub time_ms: u64,
}

/// Tiered query expander with fast-path optimization
pub struct TieredQueryExpander<'a> {
	/// daemon client for LLM expansion
	daemon: &'a DaemonClient,
	/// semantic graph for validation (optional)
	graph: Option<&'a SemanticGraph>,
	/// fast-path parser
	parser: FastPathParser,
	/// configuration
	config: TieredConfig,
}

impl<'a> TieredQueryExpander<'a> {
	/// Create a new tiered expander
	pub fn new(daemon: &'a DaemonClient, graph: Option<&'a SemanticGraph>) -> Self {
		Self {
			daemon,
			graph,
			parser: FastPathParser::new(),
			config: TieredConfig::default(),
		}
	}

	/// Create with custom configuration
	pub fn with_config(
		daemon: &'a DaemonClient,
		graph: Option<&'a SemanticGraph>,
		config: TieredConfig,
	) -> Self {
		Self {
			daemon,
			graph,
			parser: FastPathParser::new(),
			config,
		}
	}

	/// Expand query using tiered approach
	pub fn expand(&self, query: &str) -> RetrievalResult<TieredResult> {
		let start = Instant::now(); // track timing

		// Tier 1: Fast-path parsing
		let fast_result = self.parser.extract_symbols(query);

		// If conceptual query, go straight to LLM
		if fast_result.intent == FastPathIntent::Conceptual {
			return self.expand_with_llm(query, start);
		}

		// If no symbols extracted, use LLM
		if fast_result.symbols.is_empty() {
			return self.expand_with_llm(query, start);
		}

		// If confidence too low, use LLM
		if fast_result.confidence < self.config.confidence_threshold {
			return self.expand_with_llm(query, start);
		}

		// Mixed intent (conceptual + symbols) -> use LLM
		if fast_result.intent == FastPathIntent::Mixed {
			return self.expand_with_llm(query, start);
		}

		// Tier 2: Validate symbols if graph is available
		let symbol_names: Vec<String> = fast_result.symbols.iter().map(|s| s.name.clone()).collect();

		if let Some(graph) = self.graph {
			let validator = SymbolValidator::new(graph);
			let validation = validator.validate_symbols(&symbol_names);

			// Check existence threshold
			if validation.existence_ratio < self.config.existence_threshold {
				return self.expand_with_llm(query, start);
			}

			// Check importance threshold
			if validation.avg_importance < self.config.importance_threshold {
				return self.expand_with_llm(query, start);
			}

			// Use fast-path with validated symbols
			let validated_names: Vec<String> = validation
				.symbols
				.iter()
				.filter(|s| s.exists)
				.map(|s| s.name.clone())
				.collect();

			let spec = self.build_fast_path_spec(query, validated_names);
			let time_ms = start.elapsed().as_millis() as u64;

			return Ok(TieredResult {
				spec,
				tier_used: TierUsed::FastPathValidated,
				time_ms,
			});
		}

		// No graph available: use fast-path without validation
		let spec = self.build_fast_path_spec(query, symbol_names);
		let time_ms = start.elapsed().as_millis() as u64;

		Ok(TieredResult {
			spec,
			tier_used: TierUsed::FastPathUnvalidated,
			time_ms,
		})
	}

	/// Build SearchSpec from fast-path symbols
	fn build_fast_path_spec(&self, query: &str, symbols: Vec<String>) -> SearchSpec {
		// Detect intent from query keywords
		let intent = self.detect_intent(query);

		SearchSpec {
			original_query: query.to_string(),
			symbol_names: symbols,
			intent,
			file_filters: Vec::new(),
			context_hints: Vec::new(),
		}
	}

	/// Detect query intent from keywords
	fn detect_intent(&self, query: &str) -> QueryIntent {
		let lower = query.to_lowercase();

		if lower.contains("where") || lower.contains("definition") || lower.contains("defined") {
			QueryIntent::FindDefinition
		} else if lower.contains("used") || lower.contains("calls") || lower.contains("references")
		{
			QueryIntent::FindUsages
		} else if lower.contains("how") || lower.contains("what") || lower.contains("explain") {
			QueryIntent::Understand
		} else {
			QueryIntent::Search
		}
	}

	/// Expand using LLM (Phi-3) with query rewriting enhancement
	fn expand_with_llm(&self, query: &str, start: Instant) -> RetrievalResult<TieredResult> {
		// First, use query rewriter to generate enhanced search terms
		let rewriter = QueryRewriter::new(self.daemon);
		let rewrites = rewriter.rewrite(query)?;

		// Collect all symbol candidates from rewrites
		let mut all_symbols: Vec<String> = Vec::new();
		for rewrite in &rewrites {
			// Extract symbols from each rewritten query
			let fast_result = self.parser.extract_symbols(&rewrite.text);
			for sym in fast_result.symbols {
				if !all_symbols.contains(&sym.name) {
					all_symbols.push(sym.name);
				}
			}
		}

		// If rewriting found symbols, use them directly (skip LLM)
		if !all_symbols.is_empty() {
			let spec = SearchSpec {
				original_query: query.to_string(),
				symbol_names: all_symbols,
				intent: self.detect_intent(query),
				file_filters: Vec::new(),
				context_hints: rewrites.iter().map(|r| r.text.clone()).collect(),
			};
			let time_ms = start.elapsed().as_millis() as u64;

			return Ok(TieredResult {
				spec,
				tier_used: TierUsed::LlmExpansion,
				time_ms,
			});
		}

		// Fallback: use LLM for expansion
		let spec = self.daemon.expand(query.to_string())?;
		let time_ms = start.elapsed().as_millis() as u64;

		Ok(TieredResult {
			spec,
			tier_used: TierUsed::LlmExpansion,
			time_ms,
		})
	}
}

#[cfg(test)]
mod tests {
	use super::*;
	use crate::indexer::{CodeLocation, Symbol, SymbolKind, Visibility};
	use std::path::PathBuf;

	fn create_test_graph() -> SemanticGraph {
		let mut graph = SemanticGraph::new();

		let auth = Symbol::new(
			"AuthService".to_string(),
			SymbolKind::Struct,
			CodeLocation::new(PathBuf::from("src/auth.rs"), 10, 1, 0, 11),
		)
		.with_visibility(Visibility::Public);
		graph.add_definition(auth);

		// Add some references
		for i in 0..5 {
			graph.add_reference(crate::indexer::SymbolReference {
				name: "AuthService".to_string(),
				location: CodeLocation::new(PathBuf::from("src/main.rs"), 20 + i, 1, 0, 11),
				context: crate::indexer::ReferenceContext::Type,
			});
		}

		graph
	}

	#[test]
	fn test_fast_path_parser_extracts_symbols() {
		let parser = FastPathParser::new();
		let result = parser.extract_symbols("find AuthService");

		assert!(!result.symbols.is_empty());
		assert_eq!(result.symbols[0].name, "AuthService");
		assert!(result.use_fast_path);
	}

	#[test]
	fn test_validation_with_graph() {
		let graph = create_test_graph();
		let validator = SymbolValidator::new(&graph);

		// Validate existing symbol
		let result = validator.validate_symbol("AuthService");
		assert!(result.exists);
		assert!(result.importance > 0.0);

		// Validate non-existing symbol
		let result = validator.validate_symbol("NonExistent");
		assert!(!result.exists);
	}

	#[test]
	fn test_conceptual_query_detection() {
		let parser = FastPathParser::new();
		let result = parser.extract_symbols("why does the daemon fail");

		assert_eq!(result.intent, FastPathIntent::Conceptual);
		assert!(!result.use_fast_path);
	}

	#[test]
	fn test_config_default() {
		let config = TieredConfig::default();
		assert!((config.confidence_threshold - 0.7).abs() < 0.01);
		assert!((config.importance_threshold - 0.5).abs() < 0.01);
	}

	#[test]
	fn test_detect_intent() {
		// Create a minimal test without daemon
		let parser = FastPathParser::new();

		let explicit = parser.extract_symbols("where is AuthService defined");
		assert_eq!(explicit.intent, FastPathIntent::Explicit);

		let conceptual = parser.extract_symbols("explain how authentication works");
		assert_eq!(conceptual.intent, FastPathIntent::Conceptual);
	}
}
