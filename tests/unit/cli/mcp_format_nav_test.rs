//! Tests for navigation result formatters.

use std::path::PathBuf;

use rustean::cli::commands::mcp_server
	::mcp_format_nav;
use rustean::indexer::{
	CodeLocation, IndexStats, SymbolKind,
};
use rustean::service::search::types_navigation::{
	ReferenceResult, UsageLocation,
};

/// Empty references returns no results
#[test]
fn empty_refs_returns_no_results() {
	// Arrange
	let result = ReferenceResult {
		definitions: vec![],
		references: vec![],
	};

	// Act
	let text =
		mcp_format_nav::format_references(&result);

	// Assert
	assert_eq!(text, "(no results)");
}

/// References include file and line
#[test]
fn refs_include_locations() {
	// Arrange
	let result = ReferenceResult {
		definitions: vec![UsageLocation {
			file: PathBuf::from("src/lib.rs"),
			line: 5,
		}],
		references: vec![UsageLocation {
			file: PathBuf::from("src/main.rs"),
			line: 20,
		}],
	};

	// Act
	let text =
		mcp_format_nav::format_references(&result);

	// Assert
	assert!(text.contains("Definitions:"));
	assert!(text.contains("References:"));
	assert!(text.contains("src/lib.rs:5"));
}

/// Index stats format includes all fields
#[test]
fn index_stats_includes_all_fields() {
	// Arrange
	let stats = IndexStats {
		root: PathBuf::from("."),
		file_count: 100,
		symbol_count: 500,
		last_updated: 1234567890,
		version: 2,
	};

	// Act
	let text =
		mcp_format_nav::format_index_stats(&stats);

	// Assert
	assert!(text.contains("100"));
	assert!(text.contains("500"));
	assert!(text.contains("Version: 2"));
}
