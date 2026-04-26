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
	navigation_impl, navigation_structure,
	search_impl, symbol_info, SearchService,
};

use super::default::DefaultSearchService;

impl SearchService for DefaultSearchService {
	fn search(
		&self,
		query: &str,
		path: &Path,
		opts: &SearchOptions,
	) -> Result<SearchResult, SearchError> {
		self.cache().with_index(
			path,
			|result| {
				search_impl::execute_search(
					query, opts, result,
				)
			},
		)
	}

	fn search_callers(
		&self,
		symbol: &str,
		path: &Path,
	) -> Result<Vec<CallerHit>, SearchError> {
		self.cache().with_index(
			path,
			|result| {
				symbol_info::search_callers(
					symbol, result,
				)
			},
		)
	}

	fn get_symbol_info(
		&self,
		symbol: &str,
		path: &Path,
	) -> Result<Option<SymbolDetails>, SearchError>
	{
		self.cache().with_index(
			path,
			|result| {
				symbol_info::get_symbol_info(
					symbol, result,
				)
			},
		)
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
		self.cache().with_index(
			path,
			|result| {
				navigation_impl::find_definition(
					symbol, result,
				)
			},
		)
	}

	fn find_references(
		&self,
		symbol: &str,
		path: &Path,
		include_def: bool,
	) -> Result<ReferenceResult, SearchError> {
		self.cache().with_index(
			path,
			|result| {
				navigation_impl::find_references(
					symbol, result, include_def,
				)
			},
		)
	}

	fn list_symbols(
		&self,
		path: &Path,
		opts: &SymbolListOptions,
	) -> Result<Vec<SymbolEntry>, SearchError> {
		self.cache().with_index(
			path,
			|result| {
				navigation_structure::list_symbols(
					result, opts,
				)
			},
		)
	}

	fn find_structure(
		&self,
		target: &str,
		path: &Path,
	) -> Result<StructureResult, SearchError> {
		navigation_structure::find_structure(
			target, path,
		)
	}
}
