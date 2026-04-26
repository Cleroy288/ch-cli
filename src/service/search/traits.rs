use std::path::Path;

use crate::domain::errors::search::SearchError;
use crate::indexer::Symbol;

use super::types::{
	CallerHit, SearchOptions, SearchResult,
	SymbolDetails,
};
use super::types_navigation::{
	DefinitionHit, ReferenceResult,
	StructureResult, SymbolEntry,
	SymbolListOptions,
};

/// Service trait for symbol search operations
pub trait SearchService {
	fn search(
		&self,
		query: &str,
		path: &Path,
		opts: &SearchOptions,
	) -> Result<SearchResult, SearchError>;

	fn search_callers(
		&self,
		symbol: &str,
		path: &Path,
	) -> Result<Vec<CallerHit>, SearchError>;

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
