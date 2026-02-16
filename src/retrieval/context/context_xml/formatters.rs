//! XML Formatting Helpers for ContextualBlock
//!
//! Provides individual section formatters for XML output.

use super::core::escape_xml;
use super::super::ContextualBlock;

/// Format the symbol section as XML
pub(super) fn format_symbol_xml(block: &ContextualBlock) -> String {
	let mut xml = String::new();

	xml.push_str(&format!(
		"  <symbol kind=\"{}\" name=\"{}\" \
		file=\"{}\" line=\"{}\" usages=\"{}\">\n",
		block.symbol.kind,
		block.symbol.name,
		block.symbol.location.file.display(),
		block.symbol.location.line,
		block.usage_count
	));

	if let Some(ref doc) = block.doc_comment {
		xml.push_str(&format!(
			"    <doc>{}</doc>\n",
			escape_xml(doc),
		));
	}

	xml.push_str("    <code>\n");
	xml.push_str(&escape_xml(&block.code_snippet));
	xml.push_str("\n    </code>\n");
	xml.push_str("  </symbol>\n");

	xml
}

/// Format the parent section as XML
pub(super) fn format_parent_xml(
	block: &ContextualBlock,
) -> String {
	match block.parent {
		Some(ref parent) => format!(
			"  <parent kind=\"{}\" name=\"{}\" \
			file=\"{}\" line=\"{}\"/>\n",
			parent.kind,
			parent.name,
			parent.file.display(),
			parent.line,
		),
		None => String::new(),
	}
}

/// Format related types section as XML
pub(super) fn format_related_types_xml(
	block: &ContextualBlock,
) -> String {
	if block.related_types.is_empty() {
		return String::new();
	}

	let mut xml = String::from("  <related-types>\n");
	for rel_type in &block.related_types {
		xml.push_str(&format!(
			"    <type name=\"{}\" \
			relationship=\"{:?}\"/>\n",
			rel_type.name, rel_type.relationship,
		));
	}
	xml.push_str("  </related-types>\n");

	xml
}

/// Format callers section as XML
pub(super) fn format_callers_xml(
	block: &ContextualBlock,
) -> String {
	if block.callers.is_empty() {
		return String::new();
	}

	let mut xml = String::from("  <callers>\n");
	for caller in &block.callers {
		xml.push_str(&format!(
			"    <caller name=\"{}\" kind=\"{}\" \
			file=\"{}\" line=\"{}\"/>\n",
			caller.name,
			caller.kind,
			caller.file.display(),
			caller.line,
		));
	}
	xml.push_str("  </callers>\n");

	xml
}

/// Format callees section as XML
pub(super) fn format_callees_xml(
	block: &ContextualBlock,
) -> String {
	if block.callees.is_empty() {
		return String::new();
	}

	let mut xml = String::from("  <callees>\n");
	for callee in &block.callees {
		xml.push_str(&format!(
			"    <callee name=\"{}\"/>\n",
			callee.name,
		));
	}
	xml.push_str("  </callees>\n");

	xml
}
