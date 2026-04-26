use std::path::PathBuf;

use crate::indexer::{
	CodeLocation, Symbol, SymbolKind,
};

use super::boost::QueryIntent;

/// Boolean flags for search operations
#[derive(Debug, Clone, Default)]
pub struct SearchFlags {
	pub fuzzy: bool,
	pub full: bool,
}

/// Options for search operations
#[derive(Debug, Clone, Default)]
pub struct SearchOptions {
	pub limit: usize,
	pub kind: Option<SymbolKind>,
	pub flags: SearchFlags,
}

/// Result from a search operation
#[derive(Debug, Clone)]
pub struct SearchResult {
	pub hits: Vec<SearchResultHit>,
	pub intent: QueryIntent,
}

/// Single search result with boosted score
#[derive(Debug, Clone)]
pub struct SearchResultHit {
	pub symbol: Symbol,
	pub score: f64,
}

/// A call-site reference to a symbol
#[derive(Debug, Clone)]
pub struct CallerHit {
	pub file: PathBuf,
	pub line: usize,
	pub context: String,
	pub caller_name: Option<String>,
}

/// Full symbol details with callers and refs
#[derive(Debug, Clone)]
pub struct SymbolDetails {
	pub symbol: Symbol,
	pub source_code: Option<String>,
	pub callers: Vec<CallerHit>,
	pub callees: Vec<CalleeInfo>,
	pub references: Vec<CodeLocation>,
}

/// A function called by a symbol
#[derive(Debug, Clone)]
pub struct CalleeInfo {
	pub name: String,
	pub line: usize,
}
