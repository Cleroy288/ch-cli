//! Template-based doc generation for composite symbols.
//!
//! Generates instant documentation for simple Module,
//! Struct, Enum, and Impl kinds without LLM inference.

use crate::indexer::SymbolKind;
use crate::retrieval::docgen::entry::DocEntry;
use crate::retrieval::docgen::template_helpers::{
	extract_visibility, join_truncated,
};

/// Max children shown in composite template docs
const MAX_CHILDREN_SHOWN: usize = 5;

/// Generate template doc for composite symbols.
///
/// Dispatches to kind-specific template function.
/// Returns empty string for unsupported kinds.
pub fn generate_composite_doc(
	entry: &DocEntry,
) -> String {
	match entry.kind {
		SymbolKind::Module => template_module(entry),
		SymbolKind::Struct => template_struct(entry),
		SymbolKind::Enum => template_enum(entry),
		SymbolKind::Impl => template_impl(entry),
		_ => String::new(),
	}
}

/// Template for Module symbols.
///
/// Format: "Module `{name}` containing {children}."
fn template_module(entry: &DocEntry) -> String {
	let children = join_truncated(
		&entry.links.children,
		MAX_CHILDREN_SHOWN,
	);
	format!(
		"Module `{}` containing {}.",
		entry.name, children,
	)
}

/// Template for simple Struct symbols.
///
/// Format: "{vis} struct `{name}` with fields: {list}."
fn template_struct(entry: &DocEntry) -> String {
	let sig = entry.signature.as_deref().unwrap_or("");
	let vis = extract_visibility(sig);
	let vis_prefix =
		if vis.is_empty() { String::new() }
		else { format!("{} ", vis) };
	let fields = join_truncated(
		&entry.links.children,
		MAX_CHILDREN_SHOWN,
	);
	format!(
		"{}struct `{}` with fields: {}.",
		vis_prefix, entry.name, fields,
	)
}

/// Template for simple Enum symbols.
///
/// Format: "{vis} enum `{name}` with variants: {list}."
fn template_enum(entry: &DocEntry) -> String {
	let sig = entry.signature.as_deref().unwrap_or("");
	let vis = extract_visibility(sig);
	let vis_prefix =
		if vis.is_empty() { String::new() }
		else { format!("{} ", vis) };
	let variants = join_truncated(
		&entry.links.children,
		MAX_CHILDREN_SHOWN,
	);
	format!(
		"{}enum `{}` with variants: {}.",
		vis_prefix, entry.name, variants,
	)
}

/// Template for simple Impl symbols.
///
/// Format: "Implementation for `{name}` with N methods."
fn template_impl(entry: &DocEntry) -> String {
	let count = entry.links.children.len();
	let label = if count == 1 {
		"1 method".to_string()
	} else {
		format!("{} methods", count)
	};
	format!(
		"Implementation for `{}` with {}.",
		entry.name, label,
	)
}
