//! Mock implementation of RetrievalService.
//!
//! Records calls and returns preset values for
//! testing without real ML pipeline.

use std::cell::RefCell;
use std::collections::VecDeque;

use rustean::domain::errors::retrieval::{
	RetrievalError,
};
use rustean::retrieval::{
	RetrievalOutput, StructuredOutput,
};
use rustean::service::retrieval::types::{
	RetrievalRequest,
};
use rustean::service::RetrievalService;

/// Mock RetrievalService for testing
pub struct MockRetrievalService {
	/// preset returns for retrieve
	retrieve_returns: RefCell<
		VecDeque<
			Result<RetrievalOutput, RetrievalError>,
		>,
	>,
	/// preset returns for retrieve_structured
	structured_returns: RefCell<
		VecDeque<
			Result<StructuredOutput, RetrievalError>,
		>,
	>,
	/// recorded retrieve calls (query strings)
	pub retrieve_calls: RefCell<Vec<String>>,
	/// recorded structured calls (query strings)
	pub structured_calls: RefCell<Vec<String>>,
}

impl MockRetrievalService {
	/// Create an empty mock
	pub fn new() -> Self {
		Self {
			retrieve_returns: RefCell::new(
				VecDeque::new(),
			),
			structured_returns: RefCell::new(
				VecDeque::new(),
			),
			retrieve_calls: RefCell::new(Vec::new()),
			structured_calls: RefCell::new(
				Vec::new(),
			),
		}
	}

	/// Preset a return for retrieve
	pub fn on_retrieve(
		&self,
		r: Result<RetrievalOutput, RetrievalError>,
	) {
		self.retrieve_returns
			.borrow_mut()
			.push_back(r);
	}

	/// Preset a return for retrieve_structured
	pub fn on_structured(
		&self,
		r: Result<StructuredOutput, RetrievalError>,
	) {
		self.structured_returns
			.borrow_mut()
			.push_back(r);
	}
}

impl RetrievalService for MockRetrievalService {
	fn retrieve(
		&self,
		req: &RetrievalRequest,
	) -> Result<RetrievalOutput, RetrievalError> {
		self.retrieve_calls
			.borrow_mut()
			.push(req.query.clone());
		self.retrieve_returns
			.borrow_mut()
			.pop_front()
			.unwrap_or_else(|| {
				Err(RetrievalError::DaemonNotRunning(
					"no preset".into(),
				))
			})
	}

	fn retrieve_structured(
		&self,
		req: &RetrievalRequest,
	) -> Result<StructuredOutput, RetrievalError> {
		self.structured_calls
			.borrow_mut()
			.push(req.query.clone());
		self.structured_returns
			.borrow_mut()
			.pop_front()
			.unwrap_or_else(|| {
				Err(RetrievalError::DaemonNotRunning(
					"no preset".into(),
				))
			})
	}
}
