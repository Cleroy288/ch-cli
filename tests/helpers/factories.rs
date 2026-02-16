//! Test data factories for building domain objects.
//!
//! Use these builders to create test data with sane
//! defaults. Override fields per-test as needed.

use std::path::{Path, PathBuf};
use std::sync::atomic::{
	AtomicU64, Ordering,
};

use rustean::indexer::{
	CodeLocation, Symbol, SymbolKind, Visibility,
};

/// Atomic counter for unique test IDs
static TEST_COUNTER: AtomicU64 =
	AtomicU64::new(1);

/// Generate a unique test ID string
fn next_test_id() -> String {
	let n =
		TEST_COUNTER.fetch_add(1, Ordering::SeqCst);
	format!("test-{n}")
}

/// Create a test Symbol with sensible defaults
pub fn make_symbol(name: &str) -> Symbol {
	Symbol {
		name: name.to_string(),
		kind: SymbolKind::Function,
		location: make_location("src/main.rs", 1),
		doc_comment: None,
		visibility: Visibility::Public,
		signature: None,
		fqn: None,
		parent: None,
		content: None,
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
		byte_offset: 0,
		byte_length: 0,
	}
}

/// Create a temporary project directory for tests
pub fn make_test_dir(name: &str) -> PathBuf {
	let dir = std::env::temp_dir()
		.join("rustean-test")
		.join(name);
	std::fs::create_dir_all(&dir).ok();
	dir
}

/// Remove a temporary test directory
pub fn cleanup_test_dir(dir: &Path) {
	std::fs::remove_dir_all(dir).ok();
}

/// RAII guard that cleans up a temp directory
pub struct TempMemDir {
	/// Path to the temporary directory
	pub path: PathBuf,
}

impl TempMemDir {
	/// Create a new temp dir with unique suffix
	pub fn new(prefix: &str) -> Self {
		let id = next_test_id();
		let path = std::env::temp_dir()
			.join("rustean-mem-test")
			.join(format!("{prefix}-{id}"));
		std::fs::create_dir_all(&path).ok();
		Self { path }
	}
}

impl Drop for TempMemDir {
	fn drop(&mut self) {
		std::fs::remove_dir_all(&self.path).ok();
	}
}

/// Create a test Interaction with defaults
pub fn make_interaction(
	input_text: &str,
) -> rustean::domain::memory::Interaction {
	rustean::domain::memory::Interaction {
		id: next_test_id(),
		timestamp: 1000,
		session_id: "test-session".to_string(),
		input: make_user_input(input_text),
		response: make_ai_answer("test response"),
	}
}

/// Create an Interaction with custom input+response
pub fn make_interaction_with_text(
	input: &str,
	response: &str,
) -> rustean::domain::memory::Interaction {
	rustean::domain::memory::Interaction {
		id: next_test_id(),
		timestamp: 1000,
		session_id: "test-session".to_string(),
		input: make_user_input(input),
		response: make_ai_answer(response),
	}
}

/// Create a test UserInput
pub fn make_user_input(
	text: &str,
) -> rustean::domain::memory::UserInput {
	rustean::domain::memory::UserInput {
		text: text.to_string(),
		command: None,
		files: Vec::new(),
	}
}

/// Create an AiResponse::Answer
pub fn make_ai_answer(
	text: &str,
) -> rustean::domain::memory::AiResponse {
	rustean::domain::memory::AiResponse::Answer {
		text: text.to_string(),
	}
}

/// Create a temp dir for memory tests
pub fn make_memory_test_dir(
	name: &str,
) -> PathBuf {
	let id = next_test_id();
	let dir = std::env::temp_dir()
		.join("rustean-mem-test")
		.join(format!("{name}-{id}"));
	std::fs::create_dir_all(&dir).ok();
	dir
}

/// Create a test interaction with session ID
pub fn make_interaction_for_session(
	session_id: &str,
) -> rustean::domain::memory::Interaction {
	rustean::domain::memory::Interaction {
		id: next_test_id(),
		timestamp: 1000,
		session_id: session_id.to_string(),
		input: make_user_input("test query"),
		response: make_ai_answer("test answer"),
	}
}
