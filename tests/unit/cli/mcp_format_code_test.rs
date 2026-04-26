//! Tests for code search result formatters.

use std::path::PathBuf;

use rustean::cli::commands::mcp_server
	::mcp_format_code;
use rustean::indexer::{CodeLocation, SymbolKind};
use rustean::service::search::types::{
	CallerHit, SearchResultHit,
};
use rustean::service::search::types_navigation::{
	DefinitionHit, SymbolEntry,
};

/// Helper: build a test symbol
fn make_symbol(name: &str) -> rustean::indexer::Symbol {
	rustean::indexer::Symbol {
		name: name.to_string(),
		kind: SymbolKind::Function,
		location: CodeLocation {
			file: PathBuf::from("src/main.rs"),
			line: 10,
			column: 0,
			byte_offset: 0,
			byte_length: 0,
		},
		visibility: rustean::indexer::Visibility::Public,
		signature: None,
		doc_comment: None,
		fqn: None,
		parent: None,
		content: None,
	}
}

/// Empty hits returns no results
#[test]
fn empty_hits_returns_no_results() {
	// Arrange / Act
	let text =
		mcp_format_code::format_search_hits(&[]);

	// Assert
	assert_eq!(text, "(no results)");
}

/// Non-empty hits include score and name
#[test]
fn hits_include_score_and_name() {
	// Arrange
	let hits = vec![SearchResultHit {
		symbol: make_symbol("my_func"),
		score: 0.85,
	}];

	// Act
	let text =
		mcp_format_code::format_search_hits(&hits);

	// Assert
	assert!(text.contains("0.850"));
	assert!(text.contains("my_func"));
}

/// Empty definitions returns no results
#[test]
fn empty_defs_returns_no_results() {
	// Arrange / Act
	let text =
		mcp_format_code::format_definitions(&[]);

	// Assert
	assert_eq!(text, "(no results)");
}

/// Callers format includes file and line
#[test]
fn callers_include_file_and_line() {
	// Arrange
	let callers = vec![CallerHit {
		file: PathBuf::from("src/lib.rs"),
		line: 42,
		context: "call_site()".to_string(),
		caller_name: Some("main".to_string()),
	}];

	// Act
	let text =
		mcp_format_code::format_callers(&callers);

	// Assert
	assert!(text.contains("main"));
	assert!(text.contains("42"));
	assert!(text.contains("src/lib.rs"));
}

/// Symbol list format includes fqn and kind
#[test]
fn symbol_list_includes_fqn() {
	// Arrange
	let entries = vec![SymbolEntry {
		symbol: make_symbol("foo"),
		fqn: "crate::foo".to_string(),
	}];

	// Act
	let text =
		mcp_format_code::format_symbol_list(
			&entries,
		);

	// Assert
	assert!(text.contains("crate::foo"));
	assert!(text.contains("[fn]"));
}
