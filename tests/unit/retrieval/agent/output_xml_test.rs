//! Tests for retrieval::agent::output_xml

use ch_cli::retrieval::agent::output::StructuredOutput;
use ch_cli::retrieval::agent::output_structured::CodeResult;
use ch_cli::retrieval::agent::output_xml::escape_xml;

#[test]
fn test_escape_xml() {
	assert_eq!(escape_xml("<test>"), "&lt;test&gt;");
	assert_eq!(escape_xml("a & b"), "a &amp; b");
	assert_eq!(
		escape_xml("\"quoted\""),
		"&quot;quoted&quot;",
	);
}

#[test]
fn test_structured_output_xml() {
	let mut output = StructuredOutput::new(
		"test query".to_string(),
		"Search".to_string(),
	);

	output.add_code(CodeResult {
		file: "src/test.rs".to_string(),
		symbol: "test_func".to_string(),
		kind: "fn".to_string(),
		line: 5,
		signature: None,
		full_content: "fn test_func() {}".to_string(),
		line_count: 10,
		truncated: false,
		relevance_score: 0.9,
	});

	let xml = output.to_xml();
	assert!(xml.contains("<retrieval_context>"));
	assert!(xml.contains("<code_context>"));
	assert!(xml.contains("test_func"));
}
