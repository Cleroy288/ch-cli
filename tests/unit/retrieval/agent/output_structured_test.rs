//! Tests for retrieval::agent::output_structured

use rustean::retrieval::agent::output_structured::{
	CodeResult, DocResult, NotesResult,
};

#[test]
fn test_code_result_xml() {
	let result = CodeResult {
		file: "src/main.rs".to_string(),
		symbol: "main".to_string(),
		kind: "fn".to_string(),
		line: 1,
		signature: Some("fn main()".to_string()),
		full_content: "fn main() {\n    \
			println!(\"Hello\");\n}"
			.to_string(),
		line_count: 3,
		truncated: false,
		relevance_score: 1.0,
	};

	let xml = result.to_xml(0);
	assert!(xml.contains("kind=\"fn\""));
	assert!(xml.contains("name=\"main\""));
	assert!(xml.contains("signature=\"fn main()\""));
	assert!(xml.contains("<![CDATA["));
}

#[test]
fn test_doc_result_to_xml() {
	let result = DocResult {
		file: "doc/auth.md".to_string(),
		section: "Login Flow".to_string(),
		content: "The login flow uses JWT tokens"
			.to_string(),
		relevance_score: 0.85,
	};

	let xml = result.to_xml(2);

	assert!(xml.contains("file=\"doc/auth.md\""));
	assert!(xml.contains("section=\"Login Flow\""));
	assert!(xml.contains("score=\"0.850\""));
	assert!(xml.contains("<![CDATA["));
	assert!(xml.contains("JWT tokens"));
	assert!(xml.contains("</doc>"));
}

#[test]
fn test_notes_result_to_xml() {
	let result = NotesResult {
		file: "notes/sprint1.md".to_string(),
		section: "Bug Fixes".to_string(),
		content: "Fixed null pointer in parser"
			.to_string(),
		relevance_score: 0.72,
	};

	let xml = result.to_xml(4);

	assert!(xml.contains("<note "));
	assert!(xml.contains(
		"file=\"notes/sprint1.md\""
	));
	assert!(xml.contains("section=\"Bug Fixes\""));
	assert!(xml.contains("score=\"0.720\""));
	assert!(xml.contains("<![CDATA["));
	assert!(xml.contains("</note>"));
}
