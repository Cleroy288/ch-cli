//! Mock implementation of IndexService.
//!
//! Records calls and returns preset values for
//! testing without real filesystem indexing.

use std::cell::RefCell;
use std::collections::VecDeque;
use std::path::{Path, PathBuf};

use rustean::domain::errors::index::IndexError;
use rustean::indexer::{IndexResult, IndexStats};
use rustean::service::index::types::IndexOptions;
use rustean::service::IndexService;

/// Mock IndexService for testing
pub struct MockIndexService {
	/// preset returns for index_project
	index_returns: RefCell<
		VecDeque<Result<IndexResult, IndexError>>,
	>,
	/// preset returns for get_stats
	stats_returns: RefCell<
		VecDeque<
			Result<Option<IndexStats>, IndexError>,
		>,
	>,
	/// preset returns for has_index
	has_index_returns: RefCell<VecDeque<bool>>,
	/// preset returns for clear_index
	clear_returns:
		RefCell<VecDeque<Result<(), IndexError>>>,
	/// recorded index_project calls
	pub index_calls:
		RefCell<Vec<(PathBuf, IndexOptions)>>,
	/// recorded has_index calls
	pub has_index_calls: RefCell<Vec<PathBuf>>,
}

impl MockIndexService {
	/// Create an empty mock
	pub fn new() -> Self {
		Self {
			index_returns: RefCell::new(
				VecDeque::new(),
			),
			stats_returns: RefCell::new(
				VecDeque::new(),
			),
			has_index_returns: RefCell::new(
				VecDeque::new(),
			),
			clear_returns: RefCell::new(
				VecDeque::new(),
			),
			index_calls: RefCell::new(Vec::new()),
			has_index_calls: RefCell::new(Vec::new()),
		}
	}

	/// Preset a return for index_project
	pub fn on_index(
		&self,
		r: Result<IndexResult, IndexError>,
	) {
		self.index_returns.borrow_mut().push_back(r);
	}

	/// Preset a return for has_index
	pub fn on_has_index(&self, val: bool) {
		self.has_index_returns
			.borrow_mut()
			.push_back(val);
	}

	/// Preset a return for get_stats
	pub fn on_stats(
		&self,
		r: Result<Option<IndexStats>, IndexError>,
	) {
		self.stats_returns.borrow_mut().push_back(r);
	}

	/// Preset a return for clear_index
	pub fn on_clear(
		&self,
		r: Result<(), IndexError>,
	) {
		self.clear_returns.borrow_mut().push_back(r);
	}
}

impl IndexService for MockIndexService {
	fn index_project(
		&self,
		path: &Path,
		opts: &IndexOptions,
	) -> Result<IndexResult, IndexError> {
		self.index_calls.borrow_mut().push((
			path.to_path_buf(),
			opts.clone(),
		));
		self.index_returns
			.borrow_mut()
			.pop_front()
			.unwrap_or_else(|| {
				Err(IndexError::Parser(
					"no preset".into(),
				))
			})
	}

	fn get_stats(
		&self,
		path: &Path,
	) -> Result<Option<IndexStats>, IndexError> {
		let _ = path;
		self.stats_returns
			.borrow_mut()
			.pop_front()
			.unwrap_or(Ok(None))
	}

	fn has_index(&self, path: &Path) -> bool {
		self.has_index_calls
			.borrow_mut()
			.push(path.to_path_buf());
		self.has_index_returns
			.borrow_mut()
			.pop_front()
			.unwrap_or(false)
	}

	fn clear_index(
		&self,
		path: &Path,
	) -> Result<(), IndexError> {
		let _ = path;
		self.clear_returns
			.borrow_mut()
			.pop_front()
			.unwrap_or(Ok(()))
	}
}
