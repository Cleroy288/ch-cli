//! Cross-reference linker for documentation entries.
//!
//! Builds comprehensive symbol graphs with dependencies,
//! dependents, and external crate tracking.

use std::collections::HashSet;
use std::fs;
use std::path::Path;

use regex::Regex;

use crate::indexer::semantic::{ReferenceContext, SemanticGraph};
use crate::indexer::Symbol;
use crate::retrieval::docgen::entry::{ReferenceKind, ReferenceLocation};
use crate::retrieval::docgen::DocStore;

/// Number of lines to include as context around a reference.
const CONTEXT_LINES: usize = 3;

/// Builds cross-reference links between documentation entries.
pub struct DocLinker {
	/// regex for extracting external crate names from use statements
	use_regex: Regex,
	/// regex for extracting crate names from extern crate
	extern_regex: Regex,
}

impl DocLinker {
	/// Create a new linker.
	pub fn new() -> Self {
		Self {
			use_regex: Regex::new(r"use\s+([a-z_][a-z0-9_]*)::").unwrap(),
			extern_regex: Regex::new(r"extern\s+crate\s+([a-z_][a-z0-9_]*)").unwrap(),
		}
	}

	/// Build all cross-references for entries in the store.
	pub fn build_links(
		&self,
		store: &mut DocStore,
		graph: &SemanticGraph,
		symbols: &[Symbol],
	) {
		// build a map of symbol name -> entry ID for quick lookup
		let mut name_to_id: std::collections::HashMap<String, Vec<String>> =
			std::collections::HashMap::new();

		for entry in store.all_entries() {
			name_to_id
				.entry(entry.name.clone())
				.or_default()
				.push(entry.id.clone());
		}

		// collect all entry IDs first to avoid borrow issues
		let entry_ids: Vec<String> = store.all_entries().map(|e| e.id.clone()).collect();

		// process each entry
		for id in entry_ids {
			let entry_name = {
				let entry = store.get(&id).unwrap();
				entry.name.clone()
			};

			// find all references TO this symbol
			let refs = graph.find_references(&entry_name);
			let mut reference_locations = Vec::new();
			let mut depended_by = HashSet::new();

			for sym_ref in refs {
				// convert to our ReferenceLocation format
				let ref_kind = convert_reference_context(sym_ref.context);
				let context = extract_context_lines(&sym_ref.location.file, sym_ref.location.line);
				let module_path = get_module_path(&sym_ref.location.file);
				let containing = find_containing_symbol(symbols, &sym_ref.location);

				let ref_loc = ReferenceLocation::new(
					sym_ref.location.file.clone(),
					sym_ref.location.line,
					ref_kind,
				)
				.with_context(context)
				.with_module_path(module_path);

				let ref_loc = if let Some(ref container) = containing {
					depended_by.insert(container.clone());
					ref_loc.with_containing_symbol(container.clone())
				} else {
					ref_loc
				};

				reference_locations.push(ref_loc);
			}

			// find what this symbol depends on (calls/uses)
			let depends_on = find_dependencies(graph, symbols, &entry_name);

			// find external crate dependencies
			let external_deps = {
				let entry = store.get(&id).unwrap();
				extract_external_deps(&entry.code_snippet)
			};

			// find parent symbol
			let parent = symbols
				.iter()
				.find(|s| s.name == entry_name)
				.and_then(|s| s.parent.clone());

			// find children (for modules, structs with methods)
			let children = find_children(symbols, &entry_name);

			// update the entry
			if let Some(entry) = store.get_mut(&id) {
				entry.references = reference_locations;
				entry.links.depended_by = depended_by.into_iter().collect();
				entry.links.depends_on = depends_on;
				entry.links.external_deps = external_deps;
				entry.links.parent = parent;
				entry.links.children = children;
			}
		}
	}

	/// Extract external crate names from code.
	pub fn extract_crates(&self, code: &str) -> Vec<String> {
		let mut crates = HashSet::new();

		// find "use crate_name::"
		for cap in self.use_regex.captures_iter(code) {
			if let Some(m) = cap.get(1) {
				let name = m.as_str();
				// filter out std lib and common internal modules
				if !is_std_module(name) {
					crates.insert(name.to_string());
				}
			}
		}

		// find "extern crate name"
		for cap in self.extern_regex.captures_iter(code) {
			if let Some(m) = cap.get(1) {
				crates.insert(m.as_str().to_string());
			}
		}

		crates.into_iter().collect()
	}
}

impl Default for DocLinker {
	fn default() -> Self {
		Self::new()
	}
}

/// Convert SemanticGraph ReferenceContext to our ReferenceKind.
fn convert_reference_context(ctx: ReferenceContext) -> ReferenceKind {
	match ctx {
		ReferenceContext::Call => ReferenceKind::Call,
		ReferenceContext::Type => ReferenceKind::TypeUsage,
		ReferenceContext::FieldAccess => ReferenceKind::FieldAccess,
		ReferenceContext::Import => ReferenceKind::Import,
		ReferenceContext::Identifier => ReferenceKind::TypeUsage,
		ReferenceContext::Unknown => ReferenceKind::Call,
		// New type-related contexts map to TypeUsage
		ReferenceContext::FieldType => ReferenceKind::TypeUsage,
		ReferenceContext::ReturnType => ReferenceKind::TypeUsage,
		ReferenceContext::ParameterType => ReferenceKind::TypeUsage,
		ReferenceContext::GenericArg => ReferenceKind::TypeUsage,
		ReferenceContext::TraitBound => ReferenceKind::TypeUsage,
		ReferenceContext::ImplTarget => ReferenceKind::TypeUsage,
	}
}

