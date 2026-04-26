//! Integration tests: index then search.
//!
//! Exercises the real IndexService → SearchService
//! cross-layer workflow with keyword search.

use rustean::indexer::SymbolKind;
use rustean::service::index::types::IndexOptions;
use rustean::service::search::types::{
	SearchFlags, SearchOptions,
};
use rustean::service::{
	DefaultIndexService, DefaultSearchService,
	IndexService, SearchService,
};

use crate::helpers::factories_service::{
	cleanup_project, make_rust_project,
};

/// Index opts: semantic, no persistence
fn idx_opts() -> IndexOptions {
	IndexOptions::default()
}

/// Search opts: basic keyword search
fn search_opts() -> SearchOptions {
	SearchOptions {
		limit: 10,
		kind: None,
		flags: SearchFlags::default(),
	}
}

#[test]
fn index_then_basic_search_finds_symbols() {
	// Arrange
	let dir =
		make_rust_project("search_basic");
	let idx = DefaultIndexService::default();
	idx.index_project(&dir, &idx_opts())
		.expect("index_project failed");

	// Act — search for "greet" by name
	let search = DefaultSearchService::new();
	let got = search.search(
		"greet",
		&dir,
		&search_opts(),
	);

	// Assert
	let hits =
		got.expect("search failed").hits;
	let found = hits
		.iter()
		.any(|h| h.symbol.name == "greet");
	assert!(
		found,
		"expected 'greet' in search results",
	);

	cleanup_project(&dir);
}

#[test]
fn index_then_search_by_kind_filters() {
	// Arrange
	let dir =
		make_rust_project("search_kind");
	let idx = DefaultIndexService::default();
	let _ = idx.index_project(&dir, &idx_opts());

	// Act — filter by Struct kind
	let search = DefaultSearchService::new();
	let opts = SearchOptions {
		kind: Some(SymbolKind::Struct),
		..search_opts()
	};
	let got =
		search.search("Calculator", &dir, &opts);

	// Assert
	let hits =
		got.expect("search failed").hits;
	for h in &hits {
		assert_eq!(h.symbol.kind, SymbolKind::Struct);
	}

	cleanup_project(&dir);
}

#[test]
fn index_then_index_symbols_searchable() {
	// Arrange
	let dir =
		make_rust_project("search_idx_sym");
	let idx = DefaultIndexService::default();
	let result =
		idx.index_project(&dir, &idx_opts());
	let symbols = result.unwrap().symbols;
	assert!(!symbols.is_empty());

	// Act — feed symbols into search engine
	let search = DefaultSearchService::new();
	let count =
		search.index_symbols(&symbols);

	// Assert
	let n =
		count.expect("index_symbols failed");
	assert!(n > 0, "expected >0 symbols indexed");

	cleanup_project(&dir);
}

#[test]
fn index_then_search_no_match_empty() {
	// Arrange
	let dir =
		make_rust_project("search_nomatch");
	let idx = DefaultIndexService::default();
	let _ = idx.index_project(&dir, &idx_opts());

	// Act — search nonsense term
	let search = DefaultSearchService::new();
	let got = search.search(
		"zzz_nonexistent_xyz",
		&dir,
		&search_opts(),
	);

	// Assert
	let hits =
		got.expect("search failed").hits;
	assert!(hits.is_empty());

	cleanup_project(&dir);
}
