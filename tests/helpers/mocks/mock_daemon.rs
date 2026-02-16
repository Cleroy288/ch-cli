//! Mock implementation of DaemonService.
//!
//! Records calls and returns preset values for
//! testing without a real daemon process.

use std::cell::RefCell;
use std::collections::VecDeque;

use rustean::domain::errors::daemon::DaemonError;
use rustean::service::daemon::types::{
	DaemonStatusInfo,
};
use rustean::service::DaemonService;

/// Mock DaemonService for testing
pub struct MockDaemonService {
	/// preset returns for start
	start_returns:
		RefCell<VecDeque<Result<(), DaemonError>>>,
	/// preset returns for stop
	stop_returns:
		RefCell<VecDeque<Result<(), DaemonError>>>,
	/// preset returns for status
	status_returns: RefCell<
		VecDeque<
			Result<DaemonStatusInfo, DaemonError>,
		>,
	>,
	/// recorded method call names
	pub call_log: RefCell<Vec<String>>,
}

impl MockDaemonService {
	/// Create an empty mock
	pub fn new() -> Self {
		Self {
			start_returns: RefCell::new(
				VecDeque::new(),
			),
			stop_returns: RefCell::new(
				VecDeque::new(),
			),
			status_returns: RefCell::new(
				VecDeque::new(),
			),
			call_log: RefCell::new(Vec::new()),
		}
	}

	/// Preset a return for start
	pub fn on_start(
		&self,
		r: Result<(), DaemonError>,
	) {
		self.start_returns
			.borrow_mut()
			.push_back(r);
	}

	/// Preset a return for stop
	pub fn on_stop(
		&self,
		r: Result<(), DaemonError>,
	) {
		self.stop_returns
			.borrow_mut()
			.push_back(r);
	}

	/// Preset a return for status
	pub fn on_status(
		&self,
		r: Result<DaemonStatusInfo, DaemonError>,
	) {
		self.status_returns
			.borrow_mut()
			.push_back(r);
	}
}

impl DaemonService for MockDaemonService {
	fn start(&self) -> Result<(), DaemonError> {
		self.call_log
			.borrow_mut()
			.push("start".into());
		self.start_returns
			.borrow_mut()
			.pop_front()
			.unwrap_or(Ok(()))
	}

	fn stop(&self) -> Result<(), DaemonError> {
		self.call_log
			.borrow_mut()
			.push("stop".into());
		self.stop_returns
			.borrow_mut()
			.pop_front()
			.unwrap_or(Ok(()))
	}

	fn restart(
		&self,
	) -> Result<(), DaemonError> {
		self.call_log
			.borrow_mut()
			.push("restart".into());
		// restart delegates to stop + start
		self.stop()?;
		self.start()
	}

	fn status(
		&self,
	) -> Result<DaemonStatusInfo, DaemonError> {
		self.call_log
			.borrow_mut()
			.push("status".into());
		self.status_returns
			.borrow_mut()
			.pop_front()
			.unwrap_or(Ok(
				DaemonStatusInfo::default(),
			))
	}

	fn run_foreground(
		&self,
		_socket: Option<&str>,
	) -> Result<(), DaemonError> {
		self.call_log
			.borrow_mut()
			.push("run_foreground".into());
		Ok(())
	}
}
