//! Linker analysis - reference conversion and context.
//!
//! Core conversion and utility functions.

use std::path::Path;

use crate::indexer::semantic::ReferenceContext;
use crate::retrieval::docgen::entry_types::ReferenceKind;

/// Number of lines to include as context.
const CONTEXT_LINES: usize = 3;

/// Convert SemanticGraph ReferenceContext to ReferenceKind.
#[doc(hidden)]
pub fn convert_reference_context(
	ctx: ReferenceContext,
) -> ReferenceKind {
	convert_call_types(ctx)
		.unwrap_or_else(|| convert_type_usages(ctx))
}

/// Convert call-related reference contexts.
fn convert_call_types(
	ctx: ReferenceContext,
) -> Option<ReferenceKind> {
	match ctx {
		ReferenceContext::Call => {
			Some(ReferenceKind::Call)
		}
		ReferenceContext::Import => {
			Some(ReferenceKind::Import)
		}
		ReferenceContext::FieldAccess => {
			Some(ReferenceKind::FieldAccess)
		}
		ReferenceContext::Unknown => {
			Some(ReferenceKind::Call)
		}
		_ => None,
	}
}

/// Convert type-usage reference contexts.
fn convert_type_usages(
	ctx: ReferenceContext,
) -> ReferenceKind {
	match ctx {
		ReferenceContext::Type
		| ReferenceContext::Identifier
		| ReferenceContext::FieldType
		| ReferenceContext::ReturnType
		| ReferenceContext::ParameterType
		| ReferenceContext::GenericArg
		| ReferenceContext::TraitBound
		| ReferenceContext::ImplTarget => {
			ReferenceKind::TypeUsage
		}
		_ => ReferenceKind::Call,
	}
}

/// Extract surrounding lines from a file for context.
#[doc(hidden)]
pub fn extract_context_lines(
	file_path: &Path,
	line: usize,
) -> String {
	let content =
		match std::fs::read_to_string(file_path) {
			Ok(text) => text,
			Err(_) => return String::new(),
		};

	let lines: Vec<&str> = content.lines().collect();

	if line == 0 || line > lines.len() {
		return String::new();
	}

	let line_idx = line - 1;
	let start = line_idx.saturating_sub(CONTEXT_LINES);
	let end =
		(line_idx + CONTEXT_LINES + 1).min(lines.len());

	lines[start..end].join("\n")
}

/// Check if a module name is from the standard library.
#[doc(hidden)]
pub fn is_std_module(name: &str) -> bool {
	matches!(
		name,
		"std" | "core" | "alloc"
			| "self" | "super"
			| "crate"
	)
}
