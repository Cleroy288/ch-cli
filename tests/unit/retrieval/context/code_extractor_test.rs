//! Tests for retrieval::context::code_extractor

use std::path::PathBuf;

use rustean::indexer::{
	ByteSpan, CodeLocation, Symbol, SymbolKind,
};
use rustean::retrieval::context::code_extractor::{
	find_symbol_end,
};

#[test]
fn test_find_symbol_end_function() {
	let lines = vec![
		"fn foo() {",
		"    let x = 1;",
		"    x + 1",
		"}",
		"",
		"fn bar() {",
	];

	let symbol = Symbol::new(
		"foo".to_string(),
		SymbolKind::Function,
		CodeLocation::new(
			PathBuf::from("test.rs"),
			1, 1, ByteSpan::ZERO,
		),
	);

	let end = find_symbol_end(&lines, 0, &symbol);
	assert_eq!(end, 4);
}
