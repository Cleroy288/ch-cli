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

/// Remove a temporary test directory + its
/// centralized data dir
pub fn cleanup_test_dir(dir: &Path) {
	// Resolve centralized path BEFORE deleting
	// (canonicalize needs path to exist)
	let data =
		rustean::domain::data_paths::data_dir(dir);
	std::fs::remove_dir_all(dir).ok();
	std::fs::remove_dir_all(&data).ok();
}

/// RAII guard that cleans up a temp directory
pub struct TempMemDir {
	/// Path to the temporary directory
	pub path: PathBuf,
}

impl TempMemDir {
	/// Create a new temp dir with unique suffix.
	/// Pre-cleans stale centralized data from prior
	/// failed runs.
	pub fn new(prefix: &str) -> Self {
		let id = next_test_id();
		let path = std::env::temp_dir()
			.join("rustean-mem-test")
			.join(format!("{prefix}-{id}"));
		std::fs::create_dir_all(&path).ok();
		// Pre-clean centralized dir (stale data
		// from prior panicked tests)
		let data =
			rustean::domain::data_paths::data_dir(
				&path,
			);
		std::fs::remove_dir_all(&data).ok();
		Self { path }
	}
}

impl Drop for TempMemDir {
	fn drop(&mut self) {
		// Resolve centralized path BEFORE deleting
		// temp dir (canonicalize needs the path to
		// exist on macOS: /tmp → /private/tmp)
		let data =
			rustean::domain::data_paths::data_dir(
				&self.path,
			);
		std::fs::remove_dir_all(&self.path).ok();
		std::fs::remove_dir_all(&data).ok();
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

/// Create a temp dir for memory tests.
/// Pre-cleans stale centralized data from prior
/// runs to avoid cross-run pollution.
pub fn make_memory_test_dir(
	name: &str,
) -> PathBuf {
	let id = next_test_id();
	let dir = std::env::temp_dir()
		.join("rustean-mem-test")
		.join(format!("{name}-{id}"));
	std::fs::create_dir_all(&dir).ok();
	// Pre-clean centralized dir (stale data
	// from prior failed runs)
	let data =
		rustean::domain::data_paths::data_dir(&dir);
	std::fs::remove_dir_all(&data).ok();
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
