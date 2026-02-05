//! Structured Output Types
//!
//! Provides JSON-serializable output types for LLM consumption.
//! Separates code, doc, and notes context for clear presentation.

use serde::{Deserialize, Serialize};

/// Structured output for LLM consumption
/// Contains separate sections for code, docs, and notes
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StructuredOutput {
	/// original query
	pub query: String,
	/// detected intent (Search, Understand, FindDefinition, etc.)
	pub intent: String,
	/// code search results with full file content
	pub code_context: Vec<CodeResult>,
	/// documentation search results
	pub doc_context: Vec<DocResult>,
	/// notes search results
	pub notes_context: Vec<NotesResult>,
}

impl StructuredOutput {
	/// Create new structured output
	pub fn new(query: String, intent: String) -> Self {
		Self {
			query,
			intent,
			code_context: Vec::new(),
			doc_context: Vec::new(),
			notes_context: Vec::new(),
		}
	}

	/// Add code result
	pub fn add_code(&mut self, result: CodeResult) {
		self.code_context.push(result);
	}

	/// Add doc result
	pub fn add_doc(&mut self, result: DocResult) {
		self.doc_context.push(result);
	}

	/// Add notes result
	pub fn add_notes(&mut self, result: NotesResult) {
		self.notes_context.push(result);
	}

	/// Get total results count
	pub fn total_results(&self) -> usize {
		self.code_context.len() + self.doc_context.len() + self.notes_context.len()
	}

	/// Convert to JSON string
	pub fn to_json(&self) -> Result<String, serde_json::Error> {
		serde_json::to_string_pretty(self)
	}

	/// Convert to XML string
	pub fn to_xml(&self) -> String {
		let mut output = String::new();

		output.push_str("<retrieval_context>\n");
		output.push_str(&format!("  <query>{}</query>\n", escape_xml(&self.query)));
		output.push_str(&format!("  <intent>{}</intent>\n", &self.intent));

		// Code section
		output.push_str("  <code_context>\n");
		for result in &self.code_context {
			output.push_str(&result.to_xml(4));
		}
		output.push_str("  </code_context>\n");

		// Doc section
		output.push_str("  <doc_context>\n");
		for result in &self.doc_context {
			output.push_str(&result.to_xml(4));
		}
		output.push_str("  </doc_context>\n");

		// Notes section
		output.push_str("  <notes_context>\n");
		for result in &self.notes_context {
			output.push_str(&result.to_xml(4));
		}
		output.push_str("  </notes_context>\n");

		output.push_str("</retrieval_context>");
		output
	}
}

/// Code search result with full file content
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CodeResult {
	/// file path
	pub file: String,
	/// symbol name
	pub symbol: String,
	/// symbol kind (function, struct, etc.)
	pub kind: String,
	/// line number
	pub line: usize,
	/// function signature (if applicable)
	pub signature: Option<String>,
	/// full file content (up to 500 lines)
	pub full_content: String,
	/// total lines in file
	pub line_count: usize,
	/// whether content was truncated
	pub truncated: bool,
	/// relevance score
	pub relevance_score: f32,
}

impl CodeResult {
	/// Convert to XML string
	pub fn to_xml(&self, indent: usize) -> String {
		let spaces = " ".repeat(indent);
		let mut output = String::new();

		output.push_str(&format!(
			"{}<symbol kind=\"{}\" name=\"{}\" file=\"{}\" line=\"{}\"",
			spaces, &self.kind, escape_xml(&self.symbol), escape_xml(&self.file), self.line
		));

		if let Some(ref sig) = self.signature {
			output.push_str(&format!(" signature=\"{}\"", escape_xml(sig)));
		}

		output.push_str(&format!(
			" lines=\"{}\" truncated=\"{}\" score=\"{:.3}\">\n",
			self.line_count, self.truncated, self.relevance_score
		));

		output.push_str(&format!("{}  <content><![CDATA[\n", spaces));
		output.push_str(&self.full_content);
		output.push_str(&format!("\n{}  ]]></content>\n", spaces));
		output.push_str(&format!("{}</symbol>\n", spaces));

		output
	}
}

/// Documentation search result
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DocResult {
	/// file path
	pub file: String,
	/// section name
	pub section: String,
	/// section content
	pub content: String,
	/// relevance score
	pub relevance_score: f32,
}

impl DocResult {
	/// Convert to XML string
	pub fn to_xml(&self, indent: usize) -> String {
		let spaces = " ".repeat(indent);
		let mut output = String::new();

		output.push_str(&format!(
			"{}<doc file=\"{}\" section=\"{}\" score=\"{:.3}\">\n",
			spaces, escape_xml(&self.file), escape_xml(&self.section), self.relevance_score
		));
		output.push_str(&format!("{}  <content><![CDATA[\n", spaces));
		output.push_str(&self.content);
		output.push_str(&format!("\n{}  ]]></content>\n", spaces));
		output.push_str(&format!("{}</doc>\n", spaces));

		output
	}
}

/// Notes search result
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NotesResult {
	/// file path
	pub file: String,
	/// section name
	pub section: String,
	/// section content
	pub content: String,
	/// relevance score
	pub relevance_score: f32,
}

impl NotesResult {
	/// Convert to XML string
	pub fn to_xml(&self, indent: usize) -> String {
		let spaces = " ".repeat(indent);
		let mut output = String::new();

		output.push_str(&format!(
			"{}<note file=\"{}\" section=\"{}\" score=\"{:.3}\">\n",
			spaces, escape_xml(&self.file), escape_xml(&self.section), self.relevance_score
		));
		output.push_str(&format!("{}  <content><![CDATA[\n", spaces));
		output.push_str(&self.content);
		output.push_str(&format!("\n{}  ]]></content>\n", spaces));
		output.push_str(&format!("{}</note>\n", spaces));

		output
	}
}

/// Escape XML special characters
fn escape_xml(s: &str) -> String {
	s.replace('&', "&amp;")
		.replace('<', "&lt;")
		.replace('>', "&gt;")
		.replace('"', "&quot;")
		.replace('\'', "&apos;")
}

#[cfg(test)]
mod tests {
	use super::*;

	/// Test StructuredOutput creation and JSON serialization
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
			signature: Some("pub struct DaemonClient".to_string()),
			full_content: "struct DaemonClient { ... }".to_string(),
			line_count: 100,
			truncated: false,
			relevance_score: 0.95,
		});

		output.add_doc(DocResult {
			file: "doc/daemon.md".to_string(),
			section: "Overview".to_string(),
			content: "# Daemon\n\nThe daemon handles...".to_string(),
			relevance_score: 0.8,
		});

		let json = output.to_json().unwrap();
		assert!(json.contains("DaemonClient"));
		assert!(json.contains("Understand"));
	}

	/// Test StructuredOutput XML conversion
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

	/// Test total_results calculation
	#[test]
	fn test_total_results() {
		let mut output = StructuredOutput::new("q".to_string(), "i".to_string());

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

	/// Test XML escaping
	#[test]
	fn test_escape_xml() {
		assert_eq!(escape_xml("<test>"), "&lt;test&gt;");
		assert_eq!(escape_xml("a & b"), "a &amp; b");
		assert_eq!(escape_xml("\"quoted\""), "&quot;quoted&quot;");
	}

	/// Test CodeResult XML
	#[test]
	fn test_code_result_xml() {
		let result = CodeResult {
			file: "src/main.rs".to_string(),
			symbol: "main".to_string(),
			kind: "fn".to_string(),
			line: 1,
			signature: Some("fn main()".to_string()),
			full_content: "fn main() {\n    println!(\"Hello\");\n}".to_string(),
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
}
