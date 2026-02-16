//! Tests for load_all_symbols (bulk Tantivy reload).

use std::path::PathBuf;

use rustean::indexer::search::SearchIndex;
use rustean::indexer::symbols::{
	ByteSpan, CodeLocation, Symbol, SymbolKind,
};

/// Helper: create a named test symbol
fn make_sym(name: &str, kind: SymbolKind) -> Symbol {
	Symbol::new(
		name.to_string(),
		kind,
		CodeLocation::new(
			PathBuf::from("src/lib.rs"),
			1,
			0,
			ByteSpan::ZERO,
		),
	)
}

#[test]
fn load_all_returns_empty_on_empty_index() {
	// Arrange
	let index = SearchIndex::in_memory().unwrap();

	// Act
	let symbols = index.load_all_symbols().unwrap();

	// Assert
	assert!(symbols.is_empty());
}

#[test]
fn load_all_returns_every_indexed_symbol() {
	// Arrange
	let index = SearchIndex::in_memory().unwrap();
	let input = vec![
		make_sym("alpha", SymbolKind::Function),
		make_sym("Beta", SymbolKind::Struct),
		make_sym("GAMMA", SymbolKind::Constant),
	];
	index.index_symbols(&input).unwrap();

	// Act
	let loaded = index.load_all_symbols().unwrap();

	// Assert
	assert_eq!(loaded.len(), 3);
	let names: Vec<&str> =
		loaded.iter().map(|s| s.name.as_str()).collect();
	assert!(names.contains(&"alpha"));
	assert!(names.contains(&"Beta"));
	assert!(names.contains(&"GAMMA"));
}

#[test]
fn load_all_preserves_symbol_kind() {
	// Arrange
	let index = SearchIndex::in_memory().unwrap();
	let input = vec![
		make_sym("my_fn", SymbolKind::Function),
		make_sym("MyStruct", SymbolKind::Struct),
	];
	index.index_symbols(&input).unwrap();

	// Act
	let loaded = index.load_all_symbols().unwrap();

	// Assert
	let fn_sym = loaded
		.iter()
		.find(|s| s.name == "my_fn")
		.expect("my_fn not found");
	assert_eq!(fn_sym.kind, SymbolKind::Function);

	let st_sym = loaded
		.iter()
		.find(|s| s.name == "MyStruct")
		.expect("MyStruct not found");
	assert_eq!(st_sym.kind, SymbolKind::Struct);
}
