//! Search service — symbol search use cases.

#[doc(hidden)]
pub mod cache;
#[doc(hidden)]
pub mod cache_helpers;
mod default;
mod navigation_impl;
mod search_impl;
mod semantic_impl;
mod semantic_rerank;
pub mod types;
pub mod types_navigation;

pub use default::DefaultSearchService;

use std::path::Path;

use crate::domain::errors::search::SearchError;
use crate::indexer::Symbol;

use types::{
	CallerHit, SearchOptions, SearchResult,
	SymbolDetails,
};
use types_navigation::{
	DefinitionHit, ReferenceResult,
	StructureResult, SymbolEntry,
	SymbolListOptions,
};

/// Service trait for symbol search operations
pub trait SearchService {
	/// Search for symbols matching a query
	fn search(
		&self,
		query: &str,
		path: &Path,
		opts: &SearchOptions,
	) -> Result<SearchResult, SearchError>;

	/// Search for callers of a symbol
	fn search_callers(
		&self,
		symbol: &str,
		path: &Path,
	) -> Result<Vec<CallerHit>, SearchError>;

	/// Get detailed info about a symbol
	fn get_symbol_info(
		&self,
		symbol: &str,
		path: &Path,
	) -> Result<Option<SymbolDetails>, SearchError>;

	/// Index symbols into the search engine
	fn index_symbols(
		&self,
		symbols: &[Symbol],
	) -> Result<usize, SearchError>;

	/// Find symbol definitions
	fn find_definition(
		&self,
		symbol: &str,
		path: &Path,
	) -> Result<Vec<DefinitionHit>, SearchError>;

	/// Find all references to a symbol
	fn find_references(
		&self,
		symbol: &str,
		path: &Path,
		include_def: bool,
	) -> Result<ReferenceResult, SearchError>;

	/// List symbols with optional filters
	fn list_symbols(
		&self,
		path: &Path,
		opts: &SymbolListOptions,
	) -> Result<Vec<SymbolEntry>, SearchError>;

	/// Find module structure for a target
	fn find_structure(
		&self,
		target: &str,
		path: &Path,
	) -> Result<StructureResult, SearchError>;
}
