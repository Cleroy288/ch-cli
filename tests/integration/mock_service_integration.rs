//! Integration tests for mock implementations.
//!
//! Validates that mocks correctly record calls
//! and return preset values — a meta-test to
//! ensure mock infrastructure works.

use std::path::Path;

use rustean::domain::errors::index::IndexError;
use rustean::service::daemon::types::{
	DaemonStatusInfo,
};
use rustean::service::{
	DaemonService, IndexService, SearchService,
};

use crate::helpers::mocks::{
	MockDaemonService, MockIndexService,
	MockSearchService,
};
use crate::helpers::factories_service::{
	make_index_options, make_search_options,
	make_search_result,
};

#[test]
fn mock_search_returns_preset_result() {
	// Arrange
	let mock = MockSearchService::new();
	let result = make_search_result();
	mock.on_search(Ok(result));
	let opts = make_search_options();

	// Act
	let got = mock.search(
		"greet",
		Path::new("/tmp"),
		&opts,
	);

	// Assert
	assert!(got.is_ok());
	assert_eq!(got.unwrap().hits.len(), 0);
}

#[test]
fn mock_search_records_calls() {
	// Arrange
	let mock = MockSearchService::new();
	let opts = make_search_options();
	for _ in 0..3 {
		mock.on_search(Ok(make_search_result()));
	}

	// Act
	for q in &["a", "b", "c"] {
		let _ = mock.search(
			q,
			Path::new("/tmp"),
			&opts,
		);
	}

	// Assert
	let calls = mock.search_calls.borrow();
	assert_eq!(calls.len(), 3);
	assert_eq!(calls[0].0, "a");
	assert_eq!(calls[2].0, "c");
}

#[test]
fn mock_index_returns_preset_error() {
	// Arrange
	let mock = MockIndexService::new();
	mock.on_index(Err(IndexError::Parser(
		"test error".into(),
	)));

	// Act
	let got = mock.index_project(
		Path::new("/tmp"),
		&make_index_options(),
	);

	// Assert
	assert!(got.is_err());
	let msg = got.unwrap_err().to_string();
	assert!(msg.contains("test error"));
}

#[test]
fn mock_daemon_status_returns_preset() {
	// Arrange
	let mock = MockDaemonService::new();
	let info = DaemonStatusInfo {
		is_running: true,
		pid: Some(1234),
		..DaemonStatusInfo::default()
	};
	mock.on_status(Ok(info));

	// Act
	let got = mock.status();

	// Assert
	let status = got.unwrap();
	assert!(status.is_running);
	assert_eq!(status.pid, Some(1234));
	let log = mock.call_log.borrow();
	assert_eq!(log.len(), 1);
	assert_eq!(log[0], "status");
}
