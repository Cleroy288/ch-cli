//! Tests for Visibility Display implementation.

use ch_cli::indexer::symbols::Visibility;

/// Test Display for Public visibility
#[test]
fn test_display_public() {
	assert_eq!(format!("{}", Visibility::Public), "pub");
}

/// Test Display for PublicCrate visibility
#[test]
fn test_display_public_crate() {
	assert_eq!(
		format!("{}", Visibility::PublicCrate),
		"pub(crate)"
	);
}

/// Test Display for PublicSuper visibility
#[test]
fn test_display_public_super() {
	assert_eq!(
		format!("{}", Visibility::PublicSuper),
		"pub(super)"
	);
}

/// Test Display for Private visibility (empty string)
#[test]
fn test_display_private() {
	assert_eq!(format!("{}", Visibility::Private), "");
}
