//! Tests for retrieval::agent::output

use rustean::retrieval::agent::output::StructuredOutput;
use rustean::retrieval::agent::output_structured::{
	CodeResult, DocResult, NotesResult,
};

#[test]
fn test_structured_output_json() {
	let mut output = StructuredOutput::new(
		"how does daemon work".to_string(),
		"Understand".to_string(),
	);

	output.add_code(CodeResult {
		file: "src/daemon.rs".to_string(),
		symbol: "DaemonClient".to_string(),
		kind: "struct".to_string(),
		line: 10,
		signature: Some(
			"pub struct DaemonClient".to_string(),
		),
		full_content: "struct DaemonClient { ... }"
			.to_string(),
		line_count: 100,
		truncated: false,
		relevance_score: 0.95,
	});

	output.add_doc(DocResult {
		file: "doc/daemon.md".to_string(),
		section: "Overview".to_string(),
		content: "# Daemon\n\nThe daemon handles..."
			.to_string(),
		relevance_score: 0.8,
	});

	let json = output.to_json().unwrap();
	assert!(json.contains("DaemonClient"));
	assert!(json.contains("Understand"));
}

#[test]
fn test_total_results() {
	let mut output = StructuredOutput::new(
		"q".to_string(),
		"i".to_string(),
	);

	assert_eq!(output.total_results(), 0);

	output.add_code(CodeResult {
		file: "a.rs".to_string(),
		symbol: "a".to_string(),
		kind: "fn".to_string(),
		line: 1,
		signature: None,
		full_content: "".to_string(),
		line_count: 1,
		truncated: false,
		relevance_score: 0.5,
	});

	output.add_doc(DocResult {
		file: "b.md".to_string(),
		section: "s".to_string(),
		content: "".to_string(),
		relevance_score: 0.5,
	});

	output.add_notes(NotesResult {
		file: "c.md".to_string(),
		section: "s".to_string(),
		content: "".to_string(),
		relevance_score: 0.5,
	});

	assert_eq!(output.total_results(), 3);
}

#[test]
fn test_add_code() {
	let mut output = StructuredOutput::new(
		"q".to_string(),
		"Search".to_string(),
	);

	assert_eq!(output.code_context.len(), 0);

	output.add_code(CodeResult {
		file: "src/main.rs".to_string(),
		symbol: "main".to_string(),
		kind: "fn".to_string(),
		line: 1,
		signature: None,
		full_content: "fn main() {}".to_string(),
		line_count: 3,
		truncated: false,
		relevance_score: 0.9,
	});

	assert_eq!(output.code_context.len(), 1);
	assert_eq!(output.code_context[0].symbol, "main");
}

#[test]
fn test_add_doc() {
	let mut output = StructuredOutput::new(
		"q".to_string(),
		"Search".to_string(),
	);

	assert_eq!(output.doc_context.len(), 0);

	output.add_doc(DocResult {
		file: "doc/intro.md".to_string(),
		section: "Getting Started".to_string(),
		content: "Welcome".to_string(),
		relevance_score: 0.8,
	});

	assert_eq!(output.doc_context.len(), 1);
	assert_eq!(
		output.doc_context[0].section,
		"Getting Started",
	);
}

#[test]
fn test_add_notes() {
	let mut output = StructuredOutput::new(
		"q".to_string(),
		"Search".to_string(),
	);

	assert_eq!(output.notes_context.len(), 0);

	output.add_notes(NotesResult {
		file: "notes/day1.md".to_string(),
		section: "Progress".to_string(),
		content: "Implemented feature X".to_string(),
		relevance_score: 0.7,
	});

	assert_eq!(output.notes_context.len(), 1);
	assert_eq!(
		output.notes_context[0].section,
		"Progress",
	);
}
