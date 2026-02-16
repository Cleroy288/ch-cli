//! Tests for memory CLI command helpers.

use rustean::cli::commands::memory::filter_session;

use crate::helpers::factories::{
	make_interaction, make_interaction_for_session,
};

/// filter_session keeps matching session items
#[test]
fn filter_session_keeps_matching() {
	// Arrange
	let items = vec![
		make_interaction_for_session("ses-a"),
		make_interaction_for_session("ses-b"),
		make_interaction_for_session("ses-a"),
	];

	// Act
	let filtered =
		filter_session(items, "ses-a");

	// Assert
	assert_eq!(filtered.len(), 2);
	assert!(
		filtered
			.iter()
			.all(|i| i.session_id == "ses-a"),
	);
}

/// filter_session returns empty when no matches
#[test]
fn filter_session_no_match_returns_empty() {
	// Arrange
	let items = vec![
		make_interaction("hello"),
	];

	// Act
	let filtered =
		filter_session(items, "nonexistent");

	// Assert
	assert!(filtered.is_empty());
}

/// filter_session on empty input returns empty
#[test]
fn filter_session_empty_input_returns_empty() {
	// Act
	let filtered =
		filter_session(vec![], "any-session");

	// Assert
	assert!(filtered.is_empty());
}
