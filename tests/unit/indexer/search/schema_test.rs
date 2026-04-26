//! Tests for search schema definition.

use tantivy::schema::Schema;

use rustean::indexer::search::{
	SchemaFields, build_schema,
};

/// Schema contains all required field names
#[test]
fn build_schema_has_all_fields() {
	let schema = build_schema();
	let expected_fields = [
		"symbol_name", "symbol_kind", "file_path",
		"line", "column", "visibility", "signature",
		"fqn", "parent", "content", "document_type",
	];
	for name in expected_fields {
		assert!(
			schema.get_field(name).is_ok(),
			"missing field: {name}",
		);
	}
}

/// SchemaFields::from_schema extracts field handles
#[test]
fn schema_fields_from_schema_matches() {
	let schema = build_schema();
	let fields =
		SchemaFields::from_schema(&schema).unwrap();

	assert_eq!(
		schema.get_field("symbol_name").unwrap(),
		fields.symbol_name,
	);
	assert_eq!(
		schema.get_field("symbol_kind").unwrap(),
		fields.symbol_kind,
	);
	assert_eq!(
		schema.get_field("file_path").unwrap(),
		fields.file_path,
	);
	assert_eq!(
		schema.get_field("line").unwrap(),
		fields.line,
	);
	assert_eq!(
		schema.get_field("column").unwrap(),
		fields.column,
	);
}

/// Empty schema fails SchemaFields extraction
#[test]
fn schema_fields_from_empty_schema_fails() {
	let schema = Schema::builder().build();
	let result =
		SchemaFields::from_schema(&schema);
	assert!(result.is_err());
}

/// symbol_name field is indexed (searchable)
#[test]
fn symbol_name_field_is_indexed() {
	let schema = build_schema();
	let field =
		schema.get_field("symbol_name").unwrap();
	let entry = schema.get_field_entry(field);
	assert!(entry.is_indexed());
}

/// line field is stored (retrievable)
#[test]
fn line_field_is_stored() {
	let schema = build_schema();
	let field =
		schema.get_field("line").unwrap();
	let entry = schema.get_field_entry(field);
	assert!(entry.is_stored());
}
