//! Linker analysis - reference conversion and context.
//!
//! Core conversion and utility functions.

use std::fs;
use std::path::Path;

use crate::indexer::semantic::ReferenceContext;
use crate::retrieval::docgen::entry_types::ReferenceKind;

/// Number of lines to include as context around a reference.
const CONTEXT_LINES: usize = 3;

/// Convert SemanticGraph ReferenceContext to ReferenceKind.
#[doc(hidden)]
pub fn convert_reference_context(
	ctx: ReferenceContext,
) -> ReferenceKind {
	match ctx {
		ReferenceContext::Call => ReferenceKind::Call,
		ReferenceContext::Type => ReferenceKind::TypeUsage,
		ReferenceContext::FieldAccess => {
			ReferenceKind::FieldAccess
		}
		ReferenceContext::Import => ReferenceKind::Import,
		ReferenceContext::Identifier => {
			ReferenceKind::TypeUsage
		}
		ReferenceContext::Unknown => ReferenceKind::Call,
		ReferenceContext::FieldType => {
			ReferenceKind::TypeUsage
		}
		ReferenceContext::ReturnType => {
			ReferenceKind::TypeUsage
		}
		ReferenceContext::ParameterType => {
			ReferenceKind::TypeUsage
		}
		ReferenceContext::GenericArg => {
			ReferenceKind::TypeUsage
		}
		ReferenceContext::TraitBound => {
			ReferenceKind::TypeUsage
		}
		ReferenceContext::ImplTarget => {
			ReferenceKind::TypeUsage
		}
	}
}

/// Extract surrounding lines from a file for context.
#[doc(hidden)]
pub fn extract_context_lines(
	file_path: &Path,
	line: usize,
) -> String {
	let content = match fs::read_to_string(file_path) {
		Ok(c) => c,
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

