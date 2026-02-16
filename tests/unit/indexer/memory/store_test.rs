//! Tests for memory JSONL store.

use rustean::indexer::memory::store;

use crate::helpers::factories::{
	cleanup_test_dir, make_interaction_for_session,
	make_memory_test_dir,
};

/// Test append and load_session round-trip
#[test]
fn append_then_load_session() {
	// Arrange
	let dir = make_memory_test_dir("store-append");
	let item = make_interaction_for_session("s-1");

	// Act
	store::append(&dir, &item).unwrap();
	let loaded =
		store::load_session(&dir, "s-1").unwrap();

	// Assert
	assert_eq!(loaded.len(), 1);
	assert_eq!(loaded[0].id, item.id);

	cleanup_test_dir(&dir);
}

/// Test load_recent returns items
#[test]
fn load_recent_returns_items() {
	// Arrange
	let dir = make_memory_test_dir("store-recent");
	let item1 = make_interaction_for_session("s-1");
	let item2 = make_interaction_for_session("s-1");
	store::append(&dir, &item1).unwrap();
	store::append(&dir, &item2).unwrap();

	// Act
	let recent =
		store::load_recent(&dir, 10).unwrap();

	// Assert
	assert_eq!(recent.len(), 2);

	cleanup_test_dir(&dir);
}

/// Test load_recent respects limit
#[test]
fn load_recent_respects_limit() {
	// Arrange
	let dir = make_memory_test_dir("store-limit");
	for _ in 0..5 {
		let item =
			make_interaction_for_session("s-1");
		store::append(&dir, &item).unwrap();
	}

	// Act
	let recent =
		store::load_recent(&dir, 3).unwrap();

	// Assert
	assert_eq!(recent.len(), 3);

	cleanup_test_dir(&dir);
}

/// Test list_sessions returns session IDs
#[test]
fn list_sessions_returns_ids() {
	// Arrange
	let dir =
		make_memory_test_dir("store-sessions");
	store::append(
		&dir,
		&make_interaction_for_session("s-a"),
	)
	.unwrap();
	store::append(
		&dir,
		&make_interaction_for_session("s-b"),
	)
	.unwrap();

	// Act
	let sessions =
		store::list_sessions(&dir).unwrap();

	// Assert
	assert!(sessions.len() >= 2);

	cleanup_test_dir(&dir);
}

/// Test load_session missing returns error
#[test]
fn load_session_not_found() {
	// Arrange
	let dir =
		make_memory_test_dir("store-not-found");

	// Act
	let result =
		store::load_session(&dir, "missing");

	// Assert
	assert!(result.is_err());

	cleanup_test_dir(&dir);
}
