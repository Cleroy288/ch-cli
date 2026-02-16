//! Unit tests for DocBrowser

use rustean::picker::doc_browser::{
	DocBrowser, DocBrowserEntry,
};

/// Helper: build a test DocBrowserEntry
fn make_entry(
	name: &str,
	kind: &str,
	path: &str,
) -> DocBrowserEntry {
	DocBrowserEntry {
		name: name.to_string(),
		kind: kind.to_string(),
		rel_path: path.to_string(),
		line: 1,
		llm_doc: Some(format!("Doc for {}", name)),
	}
}

/// Helper: build a set of test entries
fn test_entries() -> Vec<DocBrowserEntry> {
	vec![
		make_entry("run", "Function", "src/app/mod.rs"),
		make_entry("App", "Struct", "src/app/mod.rs"),
		make_entry("main", "Function", "src/main.rs"),
		make_entry("parse", "Function", "src/cli/mod.rs"),
	]
}

// -- new --

/// Entries are sorted by path then name
#[test]
fn new_sorts_entries_by_path_then_name() {
	let browser = DocBrowser::new(test_entries());
	let items = browser.current_items("");
	let names: Vec<&str> =
		items.iter().map(|e| e.name.as_str()).collect();

	// app/mod.rs entries first (App, run), then cli, main
	assert_eq!(names, vec!["App", "run", "parse", "main"]);
}

// -- current_items --

/// Empty query returns all entries
#[test]
fn current_items_empty_query_returns_all() {
	let browser = DocBrowser::new(test_entries());
	let items = browser.current_items("");
	assert_eq!(items.len(), 4);
}

/// Query filters by name
#[test]
fn current_items_filters_by_name() {
	let browser = DocBrowser::new(test_entries());
	let items = browser.current_items("parse");
	assert_eq!(items.len(), 1);
	assert_eq!(items[0].name, "parse");
}

/// Query filters by path
#[test]
fn current_items_filters_by_path() {
	let browser = DocBrowser::new(test_entries());
	let items = browser.current_items("cli");
	assert_eq!(items.len(), 1);
	assert_eq!(items[0].name, "parse");
}

/// Query is case-insensitive
#[test]
fn current_items_case_insensitive() {
	let browser = DocBrowser::new(test_entries());
	let items = browser.current_items("PARSE");
	assert_eq!(items.len(), 1);
	assert_eq!(items[0].name, "parse");
}

// -- entry_at --

/// entry_at returns correct entry from filtered list
#[test]
fn entry_at_returns_correct_entry() {
	let browser = DocBrowser::new(test_entries());
	let entry = browser.entry_at("", 2);
	assert!(entry.is_some());
	assert_eq!(entry.unwrap().name, "parse");
}

/// entry_at returns None for out of bounds
#[test]
fn entry_at_out_of_bounds_returns_none() {
	let browser = DocBrowser::new(test_entries());
	let entry = browser.entry_at("", 99);
	assert!(entry.is_none());
}

// -- len / is_empty --

/// len returns entry count
#[test]
fn len_returns_count() {
	let browser = DocBrowser::new(test_entries());
	assert_eq!(browser.len(), 4);
}

/// is_empty returns true for empty browser
#[test]
fn is_empty_when_no_entries() {
	let browser = DocBrowser::new(Vec::new());
	assert!(browser.is_empty());
}

/// is_empty returns false when entries exist
#[test]
fn is_empty_false_when_entries_exist() {
	let browser = DocBrowser::new(test_entries());
	assert!(!browser.is_empty());
}