/// Extract surrounding lines from a file for context.
fn extract_context_lines(file_path: &Path, line: usize) -> String {
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
	let end = (line_idx + CONTEXT_LINES + 1).min(lines.len());

	lines[start..end].join("\n")
}

/// Get module path from file path (e.g., "src/retrieval/docgen/entry.rs" -> "retrieval::docgen::entry").
fn get_module_path(file_path: &Path) -> String {
	let path_str = file_path.to_string_lossy().to_string();

	// normalize path separators
	let path_str = path_str.replace('\\', "/");

	// remove src/ prefix and .rs suffix
	let path_str = if let Some(stripped) = path_str.strip_prefix("src/") {
		stripped.to_string()
	} else {
		path_str
	};

	let path_str = if let Some(stripped) = path_str.strip_suffix(".rs") {
		stripped.to_string()
	} else {
		path_str
	};

	let path_str = if let Some(stripped) = path_str.strip_suffix("/mod") {
		stripped.to_string()
	} else {
		path_str
	};

	// convert / to ::
	path_str.replace('/', "::")
}

/// Find the symbol that contains a given location.
fn find_containing_symbol(symbols: &[Symbol], location: &crate::indexer::CodeLocation) -> Option<String> {
	// find functions/methods in the same file that contain this line
	symbols
		.iter()
		.filter(|s| {
			s.location.file == location.file
				&& s.location.line <= location.line
				&& matches!(
					s.kind,
					crate::indexer::SymbolKind::Function
						| crate::indexer::SymbolKind::Method
						| crate::indexer::SymbolKind::Impl
				)
		})
		.max_by_key(|s| s.location.line)
		.map(|s| s.name.clone())
}

/// Find symbols that this symbol depends on.
fn find_dependencies(
	graph: &SemanticGraph,
	symbols: &[Symbol],
	symbol_name: &str,
) -> Vec<String> {
	// find the symbol's file and approximate range
	let symbol = match symbols.iter().find(|s| s.name == symbol_name) {
		Some(s) => s,
		None => return Vec::new(),
	};

	// get all references in the same file
	let refs = graph.references_in_file(&symbol.location.file);

	// filter to references within the symbol's body (approximate)
	let mut deps = HashSet::new();
	for ref_loc in refs {
		// heuristic: references after the symbol definition and within ~100 lines
		if ref_loc.location.line > symbol.location.line
			&& ref_loc.location.line < symbol.location.line + 100
		{
			// check if there's a definition for this reference
			if !graph.find_definitions(&ref_loc.name).is_empty() {
				deps.insert(ref_loc.name.clone());
			}
		}
	}

	deps.into_iter().collect()
}

/// Find child symbols (methods for impl, variants for enum, etc.).
fn find_children(symbols: &[Symbol], parent_name: &str) -> Vec<String> {
	symbols
		.iter()
		.filter(|s| s.parent.as_ref().map(|p| p == parent_name).unwrap_or(false))
		.map(|s| s.name.clone())
		.collect()
}

/// Extract external crate dependencies from code snippet.
fn extract_external_deps(code: &str) -> Vec<String> {
	let linker = DocLinker::new();
	linker.extract_crates(code)
}

/// Check if a module name is from the standard library.
fn is_std_module(name: &str) -> bool {
	matches!(
		name,
		"std" | "core" | "alloc" | "self" | "super" | "crate"
	)
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test_get_module_path() {
		assert_eq!(
			get_module_path(Path::new("src/retrieval/docgen/entry.rs")),
			"retrieval::docgen::entry"
		);
		assert_eq!(
			get_module_path(Path::new("src/main.rs")),
			"main"
		);
		assert_eq!(
			get_module_path(Path::new("src/lib.rs")),
			"lib"
		);
	}

	#[test]
	fn test_extract_crates() {
		let linker = DocLinker::new();

		let code = r#"
use serde::{Serialize, Deserialize};
use std::path::Path;
use tokio::runtime;
extern crate regex;
"#;

		let crates = linker.extract_crates(code);
		assert!(crates.contains(&"serde".to_string()));
		assert!(crates.contains(&"tokio".to_string()));
		assert!(crates.contains(&"regex".to_string()));
		assert!(!crates.contains(&"std".to_string())); // filtered out
	}

	#[test]
	fn test_is_std_module() {
		assert!(is_std_module("std"));
		assert!(is_std_module("core"));
		assert!(is_std_module("self"));
		assert!(!is_std_module("serde"));
		assert!(!is_std_module("tokio"));
	}

	#[test]
	fn test_convert_reference_context() {
		assert!(matches!(
			convert_reference_context(ReferenceContext::Call),
			ReferenceKind::Call
		));
		assert!(matches!(
			convert_reference_context(ReferenceContext::Type),
			ReferenceKind::TypeUsage
		));
		assert!(matches!(
			convert_reference_context(ReferenceContext::Import),
			ReferenceKind::Import
		));
	}
}
