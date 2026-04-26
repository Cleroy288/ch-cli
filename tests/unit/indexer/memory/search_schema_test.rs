//! Tests for memory search schema definition.

use tantivy::schema::Schema;

use rustean::indexer::memory::search_schema::{
	MemoryFields, build_memory_schema,
};

/// Schema contains all required field names
#[test]
fn schema_has_all_fields() {
	let schema = build_memory_schema();
	let expected = [
		"id", "session_id", "timestamp",
		"input_text", "response_text",
		"response_type", "files",
	];
	for name in expected {
		assert!(
			schema.get_field(name).is_ok(),
			"missing field: {name}",
		);
	}
}

/// MemoryFields extraction succeeds on valid schema
#[test]
fn fields_from_schema_succeeds() {
	let schema = build_memory_schema();
	let fields =
		MemoryFields::from_schema(&schema);
	assert!(fields.is_ok());
}

/// MemoryFields extraction fails on empty schema
#[test]
fn fields_from_empty_schema_fails() {
	let schema = Schema::builder().build();
	let result =
		MemoryFields::from_schema(&schema);
	assert!(result.is_err());
}

/// input_text field is indexed (searchable)
#[test]
fn input_text_is_indexed() {
	let schema = build_memory_schema();
	let field =
		schema.get_field("input_text").unwrap();
	let entry = schema.get_field_entry(field);
	assert!(entry.is_indexed());
}

/// timestamp field is stored (retrievable)
#[test]
fn timestamp_is_stored() {
	let schema = build_memory_schema();
	let field =
		schema.get_field("timestamp").unwrap();
	let entry = schema.get_field_entry(field);
	assert!(entry.is_stored());
}
