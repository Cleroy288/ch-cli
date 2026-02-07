//! Tests for retrieval::docgen::entry_types

use ch_cli::retrieval::docgen::entry_types::ReferenceKind;

/// Verify Display impl for all ReferenceKind variants.
#[test]
fn test_reference_kind_display() {
	let cases: Vec<(ReferenceKind, &str)> = vec![
		(ReferenceKind::Call, "call"),
		(ReferenceKind::TypeUsage, "type_usage"),
		(ReferenceKind::Import, "import"),
		(ReferenceKind::TraitImpl, "trait_impl"),
		(ReferenceKind::Derive, "derive"),
		(ReferenceKind::FieldAccess, "field_access"),
		(ReferenceKind::ExternalCrate, "external_crate"),
		(ReferenceKind::GenericParam, "generic_param"),
		(ReferenceKind::ReturnType, "return_type"),
		(ReferenceKind::ParamType, "param_type"),
	];

	for (kind, expected) in cases {
		let result = format!("{}", kind);
		assert_eq!(result, expected);
	}
}
