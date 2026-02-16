//! Integration tests for index lifecycle.
//!
//! Exercises real IndexService operations:
//! indexing, stats, has_index, clear_index.

use rustean::service::index::types::{
	IndexFlags, IndexOptions,
};
use rustean::service::{
	DefaultIndexService, IndexService,
};

use crate::helpers::factories_service::{
	cleanup_project, make_rust_project,
};

/// Shared options: semantic on, no persistence
fn opts() -> IndexOptions {
	IndexOptions {
		flags: IndexFlags {
			semantic: true,
			verbose: false,
			persistence: false,
		},
	}
}

#[test]
fn index_project_returns_stats() {
	// Arrange
	let dir = make_rust_project("lifecycle_stats");
	let svc = DefaultIndexService::new();

	// Act
	let result =
		svc.index_project(&dir, &opts());

	// Assert
	assert!(result.is_ok());
	let r = result.unwrap();
	assert!(!r.symbols.is_empty());

	cleanup_project(&dir);
}

#[test]
fn has_index_true_after_indexing() {
	// Arrange
	let dir = make_rust_project("lifecycle_has");
	let svc = DefaultIndexService::new();
	let persist_opts = IndexOptions {
		flags: IndexFlags {
			semantic: true,
			verbose: false,
			persistence: true,
		},
	};

	// Act
	let _ = svc.index_project(
		&dir,
		&persist_opts,
	);

	// Assert
	assert!(svc.has_index(&dir));

	cleanup_project(&dir);
}

#[test]
fn clear_index_removes_data() {
	// Arrange
	let dir =
		make_rust_project("lifecycle_clear");
	let svc = DefaultIndexService::new();
	let persist_opts = IndexOptions {
		flags: IndexFlags {
			semantic: true,
			verbose: false,
			persistence: true,
		},
	};
	let _ = svc.index_project(
		&dir,
		&persist_opts,
	);
	assert!(svc.has_index(&dir));

	// Act
	let clear = svc.clear_index(&dir);

	// Assert
	assert!(clear.is_ok());
	assert!(!svc.has_index(&dir));

	cleanup_project(&dir);
}

#[test]
fn get_stats_after_persistent_index() {
	// Arrange
	let dir =
		make_rust_project("lifecycle_persist");
	let svc = DefaultIndexService::new();
	let persist_opts = IndexOptions {
		flags: IndexFlags {
			semantic: true,
			verbose: false,
			persistence: true,
		},
	};

	// Act
	let _ = svc.index_project(
		&dir,
		&persist_opts,
	);
	let stats = svc.get_stats(&dir);

	// Assert
	assert!(stats.is_ok());
	let s = stats.unwrap();
	assert!(s.is_some());
	let info = s.unwrap();
	assert!(info.symbol_count > 0);

	cleanup_project(&dir);
}
