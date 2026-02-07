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

		// Code section
		out.push_str("  <code_context>\n");
		for result in &self.code_context {
			out.push_str(&result.to_xml(4));
		}
		out.push_str("  </code_context>\n");

		// Doc section
		out.push_str("  <doc_context>\n");
		for result in &self.doc_context {
			out.push_str(&result.to_xml(4));
		}
		out.push_str("  </doc_context>\n");

		// Notes section
		out.push_str("  <notes_context>\n");
		for result in &self.notes_context {
			out.push_str(&result.to_xml(4));
		}
		out.push_str("  </notes_context>\n");

		out.push_str("</retrieval_context>");
		out
	}
}

/// Escape XML special characters
pub fn escape_xml(s: &str) -> String {
	s.replace('&', "&amp;")
		.replace('<', "&lt;")
		.replace('>', "&gt;")
		.replace('"', "&quot;")
		.replace('\'', "&apos;")
}

