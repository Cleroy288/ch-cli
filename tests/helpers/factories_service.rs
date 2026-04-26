//! Service-level test data factories.
//!
//! Builds SearchOptions, SearchResult, and other
//! service DTOs with sensible test defaults.

use std::fs;
use std::path::{Path, PathBuf};

use rustean::service::search::boost::QueryIntent;
use rustean::service::index::types::IndexOptions;
use rustean::service::search::types::{
	SearchOptions, SearchResult,
};

/// Create default SearchOptions for tests
pub fn make_search_options() -> SearchOptions {
	SearchOptions {
		limit: 10,
		kind: None,
		flags: rustean::service::search::types
			::SearchFlags::default(),
	}
}

/// Create an empty SearchResult for tests
pub fn make_search_result() -> SearchResult {
	SearchResult {
		hits: vec![],
		intent: QueryIntent::Search,
	}
}

/// Create default IndexOptions for tests
pub fn make_index_options() -> IndexOptions {
	IndexOptions::default()
}

/// Create a temp dir with a small Rust project
///
/// Contains src/lib.rs with a greeting function,
/// a Calculator struct with methods, and a caller
/// function that exercises cross-references.
pub fn make_rust_project(
	name: &str,
) -> PathBuf {
	let dir = std::env::temp_dir().join(format!(
		"rustean-integ_{}_{}",
		name,
		std::process::id(),
	));
	let _ = fs::remove_dir_all(&dir);
	let src = dir.join("src");
	fs::create_dir_all(&src)
		.expect("create src dir");
	fs::write(src.join("lib.rs"), LIB_RS)
		.expect("write lib.rs");
	dir
}

/// Clean up a project created by make_rust_project
pub fn cleanup_project(dir: &Path) {
	let _ = fs::remove_dir_all(dir);
}

/// Sample Rust source for integration tests
const LIB_RS: &str = r#"//! Sample library

/// Greet someone by name
pub fn greet(name: &str) -> String {
    format!("Hello, {}!", name)
}

/// Basic calculator
pub struct Calculator {
    /// current value
    pub value: i32,
}

impl Calculator {
    /// Create calculator with initial value
    pub fn new(initial: i32) -> Self {
        Self { value: initial }
    }

    /// Add a number
    pub fn add(&mut self, n: i32) {
        self.value += n;
    }
}

/// Uses greet and Calculator together
pub fn run_demo() -> String {
    let msg = greet("world");
    let mut calc = Calculator::new(0);
    calc.add(42);
    format!("{} result={}", msg, calc.value)
}
"#;
