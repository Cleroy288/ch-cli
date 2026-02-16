//! Tests for retrieval::context::context_xml

use std::path::PathBuf;

use rustean::indexer::{
	ByteSpan, CodeLocation,
	ReferenceContext, SymbolKind,
};
use rustean::retrieval::context::{
	ContextualBlock,
	graph_walker_usage::UsageInfo,
};

/// Helper to create a test ContextualBlock
fn create_test_block(
	usages: Vec<UsageInfo>,
) -> ContextualBlock {
	let symbol = rustean::indexer::Symbol::new(
		"test_fn".to_string(),
		SymbolKind::Function,
		CodeLocation::new(
			PathBuf::from("/test/main.rs"),
			10, 1, ByteSpan::ZERO,
		),
	);
	let usage_count = usages.len();

	ContextualBlock {
		symbol,
		code_snippet: "fn test_fn() {}".to_string(),
		parent: None,
		related_types: Vec::new(),
		callers: Vec::new(),
		callees: Vec::new(),
		doc_comment: None,
		usage_count,
		usages,
	}
}

#[test]
fn test_usages_grouped_by_file_in_xml() {
	let usages = vec![
		UsageInfo {
			file: PathBuf::from("/project/b.rs"),
			line: 20,
			context: ReferenceContext::Call,
			snippet: Some(
				"test_fn()".to_string(),
			),
			containing_symbol: Some(
				"b".to_string(),
			),
		},
		UsageInfo {
			file: PathBuf::from("/project/a.rs"),
			line: 10,
			context: ReferenceContext::Call,
			snippet: Some(
				"test_fn()".to_string(),
			),
			containing_symbol: Some(
				"a1".to_string(),
			),
		},
	];

	let block = create_test_block(usages);
	let xml = block.to_xml();

	assert!(xml.contains("<usages total=\"2\">"));

	let a_pos =
		xml.find("/project/a.rs").unwrap();
	let b_pos =
		xml.find("/project/b.rs").unwrap();
	assert!(a_pos < b_pos);
}

#[test]
fn test_usage_xml_escapes_special_chars() {
	let usages = vec![UsageInfo {
		file: PathBuf::from("/test/file.rs"),
		line: 5,
		context: ReferenceContext::Type,
		snippet: Some(
			"x < 10 && y > 5".to_string(),
		),
		containing_symbol: None,
	}];

	let block = create_test_block(usages);
	let xml = block.to_xml();

	assert!(xml.contains("&lt;"));
	assert!(xml.contains("&gt;"));
	assert!(xml.contains("&amp;"));
}

#[test]
fn test_empty_usages_no_xml_section() {
	let block = create_test_block(Vec::new());
	let xml = block.to_xml();

	assert!(!xml.contains("<usages"));
}

#[test]
fn test_token_count() {
	let text = "fn example() -> i32 { 42 }";
	let doc = "Returns an integer";

	let symbol = rustean::indexer::Symbol::new(
		"example".to_string(),
		SymbolKind::Function,
		CodeLocation::new(
			PathBuf::from("/test/main.rs"),
			1, 1, ByteSpan::ZERO,
		),
	);

	let block = ContextualBlock {
		symbol,
		code_snippet: text.to_string(),
		parent: None,
		related_types: Vec::new(),
		callers: Vec::new(),
		callees: Vec::new(),
		doc_comment: Some(doc.to_string()),
		usage_count: 0,
		usages: Vec::new(),
	};

	let tokens = block.token_count();
	let expected = (text.len()
		+ doc.len()
		+ "example".len() * 2) / 4;

	assert_eq!(tokens, expected);
	assert!(tokens > 0);
}
