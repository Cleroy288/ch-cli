//! Tests for Visibility Display implementation.

use rustean::indexer::symbols::Visibility;

/// Display formats each variant correctly
#[test]
fn display_table() {
	let cases: &[(Visibility, &str)] = &[
		(Visibility::Public, "pub"),
		(Visibility::PublicCrate, "pub(crate)"),
		(Visibility::PublicSuper, "pub(super)"),
		(Visibility::Private, ""),
	];

	for (vis, expected) in cases {
		assert_eq!(
			format!("{vis}"),
			*expected,
			"Display for {vis:?}",
		);
	}
}
