//! Adaptive Weights for Hybrid Search
//!
//! Dynamically adjusts keyword/semantic weights based on query type.
//! Symbol-focused queries favor BM25, conceptual queries favor semantic.

use crate::retrieval::query::{FastPathIntent, FastPathParser};

/// Weight configuration for hybrid search
#[derive(Debug, Clone, Copy)]
pub struct AdaptiveWeights {
	/// weight for keyword (BM25) search
	pub keyword_weight: f32,
	/// weight for semantic search
	pub semantic_weight: f32,
}

impl Default for AdaptiveWeights {
	fn default() -> Self {
		Self {
			keyword_weight: 1.0,
			semantic_weight: 1.0,
		}
	}
}

/// Query type classification for weight selection
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum QueryType {
	/// Symbol lookup (e.g., "find AuthService")
	SymbolLookup,
	/// Conceptual query (e.g., "how does auth work")
	Conceptual,
	/// Mixed - has both symbols and concepts
	Mixed,
	/// Definition lookup (e.g., "where is X defined")
	Definition,
	/// Usage lookup (e.g., "where is X used")
	Usage,
}

/// Compute adaptive weights based on query characteristics.
/// Returns weights that favor BM25 for symbol queries,
/// semantic for conceptual queries.
pub fn compute_weights(query: &str) -> AdaptiveWeights {
	let query_type = classify_query(query);
	weights_for_type(query_type)
}

/// Classify query into a type
pub fn classify_query(query: &str) -> QueryType {
	let parser = FastPathParser::new();
	let result = parser.extract_symbols(query);

	// use fast path intent as primary signal
	match result.intent {
		FastPathIntent::Explicit => {
			// check for definition/usage keywords
			let lower = query.to_lowercase();
			if lower.contains("where") && lower.contains("defined") {
				QueryType::Definition
			} else if lower.contains("used")
				|| lower.contains("usage")
				|| lower.contains("calls")
			{
				QueryType::Usage
			} else {
				QueryType::SymbolLookup
			}
		}
		FastPathIntent::Conceptual => QueryType::Conceptual,
		FastPathIntent::Mixed => QueryType::Mixed,
	}
}

/// Get weights for a query type
pub fn weights_for_type(query_type: QueryType) -> AdaptiveWeights {
	match query_type {
		// symbol lookups: favor keyword matching (70/30)
		QueryType::SymbolLookup => AdaptiveWeights {
			keyword_weight: 1.4,
			semantic_weight: 0.6,
		},
		// definition lookups: strongly favor keyword (80/20)
		QueryType::Definition => AdaptiveWeights {
			keyword_weight: 1.6,
			semantic_weight: 0.4,
		},
		// usage lookups: balanced with slight keyword bias (60/40)
		QueryType::Usage => AdaptiveWeights {
			keyword_weight: 1.2,
			semantic_weight: 0.8,
		},
		// conceptual: favor semantic (30/70)
		QueryType::Conceptual => AdaptiveWeights {
			keyword_weight: 0.6,
			semantic_weight: 1.4,
		},
		// mixed: balanced (50/50)
		QueryType::Mixed => AdaptiveWeights {
			keyword_weight: 1.0,
			semantic_weight: 1.0,
		},
	}
}

