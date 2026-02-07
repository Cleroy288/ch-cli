//! DTOs for the search service.

use std::path::PathBuf;

use crate::indexer::{
	CodeLocation, Symbol, SymbolKind,
};
use crate::retrieval::daemon::protocol::QueryIntent;

/// Options for search operations
#[derive(Debug, Clone, Default)]
pub struct SearchOptions {
	/// Maximum results to return
	pub limit: usize,
	/// Use fuzzy matching
	pub fuzzy: bool,
	/// Filter by symbol kind
	pub kind: Option<SymbolKind>,
	/// Enable semantic (hybrid) search
	pub semantic: bool,
	/// Expand with context info
	pub context: bool,
	/// Rerank results
	pub rerank: bool,
	/// Show full file content
	pub full: bool,
}

/// Result from a search operation
#[derive(Debug, Clone)]
pub struct SearchResult {
	/// matched hits (sorted by relevance)
	pub hits: Vec<SearchResultHit>,
	/// detected query intent
	pub intent: QueryIntent,
	/// context XML if requested
	pub context_xml: Option<String>,
}

/// A single search result hit
#[derive(Debug, Clone)]
pub struct SearchResultHit {
	/// the matched symbol
	pub symbol: Symbol,
	/// relevance score
	pub score: f64,
	/// keyword rank (for hybrid)
	pub keyword_rank: Option<usize>,
	/// semantic rank (for hybrid)
	pub semantic_rank: Option<usize>,
	/// rerank score (if reranked)
	pub rerank_score: Option<f64>,
}

/// A caller hit from reference search
#[derive(Debug, Clone)]
pub struct CallerHit {
	/// File containing the call
	pub file: PathBuf,
	/// Line of the call
	pub line: usize,
	/// Context snippet
	pub context: String,
	/// Containing function name
	pub caller_name: Option<String>,
}

/// Detailed symbol information
#[derive(Debug, Clone)]
pub struct SymbolDetails {
	/// The symbol definition
	pub symbol: Symbol,
	/// Source code snippet
	pub source_code: Option<String>,
	/// Callers of this symbol
	pub callers: Vec<CallerHit>,
	/// Functions this symbol calls
	pub callees: Vec<CalleeInfo>,
	/// All reference locations
	pub references: Vec<CodeLocation>,
}

/// Info about a function called by a symbol
#[derive(Debug, Clone)]
pub struct CalleeInfo {
	/// Name of the called function
	pub name: String,
	/// Line where the call is made
	pub line: usize,
}
