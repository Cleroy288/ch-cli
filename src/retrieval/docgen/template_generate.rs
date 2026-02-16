//! Template-based doc generation for simple symbols.
//!
//! Generates instant documentation strings for
//! Constant, Static, TypeAlias, EnumVariant, and Field
//! kinds without requiring LLM inference.

use crate::indexer::SymbolKind;
use crate::retrieval::docgen::entry::DocEntry;
use crate::retrieval::docgen::template_helpers::{
	extract_type_from_sig, extract_visibility,
	format_parent_ctx,
};

/// Generate template documentation for simple symbols.
///
/// Dispatches to kind-specific template functions.
/// Returns empty string for unsupported kinds.
pub fn generate_template_doc(
	entry: &DocEntry,
) -> String {
	match entry.kind {
		SymbolKind::Constant | SymbolKind::Static => {
			template_const_static(entry)
		}
		SymbolKind::TypeAlias => {
			template_type_alias(entry)
		}
		SymbolKind::EnumVariant => {
			template_variant(entry)
		}
		SymbolKind::Field => template_field(entry),
		_ => String::new(),
	}
}

/// Template for Constant and Static symbols.
///
/// Format: "{vis} {kind} `{name}` of type `{type}`."
fn template_const_static(entry: &DocEntry) -> String {
	let kind_label = entry.kind;
	let sig = entry.signature.as_deref().unwrap_or("");
	let vis = extract_visibility(sig);
	let type_str = extract_type_from_sig(sig);
	let vis_prefix =
		if vis.is_empty() { String::new() }
		else { format!("{} ", vis) };
	format!(
		"{}{} `{}` of type `{}`.",
		vis_prefix, kind_label, entry.name, type_str,
	)
}

/// Template for TypeAlias symbols.
///
/// Format: "Type alias `{name}` for `{aliased_type}`."
fn template_type_alias(entry: &DocEntry) -> String {
	let sig = entry.signature.as_deref().unwrap_or("");
	let aliased = extract_type_from_sig(sig);
	format!(
		"Type alias `{}` for `{}`.",
		entry.name, aliased,
	)
}

/// Template for EnumVariant symbols.
///
/// Format: "Variant `{name}` of enum `{parent}`."
fn template_variant(entry: &DocEntry) -> String {
	let ctx = format_parent_ctx(
		entry.links.parent.as_deref(),
		&entry.file_path,
	);
	format!("Variant `{}`{}.", entry.name, ctx)
}

/// Template for Field symbols.
///
/// Format: "Field `{name}: {type}` on `{parent}`."
fn template_field(entry: &DocEntry) -> String {
	let sig = entry.signature.as_deref().unwrap_or("");
	let type_str = extract_type_from_sig(sig);
	let ctx = format_parent_ctx(
		entry.links.parent.as_deref(),
		&entry.file_path,
	);
	format!(
		"Field `{}: {}`{}.",
		entry.name, type_str, ctx,
	)
}
