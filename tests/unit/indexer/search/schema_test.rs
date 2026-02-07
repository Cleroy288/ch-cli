//! Tests for search schema definition.

use tantivy::schema::Schema;

use ch_cli::indexer::search::{SchemaFields, build_schema};

/// Test build_schema creates schema with all required fields
#[test]
fn test_build_schema() {
	let schema = build_schema(); // build schema

	assert!(schema.get_field("symbol_name").is_ok());
	assert!(schema.get_field("symbol_kind").is_ok());
	assert!(schema.get_field("file_path").is_ok());
	assert!(schema.get_field("line").is_ok());
	assert!(schema.get_field("column").is_ok());
	assert!(schema.get_field("visibility").is_ok());
	assert!(schema.get_field("signature").is_ok());
	assert!(schema.get_field("fqn").is_ok());
	assert!(schema.get_field("parent").is_ok());
	assert!(schema.get_field("content").is_ok());
	assert!(schema.get_field("document_type").is_ok());
}

/// Test SchemaFields::from_schema extracts all field handles
#[test]
fn test_schema_fields_from_schema() {
	let schema = build_schema(); // build schema
	let fields = SchemaFields::from_schema(&schema);

	assert!(fields.is_ok());
	let fields = fields.unwrap();
	let expected_name = schema.get_field("symbol_name").unwrap();
	assert_eq!(expected_name, fields.symbol_name);
	let expected_kind = schema.get_field("symbol_kind").unwrap();
	assert_eq!(expected_kind, fields.symbol_kind);
	let expected_path = schema.get_field("file_path").unwrap();
	assert_eq!(expected_path, fields.file_path);
	assert_eq!(schema.get_field("line").unwrap(), fields.line);
	let expected_col = schema.get_field("column").unwrap();
	assert_eq!(expected_col, fields.column);
}

/// Test SchemaFields::from_schema returns error for invalid schema
#[test]
fn test_schema_fields_invalid_schema() {
	let schema = Schema::builder().build(); // empty schema
	let result = SchemaFields::from_schema(&schema);

	assert!(result.is_err());
}

/// Test build_schema creates searchable text fields
#[test]
fn test_schema_text_fields() {
	let schema = build_schema(); // build schema
	let symbol_name_field = schema.get_field("symbol_name").unwrap();
	let entry = schema.get_field_entry(symbol_name_field);

	assert!(entry.is_indexed());
}

/// Test build_schema creates stored numeric fields
#[test]
fn test_schema_numeric_fields() {
	let schema = build_schema(); // build schema
	let line_field = schema.get_field("line").unwrap();
	let entry = schema.get_field_entry(line_field);

	assert!(entry.is_stored());
}
