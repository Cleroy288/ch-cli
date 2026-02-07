//! Default implementation of SearchService.

use std::path::Path;

use crate::domain::errors::search::SearchError;
use crate::indexer::{SearchIndex, Symbol};

use super::types::{
	CallerHit, SearchOptions, SearchResult,
	SymbolDetails,
};
use super::types_navigation::{
	DefinitionHit, ReferenceResult,
	StructureResult, SymbolEntry,
	SymbolListOptions,
};
use super::{
	navigation_impl, search_impl, SearchService,
};

/// Default search service backed by SearchIndex
pub struct DefaultSearchService;

impl DefaultSearchService {
	/// Create a new default search service
	pub fn new() -> Self {
		Self
	}
}

impl SearchService for DefaultSearchService {
	fn search(
		&self,
		query: &str,
		path: &Path,
		opts: &SearchOptions,
	) -> Result<SearchResult, SearchError> {
		search_impl::execute_search(
			query, path, opts,
		)
	}

	fn search_callers(
		&self,
		_symbol: &str,
		_path: &Path,
	) -> Result<Vec<CallerHit>, SearchError> {
		Ok(vec![])
	}

	fn get_symbol_info(
		&self,
		_symbol: &str,
		_path: &Path,
	) -> Result<Option<SymbolDetails>, SearchError>
	{
		Ok(None)
	}

	fn index_symbols(
		&self,
		symbols: &[Symbol],
	) -> Result<usize, SearchError> {
		let index = SearchIndex::in_memory()?;
		index.index_symbols(symbols)
	}

	fn find_definition(
		&self,
		symbol: &str,
		path: &Path,
	) -> Result<Vec<DefinitionHit>, SearchError> {
		navigation_impl::find_definition(
			symbol, path,
		)
	}

	fn find_references(
		&self,
		symbol: &str,
		path: &Path,
		include_def: bool,
	) -> Result<ReferenceResult, SearchError> {
		navigation_impl::find_references(
			symbol, path, include_def,
		)
	}

	fn list_symbols(
		&self,
		path: &Path,
		opts: &SymbolListOptions,
	) -> Result<Vec<SymbolEntry>, SearchError> {
		navigation_impl::list_symbols(path, opts)
	}

	fn find_structure(
		&self,
		target: &str,
		path: &Path,
	) -> Result<StructureResult, SearchError> {
		navigation_impl::find_structure(
			target, path,
		)
	}
}
