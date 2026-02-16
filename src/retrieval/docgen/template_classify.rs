//! Classification logic for doc generation strategy.
//!
//! Determines whether a DocEntry should use a user
//! comment, a template, or LLM generation.

use crate::indexer::SymbolKind;
use crate::retrieval::docgen::entry::DocEntry;

/// Documentation generation strategy for an entry.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DocStrategy {
	/// Use existing user comment as-is
	UserComment,
	/// Generate from template (instant, no GPU)
	Template,
	/// Requires LLM inference (~2s each)
	Llm,
	/// Skip generation (already readable)
	Skip,
}

/// Classify a doc entry into a generation strategy.
///
/// Priority: UserComment > Skip > Template > Llm.
pub fn classify_entry(entry: &DocEntry) -> DocStrategy {
	if has_usable_comment(entry) {
		return DocStrategy::UserComment;
	}
	if entry.kind == SymbolKind::DocumentChunk {
		return DocStrategy::Skip;
	}
	if is_template_kind(entry) {
		return DocStrategy::Template;
	}
	DocStrategy::Llm
}

/// Check if entry has a non-empty user comment.
fn has_usable_comment(entry: &DocEntry) -> bool {
	entry
		.user_comment
		.as_ref()
		.is_some_and(|txt| !txt.trim().is_empty())
}

/// Maximum children for "simple" composite
const MAX_SIMPLE_CHILDREN: usize = 2;

/// Maximum dependencies for "simple" composite
const MAX_SIMPLE_DEPS: usize = 3;

/// Check if a symbol kind can use template generation.
///
/// Simple kinds always qualify. Composite kinds
/// (Struct, Enum, Impl, Module) qualify only if
/// they have few children and dependencies.
fn is_template_kind(entry: &DocEntry) -> bool {
	match entry.kind {
		SymbolKind::Constant
		| SymbolKind::Static
		| SymbolKind::TypeAlias
		| SymbolKind::EnumVariant
		| SymbolKind::Field => true,
		SymbolKind::Module
		| SymbolKind::Struct
		| SymbolKind::Enum
		| SymbolKind::Impl => {
			!is_complex_composite(entry)
		}
		_ => false,
	}
}

/// Check if a composite entry is too complex for
/// template generation.
fn is_complex_composite(entry: &DocEntry) -> bool {
	let many_children =
		entry.links.children.len() > MAX_SIMPLE_CHILDREN;
	let many_deps =
		entry.links.depends_on.len() > MAX_SIMPLE_DEPS;
	many_children || many_deps
}
