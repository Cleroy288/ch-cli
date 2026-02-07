//! Test data factories for building domain objects.
//!
//! Use these builders to create test data with sane
//! defaults. Override fields per-test as needed.

use std::path::{Path, PathBuf};

use ch_cli::indexer::{
	CodeLocation, Symbol, SymbolKind, Visibility,
};

/// Create a test Symbol with sensible defaults
pub fn make_symbol(name: &str) -> Symbol {
	Symbol {
		name: name.to_string(),
		kind: SymbolKind::Function,
		location: make_location("src/main.rs", 1),
		doc_comment: None,
		visibility: Visibility::Public,
		signature: None,
		body: None,
	}
}

/// Create a test Symbol with a specific kind
pub fn make_symbol_with_kind(
	name: &str,
	kind: SymbolKind,
) -> Symbol {
	let mut sym = make_symbol(name);
	sym.kind = kind;
	sym
}

/// Create a test CodeLocation
pub fn make_location(
	file: &str,
	line: usize,
) -> CodeLocation {
	CodeLocation {
		file: PathBuf::from(file),
		line,
		column: 0,
		end_line: Some(line + 5),
		end_column: Some(0),
	}
}

/// Create a temporary project directory for tests
pub fn make_test_dir(name: &str) -> PathBuf {
	let dir = std::env::temp_dir()
		.join("ch-cli-test")
		.join(name);
	std::fs::create_dir_all(&dir).ok();
	dir
}

/// Remove a temporary test directory
pub fn cleanup_test_dir(dir: &Path) {
	std::fs::remove_dir_all(dir).ok();
}
