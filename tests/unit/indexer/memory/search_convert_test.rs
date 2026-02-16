//! Tests for memory search convert functions.

use rustean::domain::memory::AiResponse;
use rustean::indexer::memory::search_convert;
use rustean::indexer::memory::search_schema::{
	MemoryFields, build_memory_schema,
};

use crate::helpers::factories::make_interaction;

/// Test interaction_to_doc creates a document
#[test]
fn interaction_to_doc_creates_doc() {
	// Arrange
	let schema = build_memory_schema();
	let fields =
		MemoryFields::from_schema(&schema).unwrap();
	let item = make_interaction("test query");

	// Act
	let doc = search_convert::interaction_to_doc(
		&fields, &item,
	);

	// Assert — doc should have field values
	use tantivy::schema::Value;
	let id_val =
		doc.get_first(fields.id).unwrap();
	assert_eq!(
		id_val.as_str().unwrap(),
		item.id,
	);
}

/// Test flatten_response extracts text
#[test]
fn flatten_response_all_variants() {
	let cases = [
		AiResponse::Answer {
			text: "a".to_string(),
		},
		AiResponse::Question {
			text: "q".to_string(),
		},
		AiResponse::CodeChange {
			text: "c".to_string(),
			changes: Vec::new(),
		},
	];
	let expected = ["a", "q", "c"];
	for (resp, exp) in
		cases.iter().zip(expected.iter())
	{
		let text =
			search_convert::flatten_response(resp);
		assert_eq!(text, *exp);
	}
}

/// Test collect_files joins with spaces
#[test]
fn collect_files_joins_paths() {
	// Arrange
	let mut item =
		make_interaction("test query");
	item.input.files = vec![
		"a.rs".to_string(),
		"b.rs".to_string(),
	];

	// Act
	let result =
		search_convert::collect_files(&item);

	// Assert
	assert_eq!(result, "a.rs b.rs");
}
