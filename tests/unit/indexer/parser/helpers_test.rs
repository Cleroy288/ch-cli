//! Tests for parser helper functions.

use ch_cli::indexer::parser::{is_rust_keyword, parse_visibility};
use ch_cli::indexer::symbols::Visibility;

/// Test parse_visibility with "pub"
#[test]
fn test_parse_visibility_pub() {
	let result = parse_visibility("pub"); // parse plain pub

	assert_eq!(result, Visibility::Public);
}

/// Test parse_visibility with "pub(crate)"
#[test]
fn test_parse_visibility_pub_crate() {
	let result = parse_visibility("pub(crate)"); // parse pub(crate)

	assert_eq!(result, Visibility::PublicCrate);
}

/// Test parse_visibility with "pub(super)"
#[test]
fn test_parse_visibility_pub_super() {
	let result = parse_visibility("pub(super)"); // parse pub(super)

	assert_eq!(result, Visibility::PublicSuper);
}

/// Test parse_visibility with empty string defaults to Private
#[test]
fn test_parse_visibility_empty() {
	let result = parse_visibility(""); // parse empty string

	assert_eq!(result, Visibility::Private);
}

/// Test parse_visibility with unknown string defaults to Private
#[test]
fn test_parse_visibility_unknown() {
	let result = parse_visibility("something_else"); // parse unknown modifier

	assert_eq!(result, Visibility::Private);
}

/// Test is_rust_keyword returns true for known keywords
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

/// Test is_rust_keyword returns false for non-keywords
#[test]
fn test_is_rust_keyword_non_keyword() {
	let result = is_rust_keyword("my_variable"); // a regular identifier

	assert!(!result);
}

/// Test is_rust_keyword returns false for empty string
#[test]
fn test_is_rust_keyword_empty() {
	let result = is_rust_keyword(""); // empty string

	assert!(!result);
}
