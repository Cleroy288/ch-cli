//! Tests for retrieval::docgen::generator_batch

use std::io::Write;
use std::path::PathBuf;

use tempfile::NamedTempFile;

use ch_cli::indexer::SymbolKind;
use ch_cli::retrieval::docgen::generator_utils::extract_code_snippet;
use ch_cli::retrieval::docgen::{DocEntry, DocGenerator};

#[test]
fn test_generate_from_prompt_no_model() {
	let mut generator = DocGenerator::new();
	let mut entry = DocEntry::new(
		"test_fn".to_string(),
		SymbolKind::Function,
		PathBuf::from("test.rs"),
		1,
	);

	let result = generator
		.generate_from_prompt(&mut entry, "test prompt");
	assert!(result.is_err());
}

/// Verify extract_code_snippet reads correct lines.
#[test]
fn test_extract_code_snippet() {
	let mut tmp = NamedTempFile::new().unwrap();
	let content = (1..=40)
		.map(|i| format!("// line {}", i))
		.collect::<Vec<_>>()
		.join("\n");
	write!(tmp, "{}", content).unwrap();

	let path = tmp.path();
	let snippet =
		extract_code_snippet(path, 20).unwrap();

	assert!(snippet.contains("// line 20"));
	assert!(
		snippet.contains("// line 15")
			|| snippet.contains("// line 16")
	);
	assert!(!snippet.is_empty());
}

/// Verify extract_code_snippet for out-of-range.
#[test]
fn test_extract_code_snippet_out_of_range() {
	let mut tmp = NamedTempFile::new().unwrap();
	write!(tmp, "a\nb\nc\nd\ne").unwrap();

	let path = tmp.path();
	let snippet =
		extract_code_snippet(path, 0).unwrap();
	assert!(snippet.is_empty());

	let snippet2 =
		extract_code_snippet(path, 100).unwrap();
	assert!(snippet2.is_empty());
}
