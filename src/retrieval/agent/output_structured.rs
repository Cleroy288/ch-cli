//! Structured Result Types
//!
//! CodeResult, DocResult, and NotesResult types used by
//! StructuredOutput for code, documentation, and notes context.

use serde::{Deserialize, Serialize};

use super::output_xml::escape_xml;

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
	/// Convert to XML string with given indentation
	pub fn to_xml(&self, indent: usize) -> String {
		let sp = " ".repeat(indent);
		let mut out = String::new();

		out.push_str(&format!(
			"{}<symbol kind=\"{}\" name=\"{}\" \
			file=\"{}\" line=\"{}\"",
			sp,
			&self.kind,
			escape_xml(&self.symbol),
			escape_xml(&self.file),
			self.line,
		));

		if let Some(ref sig) = self.signature {
			out.push_str(&format!(
				" signature=\"{}\"",
				escape_xml(sig),
			));
		}

		out.push_str(&format!(
			" lines=\"{}\" truncated=\"{}\" \
			score=\"{:.3}\">\n",
			self.line_count,
			self.truncated,
			self.relevance_score,
		));

		out.push_str(&format!(
			"{}  <content><![CDATA[\n", sp,
		));
		out.push_str(&self.full_content);
		out.push_str(&format!(
			"\n{}  ]]></content>\n", sp,
		));
		out.push_str(&format!("{}</symbol>\n", sp));

		out
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
	/// Convert to XML string with given indentation
	pub fn to_xml(&self, indent: usize) -> String {
		let sp = " ".repeat(indent);
		let mut out = String::new();

		out.push_str(&format!(
			"{}<doc file=\"{}\" section=\"{}\" \
			score=\"{:.3}\">\n",
			sp,
			escape_xml(&self.file),
			escape_xml(&self.section),
			self.relevance_score,
		));
		out.push_str(&format!(
			"{}  <content><![CDATA[\n", sp,
		));
		out.push_str(&self.content);
		out.push_str(&format!(
			"\n{}  ]]></content>\n", sp,
		));
		out.push_str(&format!("{}</doc>\n", sp));

		out
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
	/// Convert to XML string with given indentation
	pub fn to_xml(&self, indent: usize) -> String {
		let sp = " ".repeat(indent);
		let mut out = String::new();

		out.push_str(&format!(
			"{}<note file=\"{}\" section=\"{}\" \
			score=\"{:.3}\">\n",
			sp,
			escape_xml(&self.file),
			escape_xml(&self.section),
			self.relevance_score,
		));
		out.push_str(&format!(
			"{}  <content><![CDATA[\n", sp,
		));
		out.push_str(&self.content);
		out.push_str(&format!(
			"\n{}  ]]></content>\n", sp,
		));
		out.push_str(&format!("{}</note>\n", sp));

		out
	}
}

