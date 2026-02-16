//! Mock implementation of DocGenService.
//!
//! Records calls and returns preset values for
//! testing without real daemon or LLM.

use std::cell::RefCell;
use std::collections::VecDeque;
use std::path::{Path, PathBuf};

use rustean::domain::errors::docgen::DocGenError;
use rustean::retrieval::DocEntry;
use rustean::service::docgen::types::DocStatusInfo;
use rustean::service::DocGenService;

/// Mock DocGenService for testing
pub struct MockDocGenService {
	/// preset returns for start_generation
	start_returns:
		RefCell<VecDeque<Result<(), DocGenError>>>,
	/// preset returns for get_status
	status_returns: RefCell<
		VecDeque<Result<DocStatusInfo, DocGenError>>,
	>,
	/// preset returns for get_doc
	doc_returns: RefCell<
		VecDeque<
			Result<Option<DocEntry>, DocGenError>,
		>,
	>,
	/// preset returns for search_docs
	search_returns: RefCell<
		VecDeque<
			Result<Vec<DocEntry>, DocGenError>,
		>,
	>,
	/// recorded start_generation calls
	pub start_calls:
		RefCell<Vec<(PathBuf, bool)>>,
}

impl MockDocGenService {
	/// Create an empty mock
	pub fn new() -> Self {
		Self {
			start_returns: RefCell::new(
				VecDeque::new(),
			),
			status_returns: RefCell::new(
				VecDeque::new(),
			),
			doc_returns: RefCell::new(
				VecDeque::new(),
			),
			search_returns: RefCell::new(
				VecDeque::new(),
			),
			start_calls: RefCell::new(Vec::new()),
		}
	}

	/// Preset a return for start_generation
	pub fn on_start(
		&self,
		r: Result<(), DocGenError>,
	) {
		self.start_returns
			.borrow_mut()
			.push_back(r);
	}

	/// Preset a return for get_status
	pub fn on_status(
		&self,
		r: Result<DocStatusInfo, DocGenError>,
	) {
		self.status_returns
			.borrow_mut()
			.push_back(r);
	}

	/// Preset a return for get_doc
	pub fn on_doc(
		&self,
		r: Result<Option<DocEntry>, DocGenError>,
	) {
		self.doc_returns.borrow_mut().push_back(r);
	}

	/// Preset a return for search_docs
	pub fn on_search(
		&self,
		r: Result<Vec<DocEntry>, DocGenError>,
	) {
		self.search_returns
			.borrow_mut()
			.push_back(r);
	}
}

impl DocGenService for MockDocGenService {
	fn start_generation(
		&self,
		path: &Path,
		force: bool,
	) -> Result<(), DocGenError> {
		self.start_calls
			.borrow_mut()
			.push((path.to_path_buf(), force));
		self.start_returns
			.borrow_mut()
			.pop_front()
			.unwrap_or(Ok(()))
	}

	fn get_status(
		&self,
		_path: &Path,
	) -> Result<DocStatusInfo, DocGenError> {
		self.status_returns
			.borrow_mut()
			.pop_front()
			.unwrap_or_else(|| {
				Err(DocGenError::DaemonUnavailable(
					"no preset".into(),
				))
			})
	}

	fn get_doc(
		&self,
		_path: &Path,
		_symbol: &str,
	) -> Result<Option<DocEntry>, DocGenError> {
		self.doc_returns
			.borrow_mut()
			.pop_front()
			.unwrap_or(Ok(None))
	}

	fn search_docs(
		&self,
		_path: &Path,
		_query: &str,
		_limit: usize,
	) -> Result<Vec<DocEntry>, DocGenError> {
		self.search_returns
			.borrow_mut()
			.pop_front()
			.unwrap_or(Ok(vec![]))
	}
}
