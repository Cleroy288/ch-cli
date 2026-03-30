//! Unit tests for parse_refs and parse_parents.

use rustean::service::git::{
	parse_parents, parse_refs,
};

// ── parse_refs ────────────────────────────────

#[test]
fn parse_refs_empty_string_returns_empty_vec() {
	// Arrange
	let raw = "";

	// Act
	let result = parse_refs(raw);

	// Assert
	assert_eq!(result, Vec::<String>::new());
}

#[test]
fn parse_refs_head_arrow_strips_head_prefix() {
	// Arrange
	let raw = "HEAD -> main";

	// Act
	let result = parse_refs(raw);

	// Assert
	assert_eq!(result, vec!["main"]);
}

#[test]
fn parse_refs_tag_strips_tag_prefix() {
	// Arrange
	let raw = "tag: v1.0";

	// Act
	let result = parse_refs(raw);

	// Assert
	assert_eq!(result, vec!["v1.0"]);
}

#[test]
fn parse_refs_origin_branch_strips_origin_prefix() {
	// Arrange
	let raw = "HEAD -> main, origin/develop";

	// Act
	let result = parse_refs(raw);

	// Assert
	assert_eq!(result, vec!["main", "develop"]);
}

#[test]
fn parse_refs_mixed_decorations_normalises_all() {
	// Arrange
	let raw = "HEAD -> main, tag: v1.0, origin/develop";

	// Act
	let result = parse_refs(raw);

	// Assert
	assert_eq!(
		result,
		vec!["main", "v1.0", "develop"],
	);
}

#[test]
fn parse_refs_detached_head_keeps_head() {
	// Arrange
	let raw = "HEAD";

	// Act
	let result = parse_refs(raw);

	// Assert
	assert_eq!(result, vec!["HEAD"]);
}

#[test]
fn parse_refs_non_prefix_origin_not_stripped() {
	// Arrange — "origin/" not a prefix here
	let raw = "my-origin/feature";

	// Act
	let result = parse_refs(raw);

	// Assert
	assert_eq!(result, vec!["my-origin/feature"]);
}

// ── parse_parents ─────────────────────────────

#[test]
fn parse_parents_empty_string_returns_empty_vec() {
	// Arrange
	let raw = "";

	// Act
	let result = parse_parents(raw);

	// Assert
	assert_eq!(result, Vec::<String>::new());
}

#[test]
fn parse_parents_single_hash_returns_one_element() {
	// Arrange
	let raw = "abc1234";

	// Act
	let result = parse_parents(raw);

	// Assert
	assert_eq!(result, vec!["abc1234"]);
}

#[test]
fn parse_parents_two_hashes_returns_both() {
	// Arrange
	let raw = "abc1234 def5678";

	// Act
	let result = parse_parents(raw);

	// Assert
	assert_eq!(result, vec!["abc1234", "def5678"]);
}
