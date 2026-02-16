//! Tests for memory stats computation.

use rustean::indexer::memory::{stats, store};

use crate::helpers::factories::{
	cleanup_test_dir, make_interaction_for_session,
	make_memory_test_dir,
};

/// Test compute_stats on empty directory
#[test]
fn stats_empty_dir() {
	// Arrange
	let dir = make_memory_test_dir("stats-empty");

	// Act
	let result = stats::compute_stats(&dir);

	// Assert
	assert!(result.is_ok());
	let stats = result.unwrap();
	assert_eq!(stats.total, 0);
	assert_eq!(stats.sessions, 0);

	cleanup_test_dir(&dir);
}

/// Test compute_stats with data
#[test]
fn stats_with_data() {
	// Arrange
	let dir = make_memory_test_dir("stats-data");
	store::append(
		&dir,
		&make_interaction_for_session("s-1"),
	)
	.unwrap();
	store::append(
		&dir,
		&make_interaction_for_session("s-1"),
	)
	.unwrap();
	store::append(
		&dir,
		&make_interaction_for_session("s-2"),
	)
	.unwrap();

	// Act
	let stats =
		stats::compute_stats(&dir).unwrap();

	// Assert
	assert_eq!(stats.total, 3);
	assert_eq!(stats.sessions, 2);
	assert!(stats.size_bytes > 0);

	cleanup_test_dir(&dir);
}

/// Test save and load cached stats
#[test]
fn save_and_load_cached() {
	// Arrange
	let dir = make_memory_test_dir("stats-cache");
	store::append(
		&dir,
		&make_interaction_for_session("s-1"),
	)
	.unwrap();
	stats::compute_stats(&dir).unwrap();

	// Act
	let cached = stats::load_cached(&dir);

	// Assert
	assert!(cached.is_ok());
	let stats = cached.unwrap();
	assert_eq!(stats.total, 1);

	cleanup_test_dir(&dir);
}

/// Test load_cached on missing file
#[test]
fn load_cached_missing_returns_err() {
	// Arrange
	let dir =
		make_memory_test_dir("stats-missing");

	// Act
	let result = stats::load_cached(&dir);

	// Assert
	assert!(result.is_err());

	cleanup_test_dir(&dir);
}
