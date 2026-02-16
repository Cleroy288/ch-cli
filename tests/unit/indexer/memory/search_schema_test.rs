//! Tests for memory search schema definition.

use tantivy::schema::Schema;

use rustean::indexer::memory::search_schema::{
	MemoryFields, build_memory_schema,
};

/// Test build_memory_schema has all fields
#[test]
fn schema_has_all_fields() {
	let schema = build_memory_schema();

	assert!(schema.get_field("id").is_ok());
	assert!(schema.get_field("session_id").is_ok());
	assert!(schema.get_field("timestamp").is_ok());
	assert!(schema.get_field("input_text").is_ok());
	assert!(
		schema.get_field("response_text").is_ok()
	);
	assert!(
		schema.get_field("response_type").is_ok()
	);
	assert!(schema.get_field("files").is_ok());
}

/// Test MemoryFields::from_schema succeeds
#[test]
fn fields_from_schema_ok() {
	let schema = build_memory_schema();
	let fields =
		MemoryFields::from_schema(&schema);

	assert!(fields.is_ok());
}

/// Test MemoryFields::from_schema fails on empty
#[test]
fn fields_from_empty_schema_fails() {
	let schema = Schema::builder().build();
	let result =
		MemoryFields::from_schema(&schema);

	assert!(result.is_err());
}

/// Test input_text field is searchable
#[test]
fn input_text_is_indexed() {
	let schema = build_memory_schema();
	let field =
		schema.get_field("input_text").unwrap();
	let entry = schema.get_field_entry(field);

	assert!(entry.is_indexed());
}

/// Test timestamp field is stored
#[test]
fn timestamp_is_stored() {
	let schema = build_memory_schema();
	let field =
		schema.get_field("timestamp").unwrap();
	let entry = schema.get_field_entry(field);

	assert!(entry.is_stored());
}
