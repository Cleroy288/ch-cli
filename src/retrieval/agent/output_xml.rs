//! XML Output Formatting
//!
//! Provides XML conversion for StructuredOutput and the
//! escape_xml utility used across the agent module.

use super::output::StructuredOutput;

impl StructuredOutput {
	/// Convert to XML string
	pub fn to_xml(&self) -> String {
		let mut out = String::new();

		out.push_str("<retrieval_context>\n");
		out.push_str(&format!(
			"  <query>{}</query>\n",
			escape_xml(&self.query),
		));
		out.push_str(&format!(
			"  <intent>{}</intent>\n",
			&self.intent,
		));

		append_section(
			&mut out, &self.code_context,
			&self.doc_context, &self.notes_context,
		);

		out.push_str("</retrieval_context>");
		out
	}
}

/// Append code, doc, and notes sections to output
fn append_section(
	out: &mut String,
	code: &[super::output_structured::CodeResult],
	docs: &[super::output_structured::DocResult],
	notes: &[super::output_structured::NotesResult],
) {
	out.push_str("  <code_context>\n");
	for result in code {
		out.push_str(&result.to_xml(4));
	}
	out.push_str("  </code_context>\n");

	out.push_str("  <doc_context>\n");
	for result in docs {
		out.push_str(&result.to_xml(4));
	}
	out.push_str("  </doc_context>\n");

	out.push_str("  <notes_context>\n");
	for result in notes {
		out.push_str(&result.to_xml(4));
	}
	out.push_str("  </notes_context>\n");
}

/// Escape XML special characters
pub fn escape_xml(text: &str) -> String {
	text.replace('&', "&amp;")
		.replace('<', "&lt;")
		.replace('>', "&gt;")
		.replace('"', "&quot;")
		.replace('\'', "&apos;")
}

