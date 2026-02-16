//! SearchService trait impl for MockSearchService.
//!
//! Separated from mock_search.rs to keep file
//! size within norms.

use std::path::Path;

use rustean::domain::errors::search::SearchError;
use rustean::indexer::Symbol;
use rustean::service::search::types::{
	CallerHit, SearchOptions, SearchResult,
	SymbolDetails,
};
use rustean::service::search::types_navigation::{
	DefinitionHit, ReferenceResult,
	StructureResult, SymbolEntry,
	SymbolListOptions,
};
use rustean::service::SearchService;

use super::mock_search::MockSearchService;

impl SearchService for MockSearchService {
	fn search(
		&self,
		query: &str,
		path: &Path,
		_opts: &SearchOptions,
	) -> Result<SearchResult, SearchError> {
		self.search_calls.borrow_mut().push((
			query.to_string(),
			path.to_path_buf(),
		));
		self.search_returns
			.borrow_mut()
			.pop_front()
			.unwrap_or_else(|| {
				Err(SearchError::FieldNotFound(
					"no preset".into(),
				))
			})
	}

	fn search_callers(
		&self,
		_symbol: &str,
		_path: &Path,
	) -> Result<Vec<CallerHit>, SearchError> {
		self.callers_returns
			.borrow_mut()
			.pop_front()
			.unwrap_or(Ok(vec![]))
	}

	fn get_symbol_info(
		&self,
		_symbol: &str,
		_path: &Path,
	) -> Result<Option<SymbolDetails>, SearchError>
	{
		self.info_returns
			.borrow_mut()
			.pop_front()
			.unwrap_or(Ok(None))
	}

	fn index_symbols(
		&self,
		_symbols: &[Symbol],
	) -> Result<usize, SearchError> {
		self.index_sym_returns
			.borrow_mut()
			.pop_front()
			.unwrap_or(Ok(0))
	}

	fn find_definition(
		&self,
		symbol: &str,
		path: &Path,
	) -> Result<Vec<DefinitionHit>, SearchError> {
		self.def_calls.borrow_mut().push((
			symbol.to_string(),
			path.to_path_buf(),
		));
		self.def_returns
			.borrow_mut()
			.pop_front()
			.unwrap_or(Ok(vec![]))
	}

	fn find_references(
		&self,
		_symbol: &str,
		_path: &Path,
		_include_def: bool,
	) -> Result<ReferenceResult, SearchError> {
		self.refs_returns
			.borrow_mut()
			.pop_front()
			.unwrap_or(Ok(ReferenceResult {
				definitions: vec![],
				references: vec![],
			}))
	}

	fn list_symbols(
		&self,
		_path: &Path,
		_opts: &SymbolListOptions,
	) -> Result<Vec<SymbolEntry>, SearchError> {
		self.list_returns
			.borrow_mut()
			.pop_front()
			.unwrap_or(Ok(vec![]))
	}

	fn find_structure(
		&self,
		_target: &str,
		_path: &Path,
	) -> Result<StructureResult, SearchError> {
		self.structure_returns
			.borrow_mut()
			.pop_front()
			.unwrap_or(Ok(StructureResult {
				target: String::new(),
				modules: vec![],
				available_dirs: vec![],
			}))
	}
}
