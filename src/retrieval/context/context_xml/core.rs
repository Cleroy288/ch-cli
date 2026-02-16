//! Core XML Formatting for ContextualBlock
//!
//! Provides to_xml and token_count methods.

use super::super::ContextualBlock;
use super::formatters::{
	format_callees_xml, format_callers_xml,
	format_parent_xml, format_related_types_xml,
	format_symbol_xml,
};
use super::super::context_xml_usages::format_usages_xml;

impl ContextualBlock {
	/// Format the block as XML for LLM consumption
	pub fn to_xml(&self) -> String {
		let mut xml = String::new();

		xml.push_str("<context-block>\n");
		xml.push_str(&format_symbol_xml(self));
		xml.push_str(&format_parent_xml(self));
		xml.push_str(&format_related_types_xml(self));
		xml.push_str(&format_callers_xml(self));
		xml.push_str(&format_callees_xml(self));
		xml.push_str(&format_usages_xml(&self.usages));
		xml.push_str("</context-block>");

		xml
	}

	/// Get approximate token count for the block
	pub fn token_count(&self) -> usize {
		let usage_chars: usize = self
			.usages
			.iter()
			.map(|usage| {
				usage
					.snippet
					.as_ref()
					.map(|snip| snip.len())
					.unwrap_or(50)
			})
			.sum();

		let doc_len = self
			.doc_comment
			.as_ref()
			.map(|doc| doc.len())
			.unwrap_or(0);

		let total = self.code_snippet.len()
			+ doc_len
			+ self.symbol.name.len() * 2
			+ self.callers.len() * 20
			+ self.callees.len() * 20
			+ usage_chars;

		total / 4
	}
}

/// Escape XML special characters
pub fn escape_xml(text: &str) -> String {
	text.replace('&', "&amp;")
		.replace('<', "&lt;")
		.replace('>', "&gt;")
		.replace('"', "&quot;")
		.replace('\'', "&apos;")
}
