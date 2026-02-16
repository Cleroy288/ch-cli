//! Tests for retrieval::docgen::generator_utils

use rustean::retrieval::docgen::generator_utils::clean_generated_doc;

#[test]
fn test_clean_generated_doc() {
	let doc = "Description: This is a test function.";
	let cleaned = clean_generated_doc(doc);
	assert_eq!(cleaned, "This is a test function.");

	let doc2 = "description: Another test.";
	let cleaned2 = clean_generated_doc(doc2);
	assert_eq!(cleaned2, "Another test.");
}

#[test]
fn test_clean_generated_doc_code_block() {
	let doc =
		"```\nThis is inside a code block.\n```";
	let cleaned = clean_generated_doc(doc);
	assert_eq!(cleaned, "This is inside a code block.");
}
