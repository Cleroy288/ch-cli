//! Document type classification for scoring and routing purposes.

use serde::{Deserialize, Serialize};

use crate::retrieval::daemon::protocol::QueryIntent;

/// Document type for scoring purposes
/// Used to boost source code results over notes/benchmarks
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum DocumentType {
	/// Source code (.rs, .py, .js, etc.)
	SourceCode,
	/// Official documentation (doc/*.md)
	Documentation,
	/// Implementation notes (notes/*.md)
	Notes,
	/// Benchmark/test notes (notes/benchmarks/*.md)
	Benchmark,
	/// Test files (*_test.rs, tests/*.rs)
	Test,
}

/// Check if a path string represents a test file
/// Detects: /tests/ directory, _test.rs suffix, test_ prefix, _tests.rs suffix
pub fn is_test_file(path_str: &str) -> bool {
	// tests directory pattern
	if path_str.contains("/tests/") {
		return true;
	}

	// _test.rs suffix pattern (e.g., parser_test.rs)
	if path_str.ends_with("_test.rs") {
		return true;
	}

	// _tests.rs suffix pattern (e.g., parser_tests.rs)
	if path_str.ends_with("_tests.rs") {
		return true;
	}

	// test_ prefix pattern (e.g., /src/test_parser.rs)
	// Check for /test_ to avoid matching paths like "latest_feature.rs"
	if path_str.contains("/test_") {
		return true;
	}

	false
}

impl DocumentType {
	/// Classify a file path into a document type
	pub fn from_path(path: &std::path::Path) -> Self {
		let path_str = path.to_string_lossy(); // path as string for pattern matching
		let extension = path // file extension
			.extension()
			.and_then(|e| e.to_str())
			.unwrap_or("");

		// Check benchmarks first (most specific)
		if path_str.contains("/benchmarks/") {
			return Self::Benchmark;
		}

		// Check notes
		if path_str.contains("/notes/") {
			return Self::Notes;
		}

		// Check documentation
		if path_str.contains("/doc/") && extension == "md" {
			return Self::Documentation;
		}

		// Check test files - multiple patterns
		// Matches: /tests/ directory, _test.rs suffix, test_ prefix, _tests.rs suffix
		if is_test_file(&path_str) {
			return Self::Test;
		}

		// Check source code extensions
		match extension {
			"rs" | "py" | "js" | "ts" | "go" | "java" | "c"
			| "cpp" | "h" => Self::SourceCode,
			"md" | "txt" => Self::Documentation,
			_ => Self::SourceCode, // default to source
		}
	}

	/// Get the boost factor for this document type
	/// Higher values = higher priority in search results
	pub fn boost_factor(&self) -> f32 {
		match self {
			Self::SourceCode => 1.5,    // Highest priority
			Self::Documentation => 1.0, // Normal priority
			Self::Test => 0.8, // tests are examples
			Self::Notes => 0.7,         // Lower priority
			Self::Benchmark => 0.3,     // Much lower priority
		}
	}

	/// Get boost factor adjusted for query context.
	/// Lowers documentation boost for implementation queries.
	pub fn boost_factor_for_query(&self, query: &str) -> f32 {
		// base boost for this document type
		let base_boost = self.boost_factor();
		// lowercase query for matching
		let lower_query = query.to_lowercase();

		// Keywords indicating user wants code, not docs
		let impl_keywords = [
			"implementation",
			"algorithm",
			"logic",
			"code",
			"function",
			"method",
			"work",
			"works",
			"working",
			"implement",
			"source",
			"actual",
		];
		// query wants code
		let wants_code = impl_keywords
			.iter()
			.any(|k| lower_query.contains(k));

		// If query wants code, reduce boost for non-code
		if wants_code {
			match self {
				Self::Documentation | Self::Notes
				| Self::Benchmark => base_boost * 0.5,
				// unchanged for code
				Self::SourceCode | Self::Test => base_boost,
			}
		} else {
			base_boost // no adjustment
		}
	}

	/// Get boost factor adjusted for query intent.
	/// For Understand intent, reduce doc/test boost.
	pub fn boost_factor_for_intent(
		&self,
		intent: &QueryIntent,
	) -> f32 {
		// base boost for this document type
		let base_boost = self.boost_factor();

		match intent {
			QueryIntent::Understand => {
				// users want implementation code, not tests/docs
				match self {
					// strongly boost source code
					Self::SourceCode => base_boost * 3.0,
					// almost eliminate docs
					Self::Documentation | Self::Notes => {
						base_boost * 0.1
					}
					// eliminate benchmarks
					Self::Benchmark => base_boost * 0.05,
					// almost eliminate tests
					Self::Test => base_boost * 0.1,
				}
			}
			QueryIntent::FindDefinition => {
				// users want actual definition, not tests/docs
				match self {
					// boost source code
					Self::SourceCode => base_boost * 2.0,
					// reduce docs
					Self::Documentation => base_boost * 0.3,
					// reduce notes
					Self::Notes => base_boost * 0.2,
					// eliminate benchmarks
					Self::Benchmark => base_boost * 0.1,
					// reduce tests significantly
					Self::Test => base_boost * 0.2,
				}
			}
			_ => base_boost, // no adjustment for other intents
		}
	}
}
