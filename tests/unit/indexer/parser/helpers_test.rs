//! Tests for parser helper functions.

use rustean::indexer::parser::{is_rust_keyword, parse_visibility};
use rustean::indexer::symbols::Visibility;

/// parse_visibility maps "pub" to Public
#[test]
fn test_parse_visibility_pub() {
	let result = parse_visibility("pub"); // parse plain pub

	assert_eq!(result, Visibility::Public);
}

/// parse_visibility maps "pub(crate)" to PublicCrate
#[test]
fn test_parse_visibility_pub_crate() {
	let result = parse_visibility("pub(crate)"); // parse pub(crate)

	assert_eq!(result, Visibility::PublicCrate);
}

/// parse_visibility maps "pub(super)" to PublicSuper
#[test]
fn test_parse_visibility_pub_super() {
	let result = parse_visibility("pub(super)"); // parse pub(super)

	assert_eq!(result, Visibility::PublicSuper);
}

/// parse_visibility defaults empty string to Private
#[test]
fn test_parse_visibility_empty() {
	let result = parse_visibility(""); // parse empty string

	assert_eq!(result, Visibility::Private);
}

/// parse_visibility defaults unknown string to Private
#[test]
fn test_parse_visibility_unknown() {
	let result = parse_visibility("something_else"); // parse unknown modifier

	assert_eq!(result, Visibility::Private);
}

/// is_rust_keyword returns true for known keywords
#[test]
fn test_is_rust_keyword_fn() {
	assert!(is_rust_keyword("fn"));
}

#[test]
fn test_is_rust_keyword_struct() {
	assert!(is_rust_keyword("struct"));
}

#[test]
fn test_is_rust_keyword_let() {
	assert!(is_rust_keyword("let"));
}

/// is_rust_keyword returns false for non-keywords
#[test]
fn test_is_rust_keyword_non_keyword() {
	let result = is_rust_keyword("my_variable"); // a regular identifier

	assert!(!result);
}

/// is_rust_keyword returns false for empty string
#[test]
fn test_is_rust_keyword_empty() {
	let result = is_rust_keyword(""); // empty string

	assert!(!result);
}
