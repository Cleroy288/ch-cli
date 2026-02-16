//! Mock implementation of SearchService — struct
//! definition and preset/accessor methods.
//!
//! The trait impl is in mock_search_trait.rs.

use std::cell::RefCell;
use std::collections::VecDeque;
use std::path::PathBuf;

use rustean::domain::errors::search::SearchError;
use rustean::service::search::types::{
	CallerHit, SearchResult, SymbolDetails,
};
use rustean::service::search::types_navigation::{
	DefinitionHit, ReferenceResult,
	StructureResult, SymbolEntry,
};

/// Mock SearchService for testing
pub struct MockSearchService {
	/// preset returns for search
	pub(super) search_returns: RefCell<
		VecDeque<Result<SearchResult, SearchError>>,
	>,
	/// preset returns for search_callers
	pub(super) callers_returns: RefCell<
		VecDeque<
			Result<Vec<CallerHit>, SearchError>,
		>,
	>,
	/// preset returns for get_symbol_info
	pub(super) info_returns: RefCell<
		VecDeque<
			Result<
				Option<SymbolDetails>,
				SearchError,
			>,
		>,
	>,
	/// preset returns for index_symbols
	pub(super) index_sym_returns:
		RefCell<VecDeque<Result<usize, SearchError>>>,
	/// preset returns for find_definition
	pub(super) def_returns: RefCell<
		VecDeque<
			Result<
				Vec<DefinitionHit>,
				SearchError,
			>,
		>,
	>,
	/// preset returns for find_references
	pub(super) refs_returns: RefCell<
		VecDeque<
			Result<ReferenceResult, SearchError>,
		>,
	>,
	/// preset returns for list_symbols
	pub(super) list_returns: RefCell<
		VecDeque<
			Result<Vec<SymbolEntry>, SearchError>,
		>,
	>,
	/// preset returns for find_structure
	pub(super) structure_returns: RefCell<
		VecDeque<
			Result<StructureResult, SearchError>,
		>,
	>,
	/// recorded search calls (query, path)
	pub search_calls:
		RefCell<Vec<(String, PathBuf)>>,
	/// recorded find_definition calls
	pub def_calls:
		RefCell<Vec<(String, PathBuf)>>,
}

impl MockSearchService {
	/// Create an empty mock
	pub fn new() -> Self {
		Self {
			search_returns: RefCell::new(
				VecDeque::new(),
			),
			callers_returns: RefCell::new(
				VecDeque::new(),
			),
			info_returns: RefCell::new(
				VecDeque::new(),
			),
			index_sym_returns: RefCell::new(
				VecDeque::new(),
			),
			def_returns: RefCell::new(
				VecDeque::new(),
			),
			refs_returns: RefCell::new(
				VecDeque::new(),
			),
			list_returns: RefCell::new(
				VecDeque::new(),
			),
			structure_returns: RefCell::new(
				VecDeque::new(),
			),
			search_calls: RefCell::new(Vec::new()),
			def_calls: RefCell::new(Vec::new()),
		}
	}

	/// Preset a return for search
	pub fn on_search(
		&self,
		r: Result<SearchResult, SearchError>,
	) {
		self.search_returns
			.borrow_mut()
			.push_back(r);
	}

	/// Preset a return for find_definition
	pub fn on_definition(
		&self,
		r: Result<Vec<DefinitionHit>, SearchError>,
	) {
		self.def_returns
			.borrow_mut()
			.push_back(r);
	}

	/// Preset a return for find_references
	pub fn on_references(
		&self,
		r: Result<ReferenceResult, SearchError>,
	) {
		self.refs_returns
			.borrow_mut()
			.push_back(r);
	}

	/// Preset a return for list_symbols
	pub fn on_list(
		&self,
		r: Result<Vec<SymbolEntry>, SearchError>,
	) {
		self.list_returns
			.borrow_mut()
			.push_back(r);
	}
}
