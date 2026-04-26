use serde::{Deserialize, Serialize};

#[derive(
	Debug, Clone, Copy, PartialEq, Eq,
	Serialize, Deserialize,
)]
pub enum DocumentType {
	SourceCode,
	Documentation,
	Notes,
	Benchmark,
	Test,
}

pub fn is_test_file(path_str: &str) -> bool {
	path_str.contains("/tests/")
		|| path_str.ends_with("_test.rs")
		|| path_str.ends_with("_tests.rs")
		|| path_str.contains("/test_")
}

impl DocumentType {
	pub fn from_path(path: &std::path::Path) -> Self {
		let path_str = path.to_string_lossy();
		let ext = path
			.extension()
			.and_then(|e| e.to_str())
			.unwrap_or("");

		if path_str.contains("/benchmarks/") {
			return Self::Benchmark;
		}
		if path_str.contains("/notes/") {
			return Self::Notes;
		}
		if path_str.contains("/doc/") && ext == "md" {
			return Self::Documentation;
		}
		if is_test_file(&path_str) {
			return Self::Test;
		}

		match ext {
			"rs" | "py" | "js" | "ts" | "go"
			| "java" | "c" | "cpp" | "h" => {
				Self::SourceCode
			}
			"md" | "txt" => Self::Documentation,
			_ => Self::SourceCode,
		}
	}
}
