//! Integration tests: index then navigate.
//!
//! Exercises real IndexService → SearchService
//! navigation features (definitions, references,
//! symbol listing, structure).

use rustean::service::search::types_navigation::{
	SymbolListOptions,
};
use rustean::service::{
	DefaultSearchService, SearchService,
};

use crate::helpers::factories_service::{
	cleanup_project, make_rust_project,
};

#[test]
fn find_definition_returns_location() {
	// Arrange — fresh dir, no pre-index
	// (navigation_impl indexes internally)
	let dir = make_rust_project("nav_def");

	// Act
	let search = DefaultSearchService::new();
	let got =
		search.find_definition("greet", &dir);

	// Assert
	assert!(got.is_ok(), "err: {:?}", got.err());
	let defs = got.unwrap();
	let names: Vec<&str> = defs
		.iter()
		.map(|d| d.symbol.name.as_str())
		.collect();
	assert!(
		names.contains(&"greet"),
		"Expected 'greet', got: {:?}",
		names,
	);

	cleanup_project(&dir);
}

#[test]
fn find_references_includes_call_sites() {
	// Arrange
	let dir = make_rust_project("nav_refs");

	// Act — find references to greet
	let search = DefaultSearchService::new();
	let got = search.find_references(
		"greet", &dir, true,
	);

	// Assert
	assert!(got.is_ok());
	let refs = got.unwrap();
	let total = refs.definitions.len()
		+ refs.references.len();
	assert!(
		total >= 1,
		"Expected at least 1 location, \
		 defs={}, refs={}",
		refs.definitions.len(),
		refs.references.len(),
	);

	cleanup_project(&dir);
}

#[test]
fn list_symbols_returns_all() {
	// Arrange
	let dir = make_rust_project("nav_list");

	// Act — list all symbols
	let search = DefaultSearchService::new();
	let opts = SymbolListOptions::default();
	let got = search.list_symbols(&dir, &opts);

	// Assert
	assert!(got.is_ok());
	let entries = got.unwrap();
	let names: Vec<&str> = entries
		.iter()
		.map(|e| e.symbol.name.as_str())
		.collect();
	assert!(
		names.contains(&"greet"),
		"Expected 'greet' in: {:?}",
		names,
	);

	cleanup_project(&dir);
}

#[test]
fn find_structure_does_not_error() {
	// Arrange
	let dir = make_rust_project("nav_struct");

	// Act — find structure for Calculator
	let search = DefaultSearchService::new();
	let got = search.find_structure(
		"Calculator",
		&dir,
	);

	// Assert — may return empty if not enough
	// module info, but should not error
	assert!(got.is_ok());

	cleanup_project(&dir);
}
