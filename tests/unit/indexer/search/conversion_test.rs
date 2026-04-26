//! Tests for Symbol <-> Tantivy document conversion.

use std::path::PathBuf;

use rustean::indexer::search::{
	SchemaFields, build_schema, doc_to_symbol, symbol_to_doc,
};
use rustean::indexer::symbols::{
	ByteSpan, CodeLocation, SymbolKind, Visibility,
};
use rustean::indexer::Symbol;

/// symbol_to_doc populates all fields for a function symbol
#[test]
fn test_symbol_to_doc_function() {
	let schema = build_schema(); // create schema
	let fields = SchemaFields::from_schema(&schema).unwrap();
	let file = PathBuf::from("src/main.rs");
	let location = CodeLocation::new(file, 10, 5, ByteSpan::ZERO);
	let sig = "fn test_fn() -> bool".to_string();
	let symbol = Symbol::new(
		"test_fn".to_string(),
		SymbolKind::Function,
		location,
	)
	.with_visibility(Visibility::Public)
	.with_signature(sig);

	let doc = symbol_to_doc(&fields, &symbol);

	use tantivy::schema::Value;
	let name = doc.get_first(fields.symbol_name).unwrap();
	assert_eq!(name.as_str().unwrap(), "test_fn");
	let kind = doc.get_first(fields.symbol_kind).unwrap();
	assert_eq!(kind.as_str().unwrap(), "fn");
	let path = doc.get_first(fields.file_path).unwrap();
	assert_eq!(path.as_str().unwrap(), "src/main.rs");
	let line = doc.get_first(fields.line).unwrap();
	assert_eq!(line.as_u64().unwrap(), 10);
	let vis = doc.get_first(fields.visibility).unwrap();
	assert_eq!(vis.as_str().unwrap(), "pub");
	let signature = doc.get_first(fields.signature).unwrap();
	assert_eq!(
		signature.as_str().unwrap(),
		"fn test_fn() -> bool"
	);
}

/// symbol_to_doc handles missing optional fields
#[test]
fn test_symbol_to_doc_minimal() {
	let schema = build_schema(); // create schema
	let fields = SchemaFields::from_schema(&schema).unwrap();
	let file = PathBuf::from("lib.rs");
	let location = CodeLocation::new(file, 1, 0, ByteSpan::ZERO);
	let symbol = Symbol::new(
		"my_struct".to_string(),
		SymbolKind::Struct,
		location,
	);

	let doc = symbol_to_doc(&fields, &symbol);

	use tantivy::schema::Value;
	let name = doc.get_first(fields.symbol_name).unwrap();
	assert_eq!(name.as_str().unwrap(), "my_struct");
	let kind = doc.get_first(fields.symbol_kind).unwrap();
	assert_eq!(kind.as_str().unwrap(), "struct");
}

/// doc_to_symbol roundtrip preserves function fields
#[test]
fn test_roundtrip_function() {
	let schema = build_schema(); // create schema
	let fields = SchemaFields::from_schema(&schema).unwrap();
	let file = PathBuf::from("src/lib.rs");
	let location = CodeLocation::new(file, 42, 8, ByteSpan::ZERO);
	let sig = "fn calculate(x: i32) -> i32".to_string();
	let original = Symbol::new(
		"calculate".to_string(),
		SymbolKind::Function,
		location,
	)
	.with_visibility(Visibility::PublicCrate)
	.with_signature(sig.clone())
	.with_parent("MyModule".to_string());

	let doc = symbol_to_doc(&fields, &original);
	let restored = doc_to_symbol(&fields, &doc).unwrap();

	assert_eq!(restored.name, "calculate");
	assert_eq!(restored.kind, SymbolKind::Function);
	let expected_file = PathBuf::from("src/lib.rs");
	assert_eq!(restored.location.file, expected_file);
	assert_eq!(restored.location.line, 42);
	assert_eq!(restored.visibility, Visibility::PublicCrate);
	assert_eq!(restored.signature, Some(sig));
	assert_eq!(restored.parent, Some("MyModule".to_string()));
}

/// doc_to_symbol roundtrip preserves struct fields
#[test]
fn test_roundtrip_struct() {
	let schema = build_schema(); // create schema
	// extract fields
	let fields = SchemaFields::from_schema(&schema).unwrap();
	// code location
	let location = CodeLocation::new(
		PathBuf::from("types.rs"), 5, 0, ByteSpan::ZERO,
	);
	// original struct symbol
	let original = Symbol::new(
		"Config".to_string(),
		SymbolKind::Struct,
		location
	).with_visibility(Visibility::Public);

	let doc = symbol_to_doc(&fields, &original); // convert to doc
	let restored = doc_to_symbol(&fields, &doc).unwrap(); // convert back

	assert_eq!(restored.name, "Config");
	assert_eq!(restored.kind, SymbolKind::Struct);
	assert_eq!(restored.visibility, Visibility::Public);
}

/// doc_to_symbol roundtrip preserves all SymbolKind variants
#[test]
fn test_symbol_kinds_roundtrip() {
	let schema = build_schema(); // create schema
	// extract fields
	let fields = SchemaFields::from_schema(&schema).unwrap();
	// symbol kinds to test
	let kinds = vec![
		(SymbolKind::Function, "fn"),
		(SymbolKind::Struct, "struct"),
		(SymbolKind::Enum, "enum"),
		(SymbolKind::Module, "mod"),
	];

	for (kind, _) in kinds {
		let location = CodeLocation::new(
			PathBuf::from("test.rs"), 1, 0, ByteSpan::ZERO,
		);
		let symbol = Symbol::new(
			"test".to_string(), kind, location
		);
		let doc = symbol_to_doc(&fields, &symbol);
		let restored = doc_to_symbol(&fields, &doc).unwrap();

		assert_eq!(restored.kind, kind);
	}
}
