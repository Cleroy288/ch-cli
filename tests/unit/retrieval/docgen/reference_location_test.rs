//! Tests for retrieval::docgen::reference_location

use std::path::PathBuf;

use rustean::retrieval::docgen::entry_types::ReferenceKind;
use rustean::retrieval::docgen::ReferenceLocation;

/// Verify ReferenceLocation builder methods chain correctly.
#[test]
fn test_reference_location_builders() {
	let path = PathBuf::from("src/lib.rs");
	let context_str = "let x = foo();".to_string();
	let module = "retrieval::hybrid".to_string();
	let symbol = "process".to_string();

	let loc = ReferenceLocation::new(
		path.clone(),
		42,
		ReferenceKind::Call,
	)
	.with_context(context_str.clone())
	.with_module_path(module.clone())
	.with_containing_symbol(symbol.clone());

	assert_eq!(loc.file_path, path);
	assert_eq!(loc.line, 42);
	assert_eq!(loc.context, context_str);
	assert_eq!(loc.ref_kind, ReferenceKind::Call);
	assert_eq!(loc.module_path, Some(module));
	assert_eq!(loc.containing_symbol, Some(symbol));
}
